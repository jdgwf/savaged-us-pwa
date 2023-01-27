use crate::components::ui_page::UIPage;
use crate::libs::fetch_api::savaged_login;
use crate::libs::global_vars::GlobalVars;
use crate::local_storage::clear_all_local_data;
use crate::main_app::MainRoute;
use gloo_console::error;
use gloo_console::log;
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::user::LoginTokenResult;
use savaged_libs::websocket_message::WebSocketMessage;
use savaged_libs::websocket_message::WebsocketMessageType;
use serde_json::Error;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::input_text::InputText;
use standard_components::ui::nbsp::Nbsp;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserLoginProps {
    pub global_vars: GlobalVars,
}

pub enum UserLoginMessage {
    UpdatePassword(String),
    UpdateUsername(String),
    UpdateCurrentUser(LoginTokenResult),
    UpdateLoginMessage(String),
}

pub struct UserLogin {
    username: String,
    password: String,

    login_message: String,
}

impl Component for UserLogin {
    type Message = UserLoginMessage;
    type Properties = UserLoginProps;

    fn create(ctx: &Context<Self>) -> Self {
        let global_vars = ctx.props().global_vars.clone();

        set_document_title(
            global_vars.site_title.to_owned(),
            "Login".to_owned(),
            global_vars.server_side_renderer,
        );
        UserLogin {
            username: "".to_owned(),
            password: "".to_owned(),
            login_message: "".to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: UserLoginMessage) -> bool {
        match msg {
            UserLoginMessage::UpdateCurrentUser(login_result) => {
                // log!("UserLoginMessage::UpdateCurrentUser", login_result.success);
                let mut global_vars = ctx.props().global_vars.clone();
                global_vars.current_user = login_result.user.clone();
                global_vars.login_token = login_result.login_token.clone();
                global_vars.user_loading = false;

                ctx.props().global_vars.update_global_vars.emit(global_vars);

                // clear out local data
                spawn_local(async move {
                    clear_all_local_data().await;
                });

                // request user and game data data

                let mut msg = WebSocketMessage::default();

                msg.token = Some(login_result.login_token.to_owned());
                msg.kind = WebsocketMessageType::GameDataPackage;

                ctx.props().global_vars.send_websocket.emit(msg);

                let mut msg_saves = WebSocketMessage::default();

                msg_saves.token = Some(login_result.login_token.to_owned());
                msg_saves.kind = WebsocketMessageType::Saves;

                ctx.props().global_vars.send_websocket.emit(msg_saves);

                // set_local_storage_string( "saves_owner_id", login_result.user.id.to_string() );
                set_local_storage_string("login_token", login_result.login_token.to_owned());

                return true;
            }

            UserLoginMessage::UpdateLoginMessage(new_value) => {
                self.login_message = new_value.to_owned();

                return true;
            }

            UserLoginMessage::UpdateUsername(new_value) => {
                self.username = new_value.to_owned();

                return true;
            }

            UserLoginMessage::UpdatePassword(new_value) => {
                self.password = new_value.to_owned();

                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let global_vars = ctx.props().global_vars.clone();
        let global_vars_event = ctx.props().global_vars.clone();

        let update_username = ctx.link().callback(UserLoginMessage::UpdateUsername);
        let update_password = ctx.link().callback(UserLoginMessage::UpdatePassword);
        let update_current_user_from_login =
            ctx.link().callback(UserLoginMessage::UpdateCurrentUser);
        let set_login_message = ctx.link().callback(UserLoginMessage::UpdateLoginMessage);

        let username = self.username.to_owned();
        let password = self.password.to_owned();
        let api_root = ctx.props().global_vars.api_root.to_owned();

        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();

        let do_login_submit = move |e: SubmitEvent| {
            // log!("trying do_login_submit");
            e.prevent_default();
            let global_vars = global_vars_event.clone();
            let update_global_vars = update_global_vars.clone();
            let username = username.to_owned();
            let password = password.to_owned();
            let api_root = api_root.to_owned();
            let update_current_user_from_login = update_current_user_from_login.clone();
            let set_login_message = set_login_message.clone();

            spawn_local(async move {
                // log!(
                //     "trying login-for-token",
                //     api_root.clone() + "/auth/login-for-token"
                // );
                let result = savaged_login(
                    (api_root + "/auth/login-for-token").to_owned(),
                    username,
                    password,
                )
                .await;

                match result {
                    Ok(value) => {
                        // let mut global_vars = global_vars.clone();
                        // global_vars.offline = false;
                        // global_vars.update_global_vars.emit( global_vars.clone() );

                        let vec_val_result: Result<LoginTokenResult, Error> =
                            JsValueSerdeExt::into_serde(&value);
                        match vec_val_result {
                            Ok(vec_val) => {
                                if !vec_val.success {
                                    set_login_message.emit("Invalid Login".to_owned());
                                } else {
                                    set_login_message.emit("".to_owned());
                                    update_current_user_from_login.emit(vec_val.clone());
                                }
                            }
                            Err(err) => {
                                let err_string: String =
                                    format!("savaged_login Serde Err(): {}", &err);
                                set_login_message.emit("Invalid Login".to_owned());
                                error!("login err_string", &err_string);
                            }
                        }
                    }
                    Err(err) => {
                        error!("savaged_login Err()", &err);
                        set_login_message.emit("Can't connect to server".to_owned());
                        let mut global_vars = global_vars.clone();
                        global_vars.offline = true;
                        update_global_vars.emit(global_vars.clone());
                    }
                }
            });
        };
        let mut global_vars = global_vars.clone();
        global_vars.current_menu = "main-user-login".to_owned();

        if global_vars.user_loading {
            html!(
                    <UIPage
                        global_vars={global_vars.clone()}
                        page_title="Login"

                    >
                        <p class={"text-center"}>{"loading user info...."}</p>
                    </UIPage>
            )
        } else {
            html! {
            <UIPage
                global_vars={global_vars.clone()}
                page_title="Login"

            >
                <div class={"main-content"}>
                    <h1>{ "Savaged.us Login" }</h1>
                    // {"ID: "}{global_vars.current_user.id}<br />
                    if global_vars.current_user.id > 0 {
                        <fieldset class={"fieldset"}>
                            <legend>{"Current User"}</legend>
                            <div class="user-profile">

                                <img src={global_vars.current_user.get_image(&global_vars.server_root)} />

                            <h3>{&global_vars.current_user.username}</h3>
                            <strong>{"ID: "}</strong>{&global_vars.current_user.id}<br />
                            <strong>{"Unread Notifications: "}</strong>{&global_vars.current_user.unread_notifications}<br />
                            <strong>{"Display Name: "}</strong>{&global_vars.current_user.get_name()}<br />
                            <strong>{"Twitter: "}</strong>{&global_vars.current_user.twitter}<br />
                            </div>

                        </fieldset>
                    } else {
                        <div class="row equal-heights">
                            <div class="col-xs-12 col-md-6">
                                <fieldset class={"fieldset"}>
                                    <h3>{"Register"}</h3>

                                {"Don't have an account?"}<Nbsp />
                                <Link<MainRoute> to={MainRoute::Register} classes={"btn btn-xs"}>
                                    {"Click Here"}
                                </Link<MainRoute>><Nbsp />
                                {"to register"}

                                <h3>{"Forgot your password?"}</h3>

                                {"Did you forget your password?"}<Nbsp />
                                <Link<MainRoute> to={MainRoute::ForgotPassword} classes={"btn btn-xs"}>
                                    {"Click Here"}
                                </Link<MainRoute>><Nbsp />
                                {"to begin to recover your account."}

                                </fieldset>
                            </div>
                            <div class="col-xs-12 col-md-6">
                                <fieldset class={"fieldset"}>
                                    <h3>{"Login"}</h3>
                                    <form onsubmit={do_login_submit}>
                                        <InputText
                                            label={"Email Address"}
                                            inline={true}
                                            input_type={"text"}
                                            placeholder={"me@example.com"}
                                            value={self.username.clone()}
                                            onchange={update_username}
                                            title={"Email Address"}
                                        />
                                        <InputText
                                            label={"Password"}
                                            inline={true}
                                            input_type={"password"}
                                            placeholder={"Password"}
                                            value={self.password.clone()}
                                            onchange={update_password}
                                            title={"Password"}
                                        />

                                    if !self.login_message.is_empty() {
                                        <div class="alert alert-warning">
                                            {&self.login_message}
                                        </div>
                                    }
                                    <button
                                        type="submit"
                                        class={"btn btn-primary"}
                                        disabled={self.username.is_empty() || self.password.is_empty()}
                                    >
                                        {"Perform Login"}
                                    </button>
                                    </form>
                                </fieldset>
                            </div>
                        </div>
                    }

                </div>

                </UIPage>
            }
        }
    }

    // fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    // fn destroy(&mut self, ctx: &Context<Self>) {}
}

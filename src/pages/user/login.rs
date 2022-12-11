use yew::prelude::*;
use yew_router::prelude::*;

use standard_components::ui::input_text::InputText;
use standard_components::ui::nbsp::Nbsp;

use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::set_document_title::set_document_title;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use serde_json::Error;
use crate::libs::fetch_api::savaged_login;
use gloo_console::error;
use wasm_bindgen_futures::spawn_local;
use gloo_utils::format::JsValueSerdeExt;
use crate::libs::global_vars::GlobalVars;

use savaged_libs::user::User;
use savaged_libs::user::LoginTokenResult;
use gloo_console::log;
use crate::main_app::MainRoute;

#[derive(Properties, PartialEq)]
pub struct UserLoginProps {
    pub global_vars: GlobalVars,
    pub update_global_vars: Callback<GlobalVars>,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum UserLoginMessage {
    UpdatePassword(String),
    UpdateUsername(String),
    UpdateCurrentUser( LoginTokenResult ),
    LogOut( String ),
    UpdateLoginMessage( String ),
}

pub struct UserLogin {
    global_vars: GlobalVars,

    username: String,
    password: String,

    login_message: String,
}

impl Component for UserLogin {
    type Message = UserLoginMessage;
    type Properties = UserLoginProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Login".to_owned(), global_vars.no_calls,);
        UserLogin {
            global_vars: global_vars,
            username: "".to_owned(),
            password: "".to_owned(),
            login_message: "".to_owned(),
        }
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserLoginProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();
        true
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: UserLoginMessage,
    ) -> bool {

        match msg {

            UserLoginMessage::UpdateCurrentUser( login_result ) => {
                log!("UserLoginMessage::UpdateCurrentUser", login_result.success);
                self.global_vars.current_user = login_result.user.clone();
                self.global_vars.login_token = login_result.login_token.clone();
                self.global_vars.user_loading = false;
                set_local_storage_string( "login_token", login_result.login_token.to_owned() );
                ctx.props().update_global_vars.emit( self.global_vars.clone() );
                return true;
            }

            UserLoginMessage::LogOut( _new_value ) => {

                // log!("LogOut?");
                self.global_vars.current_user = User::default();

                // self.global_vars.user_loading = false;
                self.global_vars.login_token = "".to_owned();
                set_local_storage_string( "login_token", "".to_owned() );

                ctx.props().update_global_vars.emit( self.global_vars.clone() );
                return true;
            }

            UserLoginMessage::UpdateLoginMessage( new_value ) => {

                self.login_message = new_value.to_owned();

                return true;
            }

            UserLoginMessage::UpdateUsername( new_value ) => {

                self.username = new_value.to_owned();

                return true;
            }

            UserLoginMessage::UpdatePassword( new_value ) => {

                self.password = new_value.to_owned();

                return true;
            }
        }

    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        // let global_vars = self.global_vars.clone();
        let global_vars = ctx.props().global_vars.clone();
        let global_vars_event = ctx.props().global_vars.clone();

        let update_username = ctx.link().callback(UserLoginMessage::UpdateUsername);
        let update_password = ctx.link().callback(UserLoginMessage::UpdatePassword);
        let update_current_user_from_login = ctx.link().callback(UserLoginMessage::UpdateCurrentUser);
        let set_login_message =  ctx.link().callback(UserLoginMessage::UpdateLoginMessage);

        let log_out = ctx.link().callback(UserLoginMessage::LogOut);

        let do_log_out = Callback::from( move | _e: MouseEvent | {
            log_out.emit( "".to_owned() );
        });

        let username = self.username.to_owned();
        let password = self.password.to_owned();
        let api_root = ctx.props().global_vars.api_root.to_owned();

        let do_login_submit = move |e: SubmitEvent | {
            // log!("trying do_login_submit");
            e.prevent_default();
            let global_vars = global_vars_event.clone();
            let username = username.to_owned();
            let password = password.to_owned();
            let api_root = api_root.to_owned();
            let update_current_user_from_login = update_current_user_from_login.clone();
            let set_login_message = set_login_message.clone();
            spawn_local (
                async move {
                    log!("trying login-for-token", api_root.clone() + "/auth/login-for-token");
                    let result = savaged_login(
                        (api_root + "/auth/login-for-token").to_owned(),
                        username,
                        password,
                    ).await;

                    match result {
                        Ok( value ) => {

                            // let mut global_vars = global_vars.clone();
                            // global_vars.offline = false;
                            // global_vars.update_global_vars.emit( global_vars.clone() );

                            let vec_val_result: Result<LoginTokenResult, Error> = JsValueSerdeExt::into_serde(&value);
                            match vec_val_result {
                                Ok( vec_val ) => {

                                    if !vec_val.success {
                                        set_login_message.emit( "Invalid Login".to_owned() );
                                    } else {
                                        set_login_message.emit( "".to_owned() );
                                        update_current_user_from_login.emit( vec_val.clone() );
                                    }


                                }
                                Err( err ) => {
                                    let err_string: String = format!("savaged_login Serde Err(): {}", &err);
                                    set_login_message.emit( "Invalid Login".to_owned() );
                                    error!("login err_string", &err_string  );
                                }
                            }

                        }
                        Err( err ) => {
                            error!("savaged_login Err()", &err );
                            set_login_message.emit( "Can't connect to server".to_owned() );
                            let mut global_vars = global_vars.clone();
                            global_vars.offline = true;
                            global_vars.update_global_vars.emit( global_vars.clone() );
                        }
                    }
                }
            );
        };
        let global_vars = global_vars.clone();

        if global_vars.user_loading {
            html!(<p class={"text-center"}>{"loading user info...."}</p>)
        } else {


            html! {
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
                            <button
                                class={"btn btn-primary"}
                                onclick={do_log_out}
                            >
                                <i class={"fa-solid fa-right-from-bracket"}></i><Nbsp />{"Log Out"}
                            </button>
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

            }
        }
    }

    // fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    // fn destroy(&mut self, ctx: &Context<Self>) {}
}

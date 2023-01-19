use yew::prelude::*;
use crate::libs::fetch_api::fetch_api;
// use standard_components::ui::input_text::InputText;
use standard_components::ui::nbsp::Nbsp;
use wasm_bindgen_futures::spawn_local;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::set_document_title::set_document_title;
use savaged_libs::user::User;
// use crate::lib::fetch_api::fetch_api;
// use crate::lib::fetch_api::savaged_login;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use serde_json::Error;
// use gloo_console::log;
use gloo_console::error;
// use web_sys::console;
// use wasm_bindgen_futures::spawn_local;
// use gloo_utils::format::JsValueSerdeExt;
use crate::libs::global_vars::GlobalVars;
use gloo_utils::format::JsValueSerdeExt;
use crate::components::ui_page::UIPage;

// use savaged_libs::user::User;

#[derive(Properties, PartialEq)]
pub struct SettingsAPIKeyProps {
    pub global_vars: GlobalVars,
}

pub enum SettingsAPIKeyMessage {
    ConfirmYes( bool ),
    RegenerateAPIKey( MouseEvent ),
}

pub struct SettingsAPIKey {

}

impl Component for SettingsAPIKey {
    type Message = SettingsAPIKeyMessage;
    type Properties = SettingsAPIKeyProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "API Key".to_owned(), global_vars.server_side_renderer,);
        SettingsAPIKey {
        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: SettingsAPIKeyMessage,
    ) -> bool {

        match msg {

            SettingsAPIKeyMessage::ConfirmYes( bool ) => {
                if bool {

                    let mut global_vars = ctx.props().global_vars.clone();
                    let login_token = global_vars.login_token.to_owned();
                    let api_root = global_vars.api_root.to_owned();
                    let update_global_vars = ctx.props().global_vars.update_global_vars.clone();

                    spawn_local (
                        async move {
                            let result = fetch_api(
                                api_root.to_owned() + &"/auth/new-api-key".to_owned(),
                                "".to_owned(),
                                login_token.to_owned(),
                            ).await;

                            match result {
                                Ok( _result_data ) => {

                                    let user_result = fetch_api(
                                        api_root.to_owned() + &"/auth/get-user-data".to_owned(),
                                        "".to_owned(),
                                        login_token.to_owned(),
                                    ).await;
                                    match user_result {
                                        Ok( user_value ) => {
                                            // let vec_val_result = user_value.into_serde::<User>();
                                            let vec_val_result: Result<User, Error> = JsValueSerdeExt::into_serde(&user_value);
                                            match vec_val_result {
                                                Ok( vec_val ) => {
                                                    // update_current_user.emit( vec_val.clone() );
                                                    global_vars.current_user = vec_val.clone();
                                                    update_global_vars.emit( global_vars );
                                                }
                                                Err( err ) => {
                                                    let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                                                    // update_current_user.emit( User::default() );
                                                    error!( &err_string  );
                                                }
                                            }

                                        }
                                        Err( err ) => {
                                            error!("SettingsAPIKeyMessage::RegenerateAPIKey user parse error", err );
                                        }
                                    }

                                }

                                Err( err ) => {
                                    error!("SettingsAPIKeyMessage::RegenerateAPIKey error", err );
                                }
                            }

                        }
                    );
                    return true;
                }
                return false;
            }

            SettingsAPIKeyMessage::RegenerateAPIKey( _ ) => {

                let dialog = ConfirmationDialogDefinition {
                    title: Some("API Key Regeneration Confirmation".to_string()),
                    callback: ctx.link().callback(SettingsAPIKeyMessage::ConfirmYes),
                    html: None,
                    text: Some("Remember: any attached apps will lose access if you regenerate your API key. Are you sure you want to do this?".to_string()),
                    label_yes: None,
                    label_no: None,
                };

                ctx.props().global_vars.open_confirmation_dialog.emit( dialog );

                return true;
            }

        }

    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let mut global_vars = ctx.props().global_vars.clone();
        global_vars.current_sub_menu = "settings_apikey".to_owned();

        if global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
                    page_title="Settings"
                    submenu_tag={"user".to_owned()}
                >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            }
        }

        if global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
                    page_title="Settings"
                    submenu_tag={"user".to_owned()}
                >
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
                </UIPage>
            }
        }

        if !global_vars.current_user.is_premium {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
                    page_title="Settings"
                    submenu_tag={"user".to_owned()}
                >
                <div class={"text-center"}>
                    <br />
                    {"You are not a WildCard subscriber!"}
                </div>
                </UIPage>
            }
        }



        html! {
            <UIPage
                global_vars={global_vars.clone()}
                page_title="API Key"
                submenu_tag={"user".to_owned()}
            >
                <h2><i class={"fa-solid fa-key"}></i><Nbsp />{"API Key"}</h2>
                <p>{"If you're planning on using the Savaged.us API and want to get more data than unregistered users, then you'll have to generate an API key to get to your data."}</p>

                <p>{"Currently, the API is undocumented, but there are open source repositories to help you reverse-engineer."}</p>

                <hr />

                <label class={"plain"}>
                    // <button
                    //     class="btn btn-primary pull-right"
                    //     onclick={ctx.link().callback( SettingsAPIKeyMessage::CopyAPIKey )}
                    // >
                    //     <i class={"fa fa-copy"} /><Nbsp />{"Copy to Clipboard"}
                    // </button>
                    {"API Key:"}<br />
                    <input
                        type={"text"}
                        class={"text-center"}
                        readonly={true}
                        value={global_vars.current_user.api_key}
                    />
                </label>

                <hr />
                <p>{"Click the following button if you need to regenerate your API key. Don't do this unless you feel your data has been compromised, as any attached apps will lose access."}</p>
                <div class={"text-center"}>
                    <button
                        type="button"
                        class="btn btn-primary"
                        onclick={ctx.link().callback( SettingsAPIKeyMessage::RegenerateAPIKey )}
                    >
                        <i class={"fa fa-refresh"} /><Nbsp />{"Regenerate API Key"}
                    </button>
                </div>
            </UIPage>

        }

    }
}

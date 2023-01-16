use yew::prelude::*;
// use standard_components::ui::input_text::InputText;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;

// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::set_document_title::set_document_title;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::components::ui_page::UIPage;

// use crate::lib::fetch_api::fetch_api;
// use crate::lib::fetch_api::savaged_login;

// use web_sys::console;
// use wasm_bindgen_futures::spawn_local;
// use gloo_utils::format::JsValueSerdeExt;

// use savaged_libs::user::User;
// use savaged_libs::user::LoginTokenResult;

#[derive(Properties, PartialEq)]
pub struct ForgotPasswordProps {
    pub global_vars: GlobalVars,
}

pub enum ForgotPasswordMessage {

}

pub struct ForgotPassword {
    // global_vars: GlobalVars,
}

impl Component for ForgotPassword {
    type Message = ForgotPasswordMessage;
    type Properties = ForgotPasswordProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Recover Password".to_owned(), global_vars.server_side_renderer,);
        ForgotPassword {
            // global_vars: global_vars,
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = ctx.props().global_vars.clone();
        if global_vars.user_loading {
            return html! {
                <UIPage
                global_vars={global_vars.clone()}
                page_title="Forgot Password"
                submenu_tag={"".to_owned()}
            >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            }
        }
        if global_vars.current_user.id > 0 {
            return html! {
                <UIPage
                global_vars={global_vars.clone()}
                page_title="Forgot Password"
                submenu_tag={"".to_owned()}
            >                <div class={"text-center"}>
                    <br />
                    {"You are already logged in!"}
                </div>
                </UIPage>
            }
        }
        html! {
            <UIPage
                global_vars={global_vars.clone()}
                page_title="Forgot Password"
                submenu_tag={"".to_owned()}
            >
                <h2><i class={"fa-solid fa-cogs"}></i><Nbsp />{"TODO: Forgot Password"}</h2>
            </UIPage>

        }

    }
}

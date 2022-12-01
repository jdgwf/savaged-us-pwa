use yew::prelude::*;
// use standard_components::ui::input_text::InputText;
use standard_components::ui::nbsp::Nbsp;

// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::set_document_title::set_document_title;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

// use crate::lib::fetch_api::fetch_api;
// use crate::lib::fetch_api::savaged_login;

// use web_sys::console;
// use wasm_bindgen_futures::spawn_local;
// use gloo_utils::format::JsValueSerdeExt;
use crate::libs::global_vars::GlobalVars;
// use savaged_libs::user::User;

#[derive(Properties, PartialEq)]
pub struct UserSubscriptionProps {
    pub global_vars: GlobalVars,
    pub update_global_vars: Callback<GlobalVars>,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum UserSubscriptionMessage {

}

pub struct UserSubscription {

    global_vars: GlobalVars,
}

impl Component for UserSubscription {
    type Message = UserSubscriptionMessage;
    type Properties = UserSubscriptionProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Subscriptions and Purchases".to_owned(), global_vars.no_calls,);
        UserSubscription {
            global_vars: global_vars,
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: UserSubscriptionMessage,
    ) -> bool {

        match msg {

            // SettingsPrivateMessage::UpdateCurrentUser( login_result ) => {

            //     return true;
            // }

        }

    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserSubscriptionProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();
        true
    }

    fn view(
        &self,
        _ctx: &Context<Self>,
    ) -> Html {

        // let global_vars = ctx.props().global_vars.clone();

        if self.global_vars.user_loading {
            return html! {
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
            }
        }

        if self.global_vars.current_user.id == 0 {
            return html! {
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
            }
        }

        html! {
            <>
                <h2><i class={"fa-solid fa-credit-card"}></i><Nbsp />{"TODO: My Subscriptions and Purchases"}</h2>
            </>
        }

    }
}

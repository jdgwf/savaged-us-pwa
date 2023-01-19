// use yew_router::prelude::*;
use yew::prelude::*;

use yew::html;

// use savaged_libs::user::User;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
// use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::components::ui_page::UIPage;
// use crate::main_app::SubmenuData;
// use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;
// use super::settings_public::SettingsPublic;
// use super::settings_private::SettingsPrivate;
// use super::settings_devices::SettingsDevices;
// use super::settings_api_key::SettingsAPIKey;
// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;
// use gloo_console::log;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Properties, PartialEq)]
pub struct UserCampaignsProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    // pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    // pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub struct UserCampaignsMessage {

}
pub struct UserCampaigns {
}

impl Component for UserCampaigns {
    type Message = UserCampaignsMessage;
    type Properties = UserCampaignsProps;

    fn create(ctx: &Context<Self>) -> Self {

        UserCampaigns {
        }
    }

    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

    //     true
    // }



    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut global_vars = ctx.props().global_vars.clone();
        if global_vars.user_loading {

            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="My Saves"
                    submenu_tag={"user-data".to_owned()}
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
                        global_vars={global_vars}
                        page_title="My Saves"
                        submenu_tag={"user-data".to_owned()}
                    >
                        <div class={"text-center"}>
                            <br />
                            {"You are not logged in!"}
                        </div>

                    </UIPage>
                }
            }

        global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data-campaigns".to_owned();

        html! {
            <UIPage
                global_vars={global_vars}
                page_title="My Campaigns"
                submenu_tag={"user-data".to_owned()}
            >
                <>{"Campaigns"}</>
            </UIPage>
        }
    }
}


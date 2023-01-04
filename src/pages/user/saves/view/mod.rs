
use savaged_libs::save_db_row::SaveDBRow;
use yew_router::prelude::*;
use yew::prelude::*;

use yew::{function_component, html};

// use savaged_libs::user::User;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::components::tertiary_menu::{
    TertiaryMenuItem,
    TertiaryMenu
};
use crate::components::ui_page::UIPage;
use crate::main_app::SubmenuData;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;
// use super::settings_public::SettingsPublic;
// use super::settings_private::SettingsPrivate;
// use super::settings_devices::SettingsDevices;
// use super::settings_api_key::SettingsAPIKey;
// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;
use gloo_console::log;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Properties, PartialEq)]
pub struct UserSavesViewProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    pub uuid: String,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum UserSavesViewMessage {
    ChangeFilter(String),
    ChangeFolder(String),
}
pub struct UserSavesView {
    global_vars: GlobalVars,
    save: Option<SaveDBRow>,
}

impl Component for UserSavesView {
    type Message = UserSavesViewMessage;
    type Properties = UserSavesViewProps;

    fn create(ctx: &Context<Self>) -> Self {

        let mut save: Option<SaveDBRow> = None;

        match ctx.props().global_vars.clone().saves {
            Some( local_saves ) => {
                for item in local_saves {
                    if item.uuid == ctx.props().uuid {
                        save = Some(item.clone());
                    }
                }
            }
            None => {

            }
        }


        UserSavesView {
            global_vars: ctx.props().global_vars.clone(),
            save: save,
        }
    }


    fn update(
        &mut self, ctx: &Context<Self>,
        msg: UserSavesViewMessage
    ) -> bool {


        match msg {
            UserSavesViewMessage::ChangeFilter( filter_type ) => {
                // log!("ChangeFilter", filter_type);
                set_local_storage_string( "saves_filter", filter_type);
            }

            UserSavesViewMessage::ChangeFolder( folder_name ) => {
                // log!("ChangeFolder", folder);
                set_local_storage_string( "saves_folder", folder_name);
            }
        }
        true
    }


    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserSavesViewProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();

        // read_notifications: self.global_vars.current_user.unread_notifications,
        //     };
        let mut save: Option<SaveDBRow> = None;

        match ctx.props().global_vars.clone().saves {
            Some( local_saves ) => {
                for item in local_saves {
                    if item.uuid == ctx.props().uuid {
                        self.save = Some(item.clone());
                    }
                }
            }
            None => {

            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut global_vars = ctx.props().global_vars.clone();

        global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data-saves".to_owned();


        if self.global_vars.user_loading {


            return html! {
            <UIPage
                global_vars={global_vars}
                page_title="Viewing Save"
                submenu_tag={"user-data".to_owned()}
            >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>

            </UIPage>
        }
        }

        if self.global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="Viewing Save"
                    submenu_tag={"user-data".to_owned()}
                >
                    <div class={"text-center"}>
                        <br />
                        {"You are not logged in!"}
                    </div>

                </UIPage>
            }
        }


        match &self.save {
            Some( save ) => {
                let mut form = html!{ <div class="text-center">{"TODO: Unhandled Save Type"}</div>};
                match save.save_type.as_ref() {
                    "character" => {
                        form = html!{ <div class="text-center">{"TODO: Character View Form"}</div>};
                    }
                    "setting" => {
                        form = html!{ <div class="text-center">{"TODO: Setting View Form"}</div>};
                    }
                    "race" => {
                        form = html!{ <div class="text-center">{"TODO: Race View Form"}</div>};
                    }
                    "bestiary" => {
                        form = html!{ <div class="text-center">{"TODO: Bestiary View Form"}</div>};
                    }
                    "gear" => {
                        form = html!{ <div class="text-center">{"TODO: Gear View Form"}</div>};
                    }
                    "weapon" => {
                        form = html!{ <div class="text-center">{"TODO: Weapon View Form"}</div>};
                    }
                    "armor" => {
                        form = html!{ <div class="text-center">{"TODO: Armor View Form"}</div>};
                    }
                    "hindrances" => {
                        form = html!{ <div class="text-center">{"TODO: Hindrance View Form"}</div>};
                    }
                    "edges" => {
                        form = html!{ <div class="text-center">{"TODO: Edge View Form"}</div>};
                    }
                    "starship" => {
                        form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }
                    "power-armor" => {
                        form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }
                    "vehicle" => {
                        form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }
                    "walker" => {
                        form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }

                    _ => {
                        html!{ <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }
                return html! {
                    <UIPage
                        global_vars={global_vars.clone()}
                        page_title="Viewing Save"
                        submenu_tag={"user-data".to_owned()}
                    >
                        <strong>{"Save UUID:"}</strong><Nbsp />{&save.uuid}<br />
                        <strong>{"Save Name:"}</strong><Nbsp />{&save.name}<br />
                        // {"Type:"}<Nbsp />{&save.save_type}<br />
                        <br />
                        {form}
                    </UIPage>}
            }
            None => {
                return html!{
                    <UIPage
                        global_vars={global_vars.clone()}
                        page_title="Viewing Save"
                        submenu_tag={"user-data".to_owned()}
                    >
                        {"Cannot find save!"}
                    </UIPage>
                }
            }
        }


    }
}




use savaged_libs::player_character::hindrance::Hindrance;
use savaged_libs::player_character::edge::Edge;
use savaged_libs::player_character::weapon::Weapon;
use savaged_libs::player_character::armor::Armor;
use savaged_libs::player_character::gear::Gear;
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
use crate::components::edit_forms::hindrance::EditHindrance;
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
pub struct UserSavesEditProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    pub uuid: String,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum UserSavesEditMessage {
    UpdateHindrance(Hindrance),
    ChangeFolder(String),
}
pub struct UserSavesEdit {
    global_vars: GlobalVars,
    save: Option<SaveDBRow>,

    editing_hindrance: Option<Hindrance>,
    editing_edge: Option<Edge>,
    editing_weapon: Option<Weapon>,
    editing_gear: Option<Gear>,
    editing_armor: Option<Armor>,

}

impl Component for UserSavesEdit {
    type Message = UserSavesEditMessage;
    type Properties = UserSavesEditProps;

    fn create(ctx: &Context<Self>) -> Self {

        let mut save_option: Option<SaveDBRow> = None;

        let mut editing_hindrance: Option<Hindrance> = None;
        let mut editing_edge: Option<Edge> = None;

        let mut editing_weapon: Option<Weapon> = None;
        let mut editing_gear: Option<Gear> = None;
        let mut editing_armor: Option<Armor> = None;

        match ctx.props().global_vars.clone().saves {
            Some( local_saves ) => {
                for item in local_saves {
                    if item.uuid == ctx.props().uuid {
                        save_option = Some(item.clone());
                    }
                }
            }
            None => {

            }
        }

        match &save_option {
            Some( save ) => {
                match save.save_type.as_ref() {
                    "character" => {
                        // form = html!{ <div class="text-center">{"TODO: Character Edit Form"}</div>};
                    }
                    "setting" => {
                        // form = html!{ <div class="text-center">{"TODO: Setting Edit Form"}</div>};
                    }
                    "race" => {
                        // form = html!{ <div class="text-center">{"TODO: Race Edit Form"}</div>};
                    }
                    "bestiary" => {
                        // form = html!{ <div class="text-center">{"TODO: Bestiary Edit Form"}</div>};
                    }
                    "gear" => {
                        // form = html!{ <div class="text-center">{"TODO: Gear Edit Form"}</div>};
                        editing_gear = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "weapon" => {
                        // form = html!{ <div class="text-center">{"TODO: Weapon Edit Form"}</div>};
                        editing_weapon = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "armor" => {
                        // form = html!{ <div class="text-center">{"TODO: Armor Edit Form"}</div>};
                        editing_armor = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "hindrances" => {
                        log!("setting Hindrance Data");
                        editing_hindrance = serde_json::from_str(save.data.as_str()).unwrap();

                    }
                    "edges" => {
                        // form = html!{ <div class="text-center">{"TODO: Edge Edit Form"}</div>};
                        editing_edge = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "starship" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }
                    "power-armor" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }
                    "vehicle" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }
                    "walker" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }

                    _ => {
                        log!( format!("Unhandled save type: {}", &save.save_type ) );
                        // html!{ <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }

            }
            None => {
                log!("create() Cannot find save!");
            }
        }

        log!( format!("editing_hindrance {:?}", editing_hindrance ));

        UserSavesEdit {
            global_vars: ctx.props().global_vars.clone(),
            save: save_option,

            editing_hindrance: editing_hindrance,
            editing_edge: editing_edge,
            editing_weapon: editing_weapon,
            editing_gear: editing_gear,
            editing_armor: editing_armor,
        }
    }


    fn update(
        &mut self, ctx: &Context<Self>,
        msg: UserSavesEditMessage
    ) -> bool {


        match msg {
            UserSavesEditMessage::UpdateHindrance( new_value ) => {
                // log!("ChangeFilter", filter_type);
                self.editing_hindrance = Some(new_value);
            }

            UserSavesEditMessage::ChangeFolder( folder_name ) => {
                // log!("ChangeFolder", folder);
                set_local_storage_string( "saves_folder", folder_name);
            }
        }
        true
    }


    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserSavesEditProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();

        // self.global_vars = ctx.props().global_vars.clone();

        // read_notifications: self.global_vars.current_user.unread_notifications,
        //     };
        // let mut save: Option<SaveDBRow> = None;


        // let mut save_option: Option<SaveDBRow> = None;

        self.editing_hindrance = None;
        self.editing_edge = None;
        self.editing_weapon = None;
        self.editing_gear = None;
        self.editing_armor = None;

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

        match &self.save {
            Some( save ) => {
                match save.save_type.as_ref() {
                    "character" => {
                        // form = html!{ <div class="text-center">{"TODO: Character Edit Form"}</div>};
                    }
                    "setting" => {
                        // form = html!{ <div class="text-center">{"TODO: Setting Edit Form"}</div>};
                    }
                    "race" => {
                        // form = html!{ <div class="text-center">{"TODO: Race Edit Form"}</div>};
                    }
                    "bestiary" => {
                        // form = html!{ <div class="text-center">{"TODO: Bestiary Edit Form"}</div>};
                    }
                    "gear" => {
                        // form = html!{ <div class="text-center">{"TODO: Gear Edit Form"}</div>};
                        self.editing_gear = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "weapon" => {
                        // form = html!{ <div class="text-center">{"TODO: Weapon Edit Form"}</div>};
                        self.editing_weapon = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "armor" => {
                        // form = html!{ <div class="text-center">{"TODO: Armor Edit Form"}</div>};
                        self.editing_armor = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "hindrances" => {
                        log!("setting Hindrance Data");
                        self.editing_hindrance = serde_json::from_str(save.data.as_str()).unwrap();

                    }
                    "edges" => {
                        // form = html!{ <div class="text-center">{"TODO: Edge Edit Form"}</div>};
                        self.editing_edge = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "starship" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }
                    "power-armor" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }
                    "vehicle" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }
                    "walker" => {
                        // form = html!{ <div class="text-center">{"TODO: SciFi Vehicle 2014 Edit Form"}</div>};
                    }

                    _ => {
                        log!( format!("Unhandled save type: {}", &save.save_type ) );
                        // html!{ <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }

            }
            None => {
                log!("create() Cannot find save!");
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
                    page_title="Editing Save"
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
                    page_title="Editing Save"
                    submenu_tag={"user-data".to_owned()}
                >
                    <div class={"text-center"}>
                        <br />
                        {"You are not logged in!"}
                    </div>

                </UIPage>
            }
        }

        let mut form = html!{<></>};
        match &self.save {
            Some( save ) => {

                form = html!{<>
                    <h2>{"Unhandled Save"}</h2>
                    <strong>{"Save UUID:"}</strong><Nbsp />{&save.uuid}<br />
                    <strong>{"Save Name:"}</strong><Nbsp />{&save.name}<br />
                    <strong>{"Save Type:"}</strong><Nbsp />{&save.save_type}<br />
                    // {"Type:"}<Nbsp />{&save.save_type}<br />
                    <br />
                </>};
                match &self.editing_hindrance {
                    Some( hindrance ) => {

                        form = html!{
                            <EditHindrance
                                global_vars={ctx.props().global_vars.clone()}
                                edit_save={save.clone()}
                                edit_item={hindrance.clone()}
                                on_changed_callback={ctx.link().callback(UserSavesEditMessage::UpdateHindrance)}
                            />
                        };
                    }
                    None => {
                        log!("Cannot find Hindrance?");
                    }
                }

                return html! {
                    <UIPage
                        global_vars={global_vars.clone()}
                        page_title="Editing Save"
                        submenu_tag={"user-data".to_owned()}
                    >

                        {form}

                    </UIPage>
                }
            }
            None => {
                return html!{
                    <UIPage
                        global_vars={global_vars.clone()}
                        page_title="Editing Save"
                        submenu_tag={"user-data".to_owned()}
                    >
                        {"Cannot find save!"}
                    </UIPage>
                }
            }
        }
    }
}


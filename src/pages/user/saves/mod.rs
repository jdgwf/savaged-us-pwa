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
pub struct UserSavesProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    // pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    // pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum UserSavesMessage {
    ChangeFilter(String),
}
pub struct UserSaves {
    global_vars: GlobalVars,
}

impl Component for UserSaves {
    type Message = UserSavesMessage;
    type Properties = UserSavesProps;

    fn create(ctx: &Context<Self>) -> Self {

        UserSaves {
            global_vars: ctx.props().global_vars.clone(),
        }
    }


    fn update(
        &mut self, ctx: &Context<Self>,
        msg: UserSavesMessage
    ) -> bool {


        match msg {
            UserSavesMessage::ChangeFilter( filter_type ) => {
                log!("ChangeFilter", filter_type);
                // set_local_storage_string( "saves_filter", filter_type);
            }
        }
        true
    }


    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserSavesProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();

        // read_notifications: self.global_vars.current_user.unread_notifications,
        //     };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut global_vars = ctx.props().global_vars.clone();


        let mut filter_type = "character".to_owned();

        if !ctx.props().global_vars.server_side_renderer {
            filter_type = get_local_storage_string( "saves_filter" , "character".to_string());
        }
        if self.global_vars.user_loading {


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

        if self.global_vars.current_user.id == 0 {
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
        global_vars.current_sub_menu = "user-data-saves".to_owned();

        let mut saves: Vec<SaveDBRow> = Vec::new();

        match &global_vars.saves {
            Some( save_items ) => {
                saves = save_items.to_vec();
            }
            None => {

            }
        }

        let server_root = global_vars.server_root.clone();

        let change_filter_callback_character = ctx.link().callback(UserSavesMessage::ChangeFilter);

        let filter_by = | save: &SaveDBRow | {
            let filter_type= filter_type.to_owned();

            if filter_type == "scifi2014".to_owned() {
                if save.save_type == "starship"
                    || save.save_type == "vehicle"
                    || save.save_type == "power-armor"
                    || save.save_type == "walker" {
                        return true;
                    }
            } else if filter_type == "gear".to_owned() {
                if save.save_type == "gear"
                    || save.save_type == "weapon"
                    || save.save_type == "armor"
                    || save.save_type == "cybernetics" {
                        return true;
                    }
            }  else if filter_type == "other".to_owned() {
                if save.save_type == "edges"
                    || save.save_type == "hindrances" {
                        return true;
                    }
            } else if save.save_type == filter_type {
                return true;
            }

            return false;

        };

        let sub_menu_items: Vec<TertiaryMenuItem> = vec![
            TertiaryMenuItem {
                tag: "character".to_owned(),
                label: "Characters".to_owned(),
                class: None,
                callback: None,
            },
            TertiaryMenuItem {
                tag: "setting".to_owned(),
                label: "Settings".to_owned(),
                class: None,
                callback: None,
            },
            TertiaryMenuItem{
                tag: "race".to_owned(),
                label: "Races".to_owned(),
                class: None,
                callback: None,
            },
            TertiaryMenuItem {
                tag: "bestiary".to_owned(),
                label: "Bestiary".to_owned(),
                class: None,
                callback: None,
            },
            TertiaryMenuItem {
                tag: "gear".to_owned(),
                label: "Gear".to_owned(),
                class: None,
                callback: None,
            },
            TertiaryMenuItem {
                tag: "other".to_owned(),
                label: "Other".to_owned(),
                class: None,
                callback: None,
            },
            TertiaryMenuItem {
                tag: "scifi2014".to_owned(),
                label: "Sci-fi Vehicles".to_owned(),
                class: None,
                callback: None,
            },
        ];

        html! {
            <UIPage
                global_vars={global_vars.clone()}
                page_title="My Saves"
                submenu_tag={"user-data".to_owned()}
            >
                <TertiaryMenu
                    global_vars={global_vars.clone()}
                    menu_items={sub_menu_items}
                    menu_changed_callback={change_filter_callback_character}
                    local_storage_variable={"saves_filter".to_owned()}
                />

                <div class="saves-card-container">
                {saves.into_iter().filter( filter_by ).map( |save| {
                    if !save.deleted {
                        let mut image_style = "".to_owned();
                        if !save.imageurl.is_empty() {
                            image_style = "background-image: url('".to_owned() + &server_root + &save.imageurl + &"')";
                        }
                        html!{
                            <div
                                class="save-card"
                                style={image_style}
                            >
                                <div class="text">
                                    <h3>{save.name}</h3>
                                    <hr />
                                    {save.short_desc}<br />
                                    <br />
                                    <div class="small-text">{save.save_type}<Nbsp />{"|"}<Nbsp />{save.folder}</div>
                                    <div class="small-text">{save.uuid}</div>
                                </div>

                                <div class={"controls"}>
                                    <button
                                        class="btn btn-primary"
                                    >
                                        <i class={"fa fa-edit"} />
                                    </button>
                                    <button
                                        class="btn btn-danger"
                                    >
                                        <i class={"fa fa-trash"} />
                                    </button>
                                    // <button
                                    //     class="btn btn-secondary"
                                    // >
                                    //     <i class={"fa fa-bars"} />
                                    // </button>

                                    <ul class="styleless">
                                        <li>{"Sub-Menu"}</li>
                                        <li>{"Sub-Menu"}</li>
                                        <li>{"Sub-Menu"}</li>
                                        <li>{"Sub-Menu"}</li>
                                    </ul>
                                </div>
                            </div>
                        }
                    } else {
                        html!{<></>}
                    }
                }).collect::<Html>()}
                </div>
            </UIPage>
        }
    }
}


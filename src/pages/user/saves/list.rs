
use std::collections::HashMap;

use savaged_libs::save_db_row::SaveDBRow;
use yew_router::prelude::*;
use yew::prelude::*;

use yew::{html};

// use savaged_libs::user::User;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::components::tertiary_menu::{
    TertiaryMenuItem,
    TertiaryMenu
};

use crate::pages::user::saves::UserSavesRoute;
use crate::components::ui_page::UIPage;
// use crate::main_app::SubmenuData;
use standard_components::ui::nbsp::Nbsp;
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
pub struct UserSavesListProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    pub global_vars: GlobalVars,
}

pub enum UserSavesListMessage {
    ChangeFilter(String),
    ChangeFolder(String),
}
pub struct UserSavesList {
    global_vars: GlobalVars,
}

impl Component for UserSavesList {
    type Message = UserSavesListMessage;
    type Properties = UserSavesListProps;

    fn create(ctx: &Context<Self>) -> Self {

        UserSavesList {
            global_vars: ctx.props().global_vars.clone(),
        }
    }

    fn update(
        &mut self, ctx: &Context<Self>,
        msg: UserSavesListMessage
    ) -> bool {

        match msg {
            UserSavesListMessage::ChangeFilter( filter_type ) => {
                // log!("ChangeFilter", filter_type);
                set_local_storage_string( "saves_filter", filter_type);
            }

            UserSavesListMessage::ChangeFolder( folder_name ) => {
                // log!("ChangeFolder", folder);
                set_local_storage_string( "saves_folder", folder_name);
            }
        }
        true
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserSavesListProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();



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

        let change_filter_callback_character = ctx.link().callback(UserSavesListMessage::ChangeFilter);
        let change_folder_callback = ctx.link().callback(UserSavesListMessage::ChangeFolder);

        let mut current_available_folders: Vec<String> = Vec::new();
        let mut current_folder_counts: HashMap< String, u32> = HashMap::new();

        let filter_by_type = | save: &SaveDBRow | {
            let filter_type= filter_type.to_owned();
            // let current_folder= current_folder.to_owned();

            if save.deleted {
                return false;
            }

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
            } else if filter_type == "other".to_owned() {
                if save.save_type == "edges"
                    || save.save_type == "hindrances" {
                        return true;
                    }
            } else if save.save_type == filter_type {
                return true;

            }

            return false;

        };

        let mut character_count = 0;
        let mut setting_count = 0;
        let mut race_count = 0;
        let mut gear_count = 0;
        let mut other_count = 0;
        let mut scifi_count = 0;
        let mut bestiary_count = 0;
        let mut trash_count = 0;

        for item in global_vars.clone().saves.unwrap_or(Vec::new()) {

            if item.deleted {
                trash_count += 1;
            } else {
                if filter_by_type(&item) {
                    if !item.folder.is_empty()
                        && !current_available_folders.contains(&item.folder)
                    {
                        current_available_folders.push(item.folder.clone());
                    }
                    if !item.folder.is_empty() {
                        current_folder_counts.entry(item.folder).and_modify(|count| *count += 1).or_insert(1);
                    }
                }

                if item.save_type == "character".to_owned() {
                    character_count += 1;
                }
                if item.save_type == "race".to_owned() {
                    race_count += 1;
                }
                if item.save_type == "setting".to_owned() {
                    setting_count += 1;
                }
                if item.save_type == "bestiary".to_owned() {
                    bestiary_count += 1;
                }
                if item.save_type == "gear".to_owned()
                    ||item.save_type == "weapon".to_owned()
                    || item.save_type == "armor".to_owned() {
                    gear_count += 1;
                }
                if item.save_type == "hindrances".to_owned()
                    ||item.save_type == "edges".to_owned() {
                    other_count += 1;
                }
                if item.save_type == "starship"
                || item.save_type == "vehicle"
                || item.save_type == "power-armor"
                || item.save_type == "walker" {
                    scifi_count += 1;
                }
            }

        }

        let mut current_folder = get_local_storage_string("saves_folder", "".to_string());
        let current_available_folders_list= current_available_folders.clone();
        if !current_folder.is_empty() && !current_available_folders_list.contains(&current_folder) {
            current_folder = "".to_string();
            set_local_storage_string("saves_folder", "".to_string());
        }
        let filter_by = | save: &SaveDBRow | {
            let filter_type= filter_type.to_owned();
            // let mut current_folder= current_folder.to_owned();

            if save.deleted {
                return false;
            }

            if filter_type == "scifi2014".to_owned() {
                if save.save_type == "starship"
                    || save.save_type == "vehicle"
                    || save.save_type == "power-armor"
                    || save.save_type == "walker" {
                        if current_folder == save.folder {
                            return true;
                        }
                    }
            } else if filter_type == "gear".to_owned() {
                if save.save_type == "gear"
                    || save.save_type == "weapon"
                    || save.save_type == "armor"
                    || save.save_type == "cybernetics" {
                        if current_folder == save.folder {
                            return true;
                        }
                    }
            } else if filter_type == "other".to_owned() {
                if save.save_type == "edges"
                    || save.save_type == "hindrances" {
                        if current_folder == save.folder {
                            return true;
                        }
                    }
            } else if save.save_type == filter_type {
                if current_folder == save.folder {
                    return true;
                }
            }

            return false;

        };

        let mut sub_menu_items: Vec<TertiaryMenuItem> = vec![
            TertiaryMenuItem {
                tag: "character".to_owned(),
                label: "Characters".to_owned() + &" (" + &character_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "setting".to_owned(),
                label: "Settings".to_owned() + &" (" + &setting_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem{
                tag: "race".to_owned(),
                label: "Races".to_owned() + &" (" + &race_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "bestiary".to_owned(),
                label: "Bestiary".to_owned() + &" (" + &bestiary_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "gear".to_owned(),
                label: "Gear".to_owned() + &" (" + &gear_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "other".to_owned(),
                label: "Other".to_owned() + &" (" + &other_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "scifi2014".to_owned(),
                label: "Sci-fi Vehicles".to_owned() + &" (" + &scifi_count.to_string() + &")",
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
        ];

        if ctx.props().global_vars.current_user.has_premium_access() {
            sub_menu_items.push(
                TertiaryMenuItem {
                    tag: "_!_trash_|_".to_owned(),
                    label: "Trash".to_owned() + &" (" + &trash_count.to_string() + &")",
                    class: None,
                    callback: None,
                    title: None,
                    icon_class: Some("fa fa-trash".to_owned()),
                    separate: false,
                },
            );
        }

        sub_menu_items.push(
            TertiaryMenuItem {
                tag: "".to_owned(),
                label: "Add".to_owned(),
                class: Some("abs-right success-tab".to_owned()),
                callback: Some(Callback::noop()),
                title: None,
                icon_class: Some("fa fa-plus".to_owned()),
                separate: true,
            },
        );

        current_available_folders.sort();
        let change_folder_callback1 = change_folder_callback.clone();
        let change_folder_callback2 = change_folder_callback.clone();

        let open_confirmation_dialog = ctx.props().global_vars.open_confirmation_dialog.clone();
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

                if !current_folder.is_empty() {
                    <h3 class="no-margins text-center">{"Current Folder:"}<Nbsp />{&current_folder}</h3>
                }
                <div class="saves-card-container">
                if !(&current_folder).is_empty() {
                    <div
                        class="folder"
                    >
                    <div
                        class="folder-nip"
                        onclick={move |_e| {

                            change_folder_callback1.emit("".to_owned());
                        }}
                    >

                    </div>
                    <div
                        class="folder-base folder-up"
                        onclick={move |_e| {
                            change_folder_callback2.emit("".to_owned());
                        }}
                    >
                        <i class="fa-solid fa-arrow-up-from-bracket"></i>
                        <h3>{"Back To Base Folder"}</h3>
                    </div>

                </div>
                } else {

                    {current_available_folders.into_iter().map( | folder | {
                    let change_folder_callback1 = change_folder_callback.clone();
                    let change_folder_callback2 = change_folder_callback.clone();

                    let mut folder_count = 0;
                    match current_folder_counts.get_key_value(&folder.clone()) {
                        Some( (_hash, val) ) => {
                            folder_count = *val;
                        }
                        None => {

                        }
                    }
                    let folder1 = folder.to_owned();
                    let folder2 = folder.to_owned();

                    html! {
                        <div
                            class="folder"
                            title={format!("This folder '{}' has {} saves.", &folder, &folder_count)}
                        >
                            <div
                                class="folder-nip"
                                onclick={move |_e| {

                                    change_folder_callback1.emit(folder1.to_owned());
                                }}

                            >
                                {&folder_count}
                            </div>
                            <div
                                class="folder-base"
                                onclick={move |_e| {
                                    change_folder_callback2.emit(folder2.to_owned());
                                }}
                            >
                                <h3>{&folder}</h3>
                            </div>
                            <div class={"controls"}>

                            <button
                                type="button"
                                class="btn btn-success"
                                // onclick={ move | _event | {
                                //     let mut conf_def: ConfirmationDialogDefinition = ConfirmationDialogDefinition::default();
                                //     conf_def.text = format!("Are you sure you want to delete '{}' (Note: this won't happen yet. This is just a confirm box)?", &save_name);
                                //     conf_def.callback = Callback::noop();
                                //     open_confirmation_dialog.emit(
                                //         conf_def
                                //     );
                                // }}
                            >
                                <i class={"fa fa-edit"} />
                            </button>
                            <button
                                type="button"
                                class="btn btn-danger"
                                // onclick={ move | _event | {
                                //     let mut conf_def: ConfirmationDialogDefinition = ConfirmationDialogDefinition::default();
                                //     conf_def.text = format!("Are you sure you want to delete '{}' (Note: this won't happen yet. This is just a confirm box)?", &save_name);
                                //     conf_def.callback = Callback::noop();
                                //     open_confirmation_dialog.emit(
                                //         conf_def
                                //     );
                                // }}
                            >
                                <i class={"fa fa-trash"} />
                            </button>

                        </div>
                        </div>
                    }
                    }).collect::<Html>()
                }
            }

                {saves.into_iter().filter( filter_by ).map( |save| {
                    let mut image_style = "".to_owned();
                    let save_name = save.name.clone();
                    let save_uuid = save.uuid.clone();
                    let open_confirmation_dialog = open_confirmation_dialog.clone();
                    match save.image_base64 {
                        Some( image_base64 ) => {
                            image_style = format!("background-image: url(\"data::{};base64, {}\");", save.image_base64_mime.unwrap(), &image_base64, );
                        }
                        None => {}
                    }

                    let mut created_on_html = html!{ <></> };
                    let mut updated_on_html = html!{ <></> };
                    match save.created_on {
                        Some( created_on ) => {
                            created_on_html = html!{ <div class="small-text"><strong>{"Created On"}</strong><br />{global_vars.current_user.format_datetime(created_on, false, true, false)}</div> };
                        }
                        None => {

                        }
                    }

                    match save.updated_on {
                        Some( updated_on ) => {
                            if &save.created_on != &save.updated_on {
                                updated_on_html = html!{ <div class="small-text"><strong>{"Updated On"}</strong><br />{global_vars.current_user.format_datetime(updated_on, false, true, false)}</div> };
                            }
                        }
                        None => {

                        }
                    }

                    // log!( format!("s co {:?}", save.created_on));
                    // log!( format!("s uo {:?}", save.updated_on));

                    html!{
                        <div
                            class="save-card"
                            style={image_style}
                        >
                            <div class="text">
                            if save.deleted {
                                {"DELETED"}
                            }
                                <h3>{save.name}</h3>
                                <hr />
                                {save.short_desc}<br />
                                <br />
                                // <div class="small-text">{save.save_type}<Nbsp />{"|"}<Nbsp />{save.folder}</div>
                                // <div class="small-text">{save.uuid.to_owned()}</div>
                                {created_on_html}
                                {updated_on_html}

                            </div>

                            <div class={"controls"}>
                                <Link<UserSavesRoute>
                                    to={UserSavesRoute::View { uuid: save.uuid.to_owned() }}
                                >
                                    <span
                                        class="btn btn-secondary"
                                    >
                                        <i class={"fa fa-eye"} />
                                    </span>
                                </Link<UserSavesRoute>>

                                <Link<UserSavesRoute>
                                    to={UserSavesRoute::Edit { uuid: save.uuid.to_owned() }}
                                >
                                    <span
                                        class="btn btn-primary"
                                    >
                                        <i class={"fa fa-edit"} />
                                    </span>
                                </Link<UserSavesRoute>>
                                <button
                                    type="button"
                                    class="btn btn-danger"
                                    onclick={ move | _event | {
                                        let mut conf_def: ConfirmationDialogDefinition = ConfirmationDialogDefinition::default();
                                        conf_def.text = Some(format!("Are you sure you want to delete '{}' (Note: this won't happen yet. This is just a confirm box)?", &save_name));
                                        conf_def.callback = Callback::noop();
                                        open_confirmation_dialog.emit(
                                            conf_def
                                        );
                                    }}
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

                }).collect::<Html>()}
                </div>
            </UIPage>
        }
    }
}


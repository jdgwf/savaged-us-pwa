use crate::components::edit_forms::armor::EditArmor;
use crate::components::edit_forms::edge::EditEdge;
use crate::components::edit_forms::gear::EditGear;
use crate::components::edit_forms::hindrance::EditHindrance;
use crate::components::edit_forms::weapon::EditWeapon;
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::local_storage::{get_saves_from_index_db, index_db_put_save};
use crate::pages::error404::Error404;
use crate::pages::user::UserRoute;
use chrono::Utc;
use gloo_console::{error, log};
use savaged_libs::player_character::armor::Armor;
use savaged_libs::player_character::edge::Edge;
use savaged_libs::player_character::gear::Gear;
use savaged_libs::player_character::hindrance::Hindrance;
use savaged_libs::player_character::weapon::Weapon;
use savaged_libs::save_db_row::SaveDBRow;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::ui::nbsp::Nbsp;
use standard_components::ui::standard_form_save_buttons::StandardFormSaveButtons;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserSavesEditProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    #[prop_or_default]
    pub new_save_type: Option<String>,

    pub uuid: String,
    pub global_vars: GlobalVars,
}

pub enum UserSavesEditMessage {
    // ChangeFolder(String),
    Cancel(bool),

    UpdateHindrance(Hindrance),
    SaveHindrance(bool),

    UpdateEdge(Edge),
    SaveEdge(bool),

    UpdateArmor(Armor),
    SaveArmor(bool),

    UpdateGear(Gear),
    SaveGear(bool),

    UpdateWeapon(Weapon),
    SaveWeapon(bool),
}
pub struct UserSavesEdit {
    save: Option<SaveDBRow>,

    editing_hindrance: Option<Hindrance>,
    editing_edge: Option<Edge>,
    editing_weapon: Option<Weapon>,
    editing_gear: Option<Gear>,
    editing_armor: Option<Armor>,

    is_adding: bool,
    redirect_back: bool,
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
            Some(local_saves) => {
                for item in local_saves {
                    if item.uuid == ctx.props().uuid {
                        save_option = Some(item.clone());
                    }
                }
            }
            None => {}
        }

        match &ctx.props().new_save_type {
            Some(save_type) => {
                match save_type.as_str() {
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
                        // editing_gear = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "cyberware" => {
                        // form = html!{ <div class="text-center">{"TODO: Gear Edit Form"}</div>};
                        // editing_gear = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "weapon"|"weapons" => {
                        // form = html!{ <div class="text-center">{"TODO: Weapon Edit Form"}</div>};
                        // editing_weapon = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "armor" => {
                        // form = html!{ <div class="text-center">{"TODO: Armor Edit Form"}</div>};
                        // editing_armor = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "hindrances" => {
                        // log!("setting Hindrance Data");
                        // editing_hindrance = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "edges" => {
                        // form = html!{ <div class="text-center">{"TODO: Edge Edit Form"}</div>};
                        // editing_edge = serde_json::from_str(save.data.as_str()).unwrap();
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
                        if !ctx.props().global_vars.server_side_renderer {
                            error!(format!("Unhandled add save type: {}", &save_type));
                        }
                        // html!{ <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }
            }
            None => {}
        }

        match &save_option {
            Some(save) => {
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
                    "cyberware" => {
                        // form = html!{ <div class="text-center">{"TODO: Gear Edit Form"}</div>};
                        // editing_gear = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "weapon"|"weapons" => {
                        // form = html!{ <div class="text-center">{"TODO: Weapon Edit Form"}</div>};
                        editing_weapon = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "armor" => {
                        // form = html!{ <div class="text-center">{"TODO: Armor Edit Form"}</div>};
                        // log!(format!("create - setting Armor Data\n{}", &save.data));
                        editing_armor = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "hindrances" => {
                        // log!("setting Hindrance Data");
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

                    "" => {
                        // handled by Add above....
                    }

                    _ => {
                        if !ctx.props().global_vars.server_side_renderer {
                            error!(format!("Unhandled save type: {}", &save.save_type));
                        }
                        // html!{ <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }
            }
            None => {
                if !ctx.props().global_vars.server_side_renderer {
                    error!("create() Cannot find save!");
                }
            }
        }

        // log!( format!("editing_hindrance {:?}", editing_hindrance ));

        UserSavesEdit {
            save: save_option,

            editing_hindrance: editing_hindrance,
            editing_edge: editing_edge,
            editing_weapon: editing_weapon,
            editing_gear: editing_gear,
            editing_armor: editing_armor,

            is_adding: false,
            redirect_back: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: UserSavesEditMessage) -> bool {
        match msg {
            UserSavesEditMessage::UpdateHindrance(new_value) => {
                // log!("ChangeFilter", filter_type);
                self.editing_hindrance = Some(new_value);
            }
            UserSavesEditMessage::SaveHindrance(_new_value) => {
                let editing_hindrance = self.editing_hindrance.clone();
                let mut save = self.save.clone().unwrap();
                let close_callback = UserSavesEditMessage::Cancel.clone();
                match editing_hindrance {
                    Some(editing_hindrance) => {
                        let item = editing_hindrance.clone();
                        let server_root = ctx.props().global_vars.server_root.clone();
                        let mut global_vars = ctx.props().global_vars.clone();
                        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                        save.data = serde_json::to_string(&item).unwrap();
                        save.name = item.name;
                        save.updated_on = Some(Utc::now());
                        save.updated_by = global_vars.current_user.id;
                        close_callback(true);
                        spawn_local(async move {
                            index_db_put_save(server_root, save).await;
                            global_vars.saves = get_saves_from_index_db().await;
                            update_global_vars.emit(global_vars);
                        });
                        self.close_and_cancel();
                    }
                    None => {}
                }
            }

            UserSavesEditMessage::UpdateArmor(new_value) => {
                // log!("ChangeFilter", filter_type);
                self.editing_armor = Some(new_value);
            }
            UserSavesEditMessage::SaveArmor(_new_value) => {
                let editing_armor = self.editing_armor.clone();
                let mut save = self.save.clone().unwrap();
                let close_callback = UserSavesEditMessage::Cancel.clone();
                match editing_armor {
                    Some(editing_armor) => {
                        let item = editing_armor.clone();
                        let server_root = ctx.props().global_vars.server_root.clone();
                        let mut global_vars = ctx.props().global_vars.clone();
                        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                        save.data = serde_json::to_string(&item).unwrap();
                        save.name = item.name;
                        save.updated_on = Some(Utc::now());
                        save.updated_by = global_vars.current_user.id;
                        close_callback(true);
                        spawn_local(async move {
                            index_db_put_save(server_root, save).await;
                            global_vars.saves = get_saves_from_index_db().await;
                            update_global_vars.emit(global_vars);
                        });
                        self.close_and_cancel();
                    }
                    None => {}
                }
            }

            UserSavesEditMessage::UpdateGear(new_value) => {
                // log!("ChangeFilter", filter_type);
                self.editing_gear = Some(new_value);
            }
            UserSavesEditMessage::SaveGear(_new_value) => {
                let editing_gear = self.editing_gear.clone();
                let mut save = self.save.clone().unwrap();
                let close_callback = UserSavesEditMessage::Cancel.clone();
                match editing_gear {
                    Some(editing_gear) => {
                        let item = editing_gear.clone();
                        let server_root = ctx.props().global_vars.server_root.clone();
                        let mut global_vars = ctx.props().global_vars.clone();
                        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                        save.data = serde_json::to_string(&item).unwrap();
                        save.name = item.name;
                        save.updated_on = Some(Utc::now());
                        save.updated_by = global_vars.current_user.id;
                        close_callback(true);
                        spawn_local(async move {
                            index_db_put_save(server_root, save).await;
                            global_vars.saves = get_saves_from_index_db().await;
                            update_global_vars.emit(global_vars);
                        });
                        self.close_and_cancel();
                    }
                    None => {}
                }
            }

            UserSavesEditMessage::UpdateWeapon(new_value) => {
                // log!("ChangeFilter", filter_type);
                self.editing_weapon = Some(new_value);
            }
            UserSavesEditMessage::SaveWeapon(_new_value) => {
                let editing_weapon = self.editing_weapon.clone();
                let mut save = self.save.clone().unwrap();
                let close_callback = UserSavesEditMessage::Cancel.clone();
                match editing_weapon {
                    Some(editing_weapon) => {
                        let item = editing_weapon.clone();
                        let server_root = ctx.props().global_vars.server_root.clone();
                        let mut global_vars = ctx.props().global_vars.clone();
                        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                        save.data = serde_json::to_string(&item).unwrap();
                        save.name = item.name;
                        save.updated_on = Some(Utc::now());
                        save.updated_by = global_vars.current_user.id;
                        close_callback(true);
                        spawn_local(async move {
                            index_db_put_save(server_root, save).await;
                            global_vars.saves = get_saves_from_index_db().await;
                            update_global_vars.emit(global_vars);
                        });
                        self.close_and_cancel();
                    }
                    None => {}
                }
            }

            UserSavesEditMessage::UpdateEdge(new_value) => {
                // log!("ChangeFilter", filter_type);
                self.editing_edge = Some(new_value);
            }
            UserSavesEditMessage::SaveEdge(_new_value) => {
                let editing_edge = self.editing_edge.clone();
                let mut save = self.save.clone().unwrap();
                let close_callback = UserSavesEditMessage::Cancel.clone();
                match editing_edge {
                    Some(editing_edge) => {
                        let item = editing_edge.clone();
                        let server_root = ctx.props().global_vars.server_root.clone();
                        let mut global_vars = ctx.props().global_vars.clone();
                        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                        save.data = serde_json::to_string(&item).unwrap();
                        save.name = item.name;
                        save.updated_on = Some(Utc::now());
                        save.updated_by = global_vars.current_user.id;
                        close_callback(true);
                        spawn_local(async move {
                            index_db_put_save(server_root, save).await;
                            global_vars.saves = get_saves_from_index_db().await;
                            update_global_vars.emit(global_vars);
                        });
                        self.close_and_cancel();
                    }
                    None => {}
                }
            }

            UserSavesEditMessage::Cancel(_new_value) => {
                // log!("Cancel called");
                self.close_and_cancel();
            } // UserSavesEditMessage::ChangeFolder( folder_name ) => {
              //     // log!("ChangeFolder", folder);
              //     set_local_storage_string( "saves_folder", folder_name);
              // }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &UserSavesEditProps) -> bool {
        // ctx.props().global_vars = ctx.props().global_vars.clone();

        self.editing_hindrance = None;
        self.editing_edge = None;
        self.editing_weapon = None;
        self.editing_gear = None;
        self.editing_armor = None;

        match ctx.props().global_vars.clone().saves {
            Some(local_saves) => {
                for item in local_saves {
                    if item.uuid == ctx.props().uuid {
                        self.save = Some(item.clone());
                    }
                }
            }
            None => {}
        }

        match &self.save {
            Some(save) => {
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
                    "cyberware" => {
                        // form = html!{ <div class="text-center">{"TODO: Gear Edit Form"}</div>};
                        // editing_gear = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "weapon"|"weapons" => {
                        // form = html!{ <div class="text-center">{"TODO: Weapon Edit Form"}</div>};
                        self.editing_weapon = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "armor" => {
                        // form = html!{ <div class="text-center">{"TODO: Armor Edit Form"}</div>};
                        // log!(format!("update - setting Armor Data\n{}", &save.data));
                        self.editing_armor = serde_json::from_str(save.data.as_str()).unwrap();
                    }
                    "hindrances" => {
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
                        error!(format!("Unhandled save type: {}", &save.save_type));
                        // html!{ <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }
            }
            None => {
                error!("create() Cannot find save!");
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut global_vars = ctx.props().global_vars.clone();

        // global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data-saves".to_owned();

        if self.redirect_back {
            return html! { <Redirect<UserRoute> to={UserRoute::UserSavesList}/> };
        }

        if global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="Editing Save"

                >
                    <div class={"text-center"}>
                        <br />
                        {"Loading..."}
                    </div>

                </UIPage>
            };
        }

        if global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="Editing Save"

                >
                    <div class={"text-center"}>
                        <br />
                        {"You are not logged in!"}
                    </div>

                </UIPage>
            };
        }

        let mut page_title = "Unhandled Save".to_owned();
        match &self.save {
            Some(save) => {
                let mut form = html! {<>
                    <h2>{"Unhandled Save"}</h2>
                    <strong>{"Save UUID:"}</strong><Nbsp />{&save.uuid}<br />
                    <strong>{"Save Name:"}</strong><Nbsp />{&save.name}<br />
                    <strong>{"Save Type:"}</strong><Nbsp />{&save.save_type}<br />
                    // {"Type:"}<Nbsp />{&save.save_type}<br />
                    <br />
                </>};
                match &self.editing_hindrance {
                    Some(hindrance) => {
                        let mut save_callback: Option<Callback<bool>> = None;
                        let mut save_as_new_callback: Option<Callback<bool>> = None;
                        let mut add_callback: Option<Callback<bool>> = None;

                        if self.is_adding {
                            page_title = "Adding Hindrance".to_owned();
                            add_callback = Some(
                                ctx.link()
                                    .callback(UserSavesEditMessage::SaveHindrance)
                                    .clone(),
                            );
                        } else {
                            page_title = "Editing Hindrance".to_owned();
                            save_callback = Some(
                                ctx.link()
                                    .callback(UserSavesEditMessage::SaveHindrance)
                                    .clone(),
                            );
                        }

                        form = html! {
                            <>
                            <EditHindrance
                                global_vars={global_vars.clone()}
                                readonly={false}
                                edit_item={hindrance.clone()}
                                form_title={Some("Editing Hindrance")}
                                on_changed_callback={ctx.link().callback(UserSavesEditMessage::UpdateHindrance)}
                            />

                            <StandardFormSaveButtons
                                close_cancel_callback={ctx.link().callback(UserSavesEditMessage::Cancel).clone()}
                                save_callback={save_callback}
                                add_callback={add_callback}
                                save_as_new_callback={save_as_new_callback}
                            />

                            </>
                        };
                    }
                    None => {
                        // don't replace form.
                    }
                }

                match &self.editing_edge {
                    Some(edge) => {
                        let mut save_callback: Option<Callback<bool>> = None;
                        let mut save_as_new_callback: Option<Callback<bool>> = None;
                        let mut add_callback: Option<Callback<bool>> = None;

                        if self.is_adding {
                            page_title = "Adding Edge".to_owned();
                            add_callback =
                                Some(ctx.link().callback(UserSavesEditMessage::SaveEdge).clone());
                        } else {
                            page_title = "Editing Edge".to_owned();
                            save_callback =
                                Some(ctx.link().callback(UserSavesEditMessage::SaveEdge).clone());
                        }

                        form = html! {
                            <>
                            <EditEdge
                                global_vars={global_vars.clone()}
                                readonly={false}
                                edit_item={edge.clone()}
                                form_title={Some("Editing Edge")}
                                on_changed_callback={ctx.link().callback(UserSavesEditMessage::UpdateEdge)}
                            />

                            <StandardFormSaveButtons
                                close_cancel_callback={ctx.link().callback(UserSavesEditMessage::Cancel).clone()}
                                save_callback={save_callback}
                                add_callback={add_callback}
                                save_as_new_callback={save_as_new_callback}
                            />

                            </>
                        };
                    }
                    None => {
                        // don't replace form.
                    }
                }

                match &self.editing_weapon {
                    Some(weapon) => {
                        let mut save_callback: Option<Callback<bool>> = None;
                        let mut save_as_new_callback: Option<Callback<bool>> = None;
                        let mut add_callback: Option<Callback<bool>> = None;

                        if self.is_adding {
                            page_title = "Adding Weapon".to_owned();
                            add_callback = Some(
                                ctx.link()
                                    .callback(UserSavesEditMessage::SaveWeapon)
                                    .clone(),
                            );
                        } else {
                            page_title = "Editing Weapon".to_owned();
                            save_callback = Some(
                                ctx.link()
                                    .callback(UserSavesEditMessage::SaveWeapon)
                                    .clone(),
                            );
                        }

                        form = html! {
                            <>
                            <EditWeapon
                                global_vars={global_vars.clone()}
                                readonly={false}
                                edit_item={weapon.clone()}
                                form_title={Some("Editing Weapon")}
                                on_changed_callback={ctx.link().callback(UserSavesEditMessage::UpdateWeapon)}
                            />

                            <StandardFormSaveButtons
                                close_cancel_callback={ctx.link().callback(UserSavesEditMessage::Cancel).clone()}
                                save_callback={save_callback}
                                add_callback={add_callback}
                                save_as_new_callback={save_as_new_callback}
                            />

                            </>
                        };
                    }
                    None => {
                        // don't replace form.
                    }
                }

                match &self.editing_gear {
                    Some(gear) => {
                        let mut save_callback: Option<Callback<bool>> = None;
                        let mut save_as_new_callback: Option<Callback<bool>> = None;
                        let mut add_callback: Option<Callback<bool>> = None;

                        if self.is_adding {
                            page_title = "Adding Gear".to_owned();
                            add_callback =
                                Some(ctx.link().callback(UserSavesEditMessage::SaveGear).clone());
                        } else {
                            page_title = "Editing Gear".to_owned();
                            save_callback =
                                Some(ctx.link().callback(UserSavesEditMessage::SaveGear).clone());
                        }

                        form = html! {
                            <>
                            <EditGear
                                global_vars={global_vars.clone()}
                                readonly={false}
                                edit_item={gear.clone()}
                                form_title={Some("Editing Gear")}
                                on_changed_callback={ctx.link().callback(UserSavesEditMessage::UpdateGear)}
                            />

                            <StandardFormSaveButtons
                                close_cancel_callback={ctx.link().callback(UserSavesEditMessage::Cancel).clone()}
                                save_callback={save_callback}
                                add_callback={add_callback}
                                save_as_new_callback={save_as_new_callback}
                            />

                            </>
                        };
                    }
                    None => {
                        // don't replace form.
                    }
                }

                match &self.editing_armor {
                    Some(armor) => {
                        let mut save_callback: Option<Callback<bool>> = None;
                        let mut save_as_new_callback: Option<Callback<bool>> = None;
                        let mut add_callback: Option<Callback<bool>> = None;

                        if self.is_adding {
                            page_title = "Adding Armor".to_owned();
                            add_callback =
                                Some(ctx.link().callback(UserSavesEditMessage::SaveArmor).clone());
                        } else {
                            page_title = "Editing Armor".to_owned();
                            save_callback =
                                Some(ctx.link().callback(UserSavesEditMessage::SaveArmor).clone());
                        }

                        form = html! {
                            <>
                            <EditArmor
                                global_vars={global_vars.clone()}
                                readonly={false}
                                edit_item={armor.clone()}
                                form_title={Some("Editing Armor")}
                                on_changed_callback={ctx.link().callback(UserSavesEditMessage::UpdateArmor)}
                            />

                            <StandardFormSaveButtons
                                close_cancel_callback={ctx.link().callback(UserSavesEditMessage::Cancel).clone()}
                                save_callback={save_callback}
                                add_callback={add_callback}
                                save_as_new_callback={save_as_new_callback}
                            />

                            </>
                        };
                    }
                    None => {
                        // don't replace form.
                    }
                }

                return html! {
                    <UIPage
                        global_vars={global_vars.clone()}
                        page_title={page_title}

                    >
                        {form}
                    </UIPage>
                };
            }
            None => {
                return html! {
                    html! {
                        <Error404
                            global_vars={global_vars}
                        />
                    }
                }
            }
        }
    }
}

impl UserSavesEdit {
    fn close_and_cancel(&mut self) {
        self.editing_hindrance = None;
        self.editing_edge = None;
        self.editing_gear = None;
        self.editing_weapon = None;
        self.editing_armor = None;

        self.redirect_back = true;
    }
}

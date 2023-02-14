use crate::components::admin::admin_filter_search::AdminTableFilterSearch;
use crate::components::admin::admin_table::AdminTable;
use crate::components::admin::admin_table_field::active::AdminTableFieldActive;
use crate::components::admin::admin_table_ownership_badge::AdminTableOwnershipBadge;
use crate::components::admin::admin_table_paging::AdminTablePaging;
use crate::components::admin::edit_view_delete_buttons::EditViewDeleteButtons;
use crate::components::alerts::AlertDefinition;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::components::edit_forms::armor::EditArmor;
use crate::components::standard_modal::StandardModal;
use crate::components::tertiary_links_menu::{TertiaryLinksMenu, TertiaryLinksMenuItem};
use crate::components::ui_page::UIPage;
use crate::libs::admin_api::{fetch_api_delete_game_data_row, fetch_api_save_game_data_row};
use crate::libs::global_vars::GlobalVars;
use crate::{
    components::admin::admin_table_field::text::AdminTableFieldText,
    libs::fetch_api::fetch_admin_api,
};
use gloo_console::{error, log};
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::admin_libs::{
    AdminDeletePackage, AdminPagingStatistics, AdminSavePackage, AdminSaveReturn,
};
use savaged_libs::alert_level::AlertLevel;
use savaged_libs::book::Book;
use savaged_libs::game_data_row::GameDataRow;
use savaged_libs::player_character::armor::Armor;
use savaged_libs::{admin_libs::new_fetch_admin_params, admin_libs::FetchAdminParameters};
use serde_json::Error;
use standard_components::libs::local_storage_shortcuts::{
    get_local_storage_u32, set_local_storage_u32, get_local_storage_bool
};
use standard_components::ui::nbsp::Nbsp;
// use std::mem;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminGameDataArmorProps {
    pub global_vars: GlobalVars,
    pub sub_menu_items: Vec<TertiaryLinksMenuItem>,
}

pub enum AdminGameDataArmorMessage {
    SetItems(Vec<Armor>),
    NewItem(u32),
    SetPagingStats(Option<AdminPagingStatistics>),
    SetFetchAdminParams(FetchAdminParameters),
    UpdateArmor(Armor),
    UpdateArmorAndRefresh(Armor),

    ViewItem(u32),
    EditItemDialog(u32),
    DeleteItem(u32),
    DuplicateItem(u32),

    AddItemDialog(bool),

    Cancel(bool),
    SaveItemAndLeaveOpen(bool),
    SaveItem(bool),
}
pub struct AdminGameDataArmor {
    // global_vars: GlobalVars,
    items: Vec<Armor>,
    paging_data: Option<AdminPagingStatistics>,
    paging_sorting_and_filter: FetchAdminParameters,
    loading: bool,
    editing_item: Option<Armor>,
    is_adding: bool,
    is_editing: bool,
}

impl Component for AdminGameDataArmor {
    type Message = AdminGameDataArmorMessage;
    type Properties = AdminGameDataArmorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let global_vars = ctx.props().global_vars.clone();

        let login_token = global_vars.login_token.clone();
        let set_items = ctx.link().callback(AdminGameDataArmorMessage::SetItems);
        let set_paging = ctx
            .link()
            .callback(AdminGameDataArmorMessage::SetPagingStats);

        let mut paging_sorting_and_filter = new_fetch_admin_params();

        paging_sorting_and_filter.login_token = Some(login_token);
        paging_sorting_and_filter.needs_book_list = true;
        paging_sorting_and_filter.number_per_page = get_local_storage_u32(
            "admin_page_count",
            paging_sorting_and_filter.number_per_page,
        );
        paging_sorting_and_filter.filter_book =
            get_local_storage_u32("admin_selected_book", paging_sorting_and_filter.filter_book);

        paging_sorting_and_filter.hide_no_select =
            get_local_storage_bool("admin_hide_no_select", paging_sorting_and_filter.hide_no_select);
        let paging = paging_sorting_and_filter.clone();
        spawn_local(async move {
            _get_data(
                global_vars,
                paging_sorting_and_filter,
                set_items,
                set_paging,
            )
            .await;
        });

        AdminGameDataArmor {
            paging_sorting_and_filter: paging,
            // global_vars: ctx.props().global_vars.clone(),
            items: Vec::new(),
            paging_data: None,
            loading: true,
            editing_item: None,
            is_adding: false,
            is_editing: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: AdminGameDataArmorMessage) -> bool {
        match msg {
            AdminGameDataArmorMessage::ViewItem(id) => {
                // self.editing_item = None;
                for item in self.items.clone().into_iter() {
                    if item.id == id {
                        self.editing_item = Some(item.clone());
                        self.is_editing = false;
                        self.is_adding = false;
                        return true;
                    }
                }
                return false;
            }

            AdminGameDataArmorMessage::EditItemDialog(id) => {
                // self.editing_item = None;
                for item in self.items.clone().into_iter() {
                    if item.id == id {
                        self.editing_item = Some(item.clone());
                        self.is_editing = true;
                        self.is_adding = false;
                        return true;
                    }
                }
                return false;
            }

            AdminGameDataArmorMessage::AddItemDialog(_nv) => {
                log!("AdminGameDataArmorMessage::AddItemDialog");
                let mut new_item = Armor::default();
                new_item.book_id = self.paging_sorting_and_filter.filter_book;
                new_item.active = true;
                self.editing_item = Some(new_item);

                self.is_editing = false;
                self.is_adding = true;
                return true;
            }

            AdminGameDataArmorMessage::SaveItem(as_new) => {
                log!("AdminGameDataArmorMessage::SaveItem");
                let self_editing_item = self.editing_item.clone();
                let self_is_adding = self.is_adding;
                match self_editing_item {
                    Some(mut editing_item) => {
                        if as_new || self_is_adding {
                            editing_item.id = 0;
                        }
                        let req = AdminSavePackage {
                            id: editing_item.id,
                            fetch_parameters: self.paging_sorting_and_filter.clone(),
                            data: serde_json::to_string(&editing_item).unwrap(),
                            name: editing_item.name,
                            book_id: editing_item.book_id,
                            login_token: Some(ctx.props().global_vars.login_token.to_owned()),
                            api_key: None,
                        };

                        let api_root = ctx.props().global_vars.api_root.to_owned();
                        let global_vars = ctx.props().global_vars.clone();
                        // let item_name = editing_item.name.to_owned();
                        let set_items = ctx.link().callback(AdminGameDataArmorMessage::SetItems);
                        spawn_local(async move {
                            let result = fetch_api_save_game_data_row(
                                (api_root + "/admin/game-data/armor/save").to_owned(),
                                req,
                            )
                            .await;

                            match result {
                                Ok(value) => {
                                    let save_result: Result<AdminSaveReturn, Error> =
                                        JsValueSerdeExt::into_serde(&value);
                                    match save_result {
                                        Ok(save_result_data) => {
                                            match save_result_data.game_data {
                                                Some(vec_val) => {
                                                    let mut rv: Vec<Armor> = Vec::new();
                                                    for mut data in vec_val.into_iter() {
                                                        data.created_by_user = None;
                                                        data.updated_by_user = None;
                                                        data.updated_by_user = None;

                                                        // log!("data", format!("{:?}", data) );
                                                        let item = data.to_armor().unwrap();
                                                        // log!("data.updated_on", data.updated_on);
                                                        // log!("data.created_on", data.created_on);

                                                        // log!("hind.updated_on", hind.updated_on);
                                                        // log!("hind.created_on", hind.created_on);
                                                        // log!("data.updated_by_user", format!("{:?}", data.updated_by_user) );
                                                        // log!("data.updated_by", data.updated_by);
                                                        // log!("data.created_by", data.created_by);

                                                        // log!("hind.updated_by_obj", format!("{:?}", hind.updated_by_obj) );
                                                        // log!("hind.updated_by", hind.updated_by);
                                                        // log!("hind.created_by", hind.created_by);
                                                        rv.push(item)
                                                    }
                                                    set_items.emit(rv);

                                                    let alert_def: AlertDefinition =
                                                        AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some(save_result_data.message),
                                                            ..Default::default()
                                                        };
                                                    global_vars.add_alert.emit(alert_def);
                                                }

                                                None => {
                                                    set_items.emit(Vec::new());
                                                    let alert_def: AlertDefinition =
                                                        AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some(save_result_data.message),
                                                            ..Default::default()
                                                        };
                                                    global_vars.add_alert.emit(alert_def);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            let err_string: String =
                                                format!("SaveItem Serde Err(): {}", &err);
                                            // set_paging.emit( None );
                                            set_items.emit(Vec::new());
                                            error!(&err_string);
                                            let alert_def: AlertDefinition = AlertDefinition {
                                                level: AlertLevel::Danger,
                                                text: Some(format!("{:?}", err)),
                                                ..Default::default()
                                            };

                                            global_vars.add_alert.emit(alert_def);
                                        }
                                    }
                                }

                                Err(err) => {
                                    set_items.emit(Vec::new());
                                    error!("get_items paging Err()", &err);
                                    let alert_def: AlertDefinition = AlertDefinition {
                                        level: AlertLevel::Danger,
                                        text: Some(format!("{:?}", err)),
                                        ..Default::default()
                                    };

                                    global_vars.add_alert.emit(alert_def);
                                }
                            }
                        });

                        self.editing_item = None;
                    }
                    None => {}
                }
            }

            AdminGameDataArmorMessage::NewItem(book_id) => {
                let self_editing_item = self.editing_item.clone();
                let mut new_item = Armor::default();
                match self_editing_item {
                    Some(editing_item) => {
                        new_item.active = editing_item.active;
                        new_item.book_id = editing_item.book_id;
                    }
                    None => {
                        new_item.active = true;
                        new_item.book_id = book_id;
                    }
                }

                self.editing_item = Some(new_item);

                return true;
            }

            AdminGameDataArmorMessage::SaveItemAndLeaveOpen(_unused) => {
                log!("AdminGameDataArmorMessage::SaveItemAndLeaveOpen");
                let self_editing_item = self.editing_item.clone();
                let self_is_adding = self.is_adding;

                match self_editing_item {
                    Some(mut editing_item) => {
                        if self_is_adding {
                            editing_item.id = 0;
                        }
                        let editing_item_name = editing_item.name.clone();
                        let req = AdminSavePackage {
                            id: editing_item.id,
                            fetch_parameters: self.paging_sorting_and_filter.clone(),
                            data: serde_json::to_string(&editing_item).unwrap(),
                            name: editing_item_name,
                            book_id: editing_item.book_id,
                            login_token: Some(ctx.props().global_vars.login_token.to_owned()),
                            api_key: None,
                        };

                        let edit_item_id = editing_item.id;
                        let edit_item_active = editing_item.active;
                        let edit_item_book_id = editing_item.book_id;
                        let edit_item_book_page = editing_item.page.to_owned();

                        let api_root = ctx.props().global_vars.api_root.to_owned();
                        let global_vars = ctx.props().global_vars.clone();
                        // let item_name = editing_item.name.to_owned();
                        let set_items = ctx.link().callback(AdminGameDataArmorMessage::SetItems);
                        let new_item_callback =
                            ctx.link().callback(AdminGameDataArmorMessage::NewItem);

                        let update_armor_callback = ctx
                            .link()
                            .callback(AdminGameDataArmorMessage::UpdateArmorAndRefresh);

                        spawn_local(async move {
                            let result = fetch_api_save_game_data_row(
                                (api_root + "/admin/game-data/armor/save").to_owned(),
                                req,
                            )
                            .await;

                            match result {
                                Ok(value) => {
                                    let save_result: Result<AdminSaveReturn, Error> =
                                        JsValueSerdeExt::into_serde(&value);
                                    match save_result {
                                        Ok(save_result_data) => {
                                            match save_result_data.game_data {
                                                Some(vec_val) => {
                                                    if edit_item_id == 0 {
                                                        new_item_callback.emit(edit_item_book_id);
                                                    }

                                                    let mut rv: Vec<Armor> = Vec::new();
                                                    for mut data in vec_val.into_iter() {
                                                        data.created_by_user = None;
                                                        data.updated_by_user = None;
                                                        data.updated_by_user = None;

                                                        let item = data.to_armor().unwrap();

                                                        // log!("data", format!("{:?}", data) );

                                                        // log!("data.updated_on", data.updated_on);
                                                        // log!("data.created_on", data.created_on);

                                                        // log!("hind.updated_on", hind.updated_on);
                                                        // log!("hind.created_on", hind.created_on);
                                                        // log!("data.updated_by_user", format!("{:?}", data.updated_by_user) );
                                                        // log!("data.updated_by", data.updated_by);
                                                        // log!("data.created_by", data.created_by);

                                                        // log!("hind.updated_by_obj", format!("{:?}", hind.updated_by_obj) );
                                                        // log!("hind.updated_by", hind.updated_by);
                                                        // log!("hind.created_by", hind.created_by);
                                                        rv.push(item)
                                                    }
                                                    set_items.emit(rv);
                                                    let alert_def: AlertDefinition =
                                                        AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some(save_result_data.message),
                                                            ..Default::default()
                                                        };
                                                    global_vars.add_alert.emit(alert_def);

                                                    let mut new_item = Armor::default();
                                                    new_item.book_id = edit_item_book_id;
                                                    new_item.active = edit_item_active;
                                                    new_item.page = edit_item_book_page;
                                                    update_armor_callback.emit(new_item);
                                                }

                                                None => {
                                                    set_items.emit(Vec::new());
                                                    let alert_def: AlertDefinition =
                                                        AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some(save_result_data.message),
                                                            ..Default::default()
                                                        };
                                                    global_vars.add_alert.emit(alert_def);
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            let err_string: String =
                                                format!("SaveItem Serde Err(): {}", &err);
                                            // set_paging.emit( None );
                                            set_items.emit(Vec::new());
                                            error!(&err_string);
                                            let alert_def: AlertDefinition = AlertDefinition {
                                                level: AlertLevel::Danger,
                                                text: Some(format!("{:?}", err)),
                                                ..Default::default()
                                            };

                                            global_vars.add_alert.emit(alert_def);
                                        }
                                    }
                                }

                                Err(err) => {
                                    set_items.emit(Vec::new());
                                    error!("get_items paging Err()", &err);
                                    let alert_def: AlertDefinition = AlertDefinition {
                                        level: AlertLevel::Danger,
                                        text: Some(format!("{:?}", err)),
                                        ..Default::default()
                                    };

                                    global_vars.add_alert.emit(alert_def);
                                }
                            }
                        });

                        // self.editing_item = None;
                    }
                    None => {}
                }
            }

            AdminGameDataArmorMessage::DeleteItem(id) => {
                log!("AdminGameDataArmorMessage::DeleteItem ", id);

                let api_root = ctx.props().global_vars.api_root.to_owned();
                let global_vars = ctx.props().global_vars.clone();
                let login_token = Some(ctx.props().global_vars.login_token.to_owned());
                let set_items = ctx.link().callback(AdminGameDataArmorMessage::SetItems);
                let paging_sorting_and_filter = self.paging_sorting_and_filter.clone();

                for item in self.items.clone().into_iter() {
                    if item.id == id {
                        let open_confirmation_dialog =
                            ctx.props().global_vars.open_confirmation_dialog.clone();
                        let global_vars = global_vars.clone();
                        let set_items = set_items.clone();
                        let api_root = api_root.clone();
                        let paging_sorting_and_filter = paging_sorting_and_filter.clone();
                        let login_token = login_token.clone();
                        let dialog = ConfirmationDialogDefinition {
                            title: Some("Deletion Confirmation".to_owned()),

                            html: None,
                            text: Some(
                                "Are you sure you would like to delete the armor '".to_owned()
                                    + &item.name
                                    + &"'?",
                            ),
                            label_yes: None,
                            label_no: None,
                            callback: Callback::from(move |_clicked_yes| {
                                let api_root = api_root.to_owned();
                                let global_vars = global_vars.clone();
                                let login_token = login_token.clone();
                                let set_items = set_items.clone();
                                let paging_sorting_and_filter = paging_sorting_and_filter.clone();

                                let mut editing_item = item.clone();

                                editing_item.id = 0;

                                let req = AdminDeletePackage {
                                    id: id,
                                    fetch_parameters: paging_sorting_and_filter,
                                    name: editing_item.name,
                                    login_token: login_token,
                                    api_key: None,
                                };

                                // let item_name = editing_item.name.to_owned();
                                spawn_local(async move {
                                    let result = fetch_api_delete_game_data_row(
                                        (api_root + "/admin/game-data/armor/delete").to_owned(),
                                        req,
                                    )
                                    .await;

                                    match result {
                                        Ok(value) => {
                                            let save_result: Result<AdminSaveReturn, Error> =
                                                JsValueSerdeExt::into_serde(&value);
                                            match save_result {
                                                Ok(save_result_data) => {
                                                    match save_result_data.game_data {
                                                        Some(vec_val) => {
                                                            let mut rv: Vec<Armor> = Vec::new();
                                                            for mut data in vec_val.into_iter() {
                                                                data.created_by_user = None;
                                                                data.updated_by_user = None;
                                                                data.updated_by_user = None;

                                                                let item = data.to_armor().unwrap();

                                                                rv.push(item)
                                                            }
                                                            set_items.emit(rv);
                                                            let alert_def: AlertDefinition =
                                                                AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some(
                                                                        save_result_data.message,
                                                                    ),
                                                                    ..Default::default()
                                                                };
                                                            global_vars.add_alert.emit(alert_def);
                                                        }

                                                        None => {
                                                            set_items.emit(Vec::new());
                                                            // error!("get_items Err()", &err );
                                                            let alert_def: AlertDefinition =
                                                                AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some(
                                                                        save_result_data.message,
                                                                    ),
                                                                    ..Default::default()
                                                                };
                                                            global_vars.add_alert.emit(alert_def);
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    let err_string: String =
                                                        format!("SaveItem Serde Err(): {}", &err);
                                                    // set_paging.emit( None );
                                                    set_items.emit(Vec::new());
                                                    error!(&err_string);
                                                    let alert_def: AlertDefinition =
                                                        AlertDefinition {
                                                            level: AlertLevel::Danger,
                                                            text: Some(format!("{:?}", err)),
                                                            ..Default::default()
                                                        };

                                                    global_vars.add_alert.emit(alert_def);
                                                }
                                            }
                                        }

                                        Err(err) => {
                                            set_items.emit(Vec::new());
                                            error!("get_items paging Err()", &err);
                                            let alert_def: AlertDefinition = AlertDefinition {
                                                level: AlertLevel::Danger,
                                                text: Some(format!("{:?}", err)),
                                                ..Default::default()
                                            };

                                            global_vars.add_alert.emit(alert_def);
                                        }
                                    }
                                });
                                // return false;
                            }),
                        };

                        open_confirmation_dialog.emit(dialog);
                    }
                }
            }

            AdminGameDataArmorMessage::DuplicateItem(id) => {
                log!("AdminGameDataArmorMessage::DuplicateItem", id);

                for item in self.items.clone().into_iter() {
                    if item.id == id {
                        let open_confirmation_dialog =
                            ctx.props().global_vars.open_confirmation_dialog.clone();

                        let api_root = ctx.props().global_vars.api_root.to_owned();
                        let login_token = Some(ctx.props().global_vars.login_token.to_owned());
                        let set_items = ctx.link().callback(AdminGameDataArmorMessage::SetItems);
                        let paging_sorting_and_filter = self.paging_sorting_and_filter.clone();
                        let item = item.clone();
                        let global_vars = ctx.props().global_vars.clone();
                        let item_name = item.name.clone();

                        // let editing_item_name = item.name.to_owned();
                        let dialog = ConfirmationDialogDefinition {
                            title: Some("Duplication Confirmation".to_owned()),

                            html: None,
                            text: Some(
                                "Are you sure you would like to duplicate the armor '".to_owned()
                                    + &item_name
                                    + &"'?",
                            ),
                            label_yes: None,
                            label_no: None,
                            callback: Callback::from(move |_clicked_yes| {
                                let global_vars = global_vars.clone();
                                let item_name = item_name.clone();

                                let api_root = api_root.to_owned();
                                let login_token = login_token.clone();
                                let set_items = set_items.clone();
                                let paging_sorting_and_filter = paging_sorting_and_filter.clone();

                                let mut editing_item = item.clone();

                                editing_item.id = 0;

                                let req = AdminSavePackage {
                                    id: 0,
                                    fetch_parameters: paging_sorting_and_filter,
                                    name: item_name.to_owned(),
                                    data: serde_json::to_string(&editing_item).unwrap(),
                                    book_id: editing_item.book_id,
                                    login_token: login_token,
                                    api_key: None,
                                };

                                spawn_local(async move {
                                    let result = fetch_api_save_game_data_row(
                                        (api_root + "/admin/game-data/armor/save").to_owned(),
                                        req,
                                    )
                                    .await;

                                    match result {
                                        Ok(value) => {
                                            let save_result: Result<AdminSaveReturn, Error> =
                                                JsValueSerdeExt::into_serde(&value);
                                            match save_result {
                                                Ok(save_result_data) => {
                                                    match save_result_data.game_data {
                                                        Some(vec_val) => {
                                                            let mut rv: Vec<Armor> = Vec::new();
                                                            for mut data in vec_val.into_iter() {
                                                                data.created_by_user = None;
                                                                data.updated_by_user = None;
                                                                data.updated_by_user = None;

                                                                let item = data.to_armor().unwrap();

                                                                rv.push(item)
                                                            }
                                                            set_items.emit(rv);

                                                            let alert_def: AlertDefinition = AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some("Armor '".to_owned() + &item_name.to_owned() + &"' has been duplicated."),
                                                                    ..Default::default()
                                                                };
                                                            global_vars.add_alert.emit(alert_def);
                                                        }

                                                        None => {
                                                            set_items.emit(Vec::new());
                                                            let alert_def: AlertDefinition =
                                                                AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some(
                                                                        "No Data received!!"
                                                                            .to_owned(),
                                                                    ),
                                                                    ..Default::default()
                                                                };
                                                            global_vars.add_alert.emit(alert_def);
                                                            // error!("get_items Err()", &err );
                                                        }
                                                    }
                                                }
                                                Err(err) => {
                                                    let err_string: String =
                                                        format!("SaveItem Serde Err(): {}", &err);
                                                    // set_paging.emit( None );
                                                    set_items.emit(Vec::new());

                                                    let alert_def: AlertDefinition =
                                                        AlertDefinition {
                                                            level: AlertLevel::Danger,
                                                            text: Some(format!("{:?}", err)),
                                                            ..Default::default()
                                                        };

                                                    global_vars.add_alert.emit(alert_def);

                                                    error!(&err_string);
                                                }
                                            }
                                        }

                                        Err(err) => {
                                            set_items.emit(Vec::new());
                                            error!("get_items paging Err()", &err);
                                            let alert_def: AlertDefinition = AlertDefinition {
                                                level: AlertLevel::Danger,
                                                text: Some(format!("{:?}", err)),
                                                ..Default::default()
                                            };

                                            global_vars.add_alert.emit(alert_def);
                                        }
                                    }
                                });
                                // return false;
                            }),
                        };

                        open_confirmation_dialog.emit(dialog);

                        return false;
                    }
                }
            }

            AdminGameDataArmorMessage::Cancel(_new_value) => {
                log!("AdminGameDataArmorMessage::Cancel");
                self.editing_item = None;
            }

            AdminGameDataArmorMessage::UpdateArmor(new_value) => {
                self.editing_item = Some(new_value);
                return false;
            }
            AdminGameDataArmorMessage::UpdateArmorAndRefresh(new_value) => {
                self.editing_item = Some(new_value);
                return true;
            }

            AdminGameDataArmorMessage::SetItems(new_value) => {
                self.items = new_value;
                self.loading = false;
            }

            AdminGameDataArmorMessage::SetPagingStats(new_value) => {
                match new_value {
                    Some(mut nv) => {
                        match nv.book_list {
                            Some(bl) => {
                                nv.book_list = Some(bl);
                            }
                            None => match &self.paging_data {
                                Some(pg) => {
                                    nv.book_list = pg.book_list.clone();
                                }
                                None => {}
                            },
                        }
                        self.paging_data = Some(nv.clone());
                    }
                    None => {
                        self.paging_data = None;
                    }
                }

                self.loading = false;
            }

            AdminGameDataArmorMessage::SetFetchAdminParams(new_value) => {
                let mut paging_sorting_and_filter = new_value.clone();
                self.paging_sorting_and_filter = new_value.clone();

                let global_vars = ctx.props().global_vars.clone();

                let login_token = global_vars.login_token.clone();
                let set_items = ctx.link().callback(AdminGameDataArmorMessage::SetItems);
                let set_paging = ctx
                    .link()
                    .callback(AdminGameDataArmorMessage::SetPagingStats);

                set_local_storage_u32(
                    "admin_page_count",
                    paging_sorting_and_filter.number_per_page,
                );

                paging_sorting_and_filter.login_token = Some(login_token);

                paging_sorting_and_filter.needs_book_list = true;
                match &self.paging_data {
                    Some(paging) => match &paging.book_list {
                        Some(book_list) => {
                            if book_list.len() > 0 {
                                paging_sorting_and_filter.needs_book_list = false;
                            }
                        }
                        None => {}
                    },
                    None => {}
                }
                spawn_local(async move {
                    _get_data(
                        global_vars,
                        paging_sorting_and_filter,
                        set_items,
                        set_paging,
                    )
                    .await;
                });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback_fetch_admin_params = ctx
            .link()
            .callback(AdminGameDataArmorMessage::SetFetchAdminParams)
            .clone();
        let callback_fetch_admin_params_2 = ctx
            .link()
            .callback(AdminGameDataArmorMessage::SetFetchAdminParams)
            .clone();

        let mut non_filtered_count: u32 = 0;
        let mut filtered_count: u32 = 0;

        match &self.paging_data {
            Some(paging_data) => {
                non_filtered_count = paging_data.non_filtered_count;
                filtered_count = paging_data.filtered_count;
            }
            None => {}
        }

        let mut global_vars = ctx.props().global_vars.clone();
        global_vars.current_menu = "main-admin".to_owned();
        global_vars.current_sub_menu = "admin-game-data".to_owned();

        let mut show_book_column = true;

        let mut current_book_id: u32 = 0;
        if self.paging_sorting_and_filter.filter_book > 0 {
            show_book_column = false;
            current_book_id = self.paging_sorting_and_filter.filter_book;
        }

        let mut book_list: Option<Vec<Book>> = None;

        match &self.paging_data {
            Some(pg) => {
                book_list = pg.book_list.clone();
            }
            None => {}
        }

        let mut edit_modal = html! {<></>};
        match &self.editing_item {
            Some(editing_item) => {
                let mut editing_title = Some("Viewing Armor".to_owned());

                let mut save_callback: Option<Callback<bool>> = None;
                let mut add_callback: Option<Callback<bool>> = None;
                let mut save_as_new_callback: Option<Callback<bool>> = None;
                let mut save_and_leave_open_callback: Option<Callback<bool>> = None;

                let mut read_only = true;

                if self.is_adding {
                    editing_title = Some("Adding Armor".to_owned());
                    add_callback = Some(
                        ctx.link()
                            .callback(AdminGameDataArmorMessage::SaveItem)
                            .clone(),
                    );
                    save_and_leave_open_callback = Some(
                        ctx.link()
                            .callback(AdminGameDataArmorMessage::SaveItemAndLeaveOpen)
                            .clone(),
                    );
                    read_only = false;
                }

                if self.is_editing {
                    editing_title = Some("Editing Armor".to_owned());
                    save_callback = Some(
                        ctx.link()
                            .callback(AdminGameDataArmorMessage::SaveItem)
                            .clone(),
                    );
                    save_as_new_callback = Some(
                        ctx.link()
                            .callback(AdminGameDataArmorMessage::SaveItem)
                            .clone(),
                    );
                    save_and_leave_open_callback = Some(
                        ctx.link()
                            .callback(AdminGameDataArmorMessage::SaveItemAndLeaveOpen)
                            .clone(),
                    );
                    read_only = false;
                }

                let mut book_list: Vec<Book> = Vec::new();

                match self.paging_data.clone() {
                    Some(paging_data) => {
                        book_list = paging_data.book_list.unwrap_or(Vec::new());
                    }
                    None => {}
                }

                edit_modal = html! {
                <StandardModal
                    xl={true}
                    title={editing_title}
                    close_cancel_callback={Some(ctx.link().callback(AdminGameDataArmorMessage::Cancel).clone())}
                    save_callback={save_callback}
                    add_callback={add_callback}
                    save_as_new_callback={save_as_new_callback}
                    save_and_leave_open_callback={save_and_leave_open_callback}
                >
                    <EditArmor
                        for_admin={true}
                        global_vars={global_vars.clone()}
                        readonly={read_only}
                        edit_item={editing_item.clone()}
                        book_list={book_list}
                        on_changed_callback={ctx.link().callback(AdminGameDataArmorMessage::UpdateArmor).clone()}
                    />

                </StandardModal>
                };
            }
            None => {}
        }

        let add_item = ctx
            .link()
            .callback(AdminGameDataArmorMessage::AddItemDialog);

        let edit_callback = ctx.link().callback(AdminGameDataArmorMessage::EditItemDialog);
        let duplicate_callback = ctx.link().callback(AdminGameDataArmorMessage::DuplicateItem);
        let delete_callback = ctx.link().callback(AdminGameDataArmorMessage::DeleteItem);
        let view_callback = ctx.link().callback(AdminGameDataArmorMessage::ViewItem);

        html! {
        <UIPage
            global_vars={global_vars.clone()}
            page_title="Admin Armor"

            modal={Some(edit_modal)}
        >

        <TertiaryLinksMenu
            server_side_renderer={global_vars.server_side_renderer}
            menu_items={ctx.props().sub_menu_items.clone()}

            current_tag={"armor".to_owned()}
        />

        <div class="pull-right">
            <AdminTableFilterSearch
                callback_fetch_admin_params={callback_fetch_admin_params_2}
                paging_sorting_and_filter={self.paging_sorting_and_filter.clone()}
                stats={self.paging_data.clone()}
                current_user={global_vars.current_user.clone()}
                show_no_select={true}
            />
        </div>
            <AdminTable
                callback_fetch_admin_params={callback_fetch_admin_params_2}
                paging_sorting_and_filter={self.paging_sorting_and_filter.clone()}
                stats={self.paging_data.clone()}
                current_user={global_vars.current_user.clone()}
                show_no_select={true}
                add_item={add_item}
                book_list={book_list}
                current_book_id={current_book_id}
                show_book_column={show_book_column}
                loading={self.loading}
                edit_callback={edit_callback}
                duplicate_callback={duplicate_callback}
                delete_callback={delete_callback}
                view_callback={view_callback}
                non_filtered_count={non_filtered_count}
                filtered_count={filtered_count}
                items={self.items}
            />
            </UIPage>
        }
    }
}

async fn _get_data(
    global_vars: GlobalVars,
    paging_sorting_and_filter: FetchAdminParameters,
    set_items: Callback<Vec<Armor>>,
    set_paging: Callback<Option<AdminPagingStatistics>>,
) {
    let api_root = global_vars.api_root.clone();

    let result = fetch_admin_api(
        (api_root.to_owned() + "/admin/game-data/armor/get").to_owned(),
        paging_sorting_and_filter.clone(),
    )
    .await;

    match result {
        Ok(value) => {
            let vec_val_result: Result<Vec<GameDataRow>, Error> =
                JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    let mut rv: Vec<Armor> = Vec::new();
                    for data in vec_val.into_iter() {
                        // log!(format!("item {}", &data.data));
                        let item = data.to_armor().unwrap();
                        // log!(format!("armor {} {}", &item.name, mem::size_of_val(&item)));
                        rv.push(item);
                    }
                    // log!(format!("rv {}", mem::size_of_val(&rv)));
                    set_items.emit(rv);
                }
                Err(err) => {
                    let err_string: String = format!("get_items Serde Err(): {}", &err);
                    set_items.emit(Vec::new());
                    error!(&err_string);
                }
            }
        }
        Err(err) => {
            set_items.emit(Vec::new());
            error!("get_items Err()", &err);
        }
    }

    let result = fetch_admin_api(
        (api_root + "/admin/game-data/armor/paging").to_owned(),
        paging_sorting_and_filter.clone(),
    )
    .await;

    match result {
        Ok(value) => {
            // let vec_val_result = value.into_serde::< Vec<GameDataRow> >();
            let vec_val_result: Result<AdminPagingStatistics, Error> =
                JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    // log!(format!("vec_val {}", mem::size_of_val(&vec_val)));
                    // log!(format!(
                    //     "vec_val.book_list {}",
                    //     mem::size_of_val(&vec_val.book_list)
                    // ));
                    set_paging.emit(Some(vec_val));
                }
                Err(err) => {
                    let err_string: String = format!("get_items paging Serde Err(): {}", &err);
                    set_paging.emit(None);
                    error!(&err_string);
                }
            }
        }
        Err(err) => {
            set_paging.emit(None);
            error!("get_items paging Err()", &err);
        }
    }
}
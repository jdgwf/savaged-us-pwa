use crate::components::admin::admin_filter_search::AdminTableFilterSearch;
use crate::components::admin::admin_table_field::active::AdminTableFieldActive;
use crate::components::admin::admin_table_ownership_badge::AdminTableOwnershipBadge;
use crate::components::admin::admin_table_paging::AdminTablePaging;
use crate::components::admin::edit_view_delete_buttons::EditViewDeleteButtons;
use crate::components::alerts::AlertDefinition;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::components::edit_forms::hindrance::EditHindrance;
use crate::components::standard_modal::StandardModal;
use crate::components::tertiary_links_menu::{TertiaryLinksMenuItem, TertiaryLinksMenu};
use crate::components::ui_page::UIPage;
use crate::libs::admin_api::{fetch_api_save_game_data_row, fetch_api_delete_game_data_row};
use crate::libs::global_vars::GlobalVars;
use crate::{components::admin::admin_table_field::text::AdminTableFieldText, libs::fetch_api::fetch_admin_api};
use gloo_console::{ error, log };
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::admin_libs::{AdminPagingStatistics, AdminSavePackage, AdminSaveReturn, AdminDeletePackage};
use savaged_libs::alert_level::AlertLevel;
use savaged_libs::book::Book;
use savaged_libs::game_data_row::GameDataRow;
use savaged_libs::player_character::hindrance::Hindrance;
use savaged_libs::{ admin_libs::FetchAdminParameters, admin_libs::new_fetch_admin_params};
use serde_json::Error;
use standard_components::libs::local_storage_shortcuts::{get_local_storage_u32, set_local_storage_u32, get_local_storage_bool, };
use standard_components::ui::nbsp::Nbsp;
// use std::mem;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminGameDataHindrancesProps {
    pub global_vars: GlobalVars,
    pub sub_menu_items: Vec<TertiaryLinksMenuItem>,

}

pub enum AdminGameDataHindrancesMessage {
    SetItems(Vec<Hindrance>),
    NewItem( u32 ),
    SetPagingStats(Option<AdminPagingStatistics>),
    SetFetchAdminParams(FetchAdminParameters),
    UpdateHindrance(Hindrance),
    UpdateHindranceAndRefresh(Hindrance),

    ViewItem( u32 ),
    EditItemDialog( u32 ),
    DeleteItem( u32 ),
    DuplicateItem( u32 ),

    AddItemDialog(bool),

    Cancel(bool),
    SaveItemAndLeaveOpen(bool),
    SaveItem(bool),
}
pub struct AdminGameDataHindrances {
    items: Vec<Hindrance>,
    paging_data: Option<AdminPagingStatistics>,
    paging_sorting_and_filter: FetchAdminParameters,
    loading: bool,
    editing_item: Option<Hindrance>,
    is_adding: bool,
    is_editing: bool,
}

impl Component for AdminGameDataHindrances {
    type Message = AdminGameDataHindrancesMessage;
    type Properties = AdminGameDataHindrancesProps;

    fn create(ctx: &Context<Self>) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        let login_token = global_vars.login_token.clone();
        let set_items = ctx.link().callback(AdminGameDataHindrancesMessage::SetItems);
        let set_paging = ctx.link().callback(AdminGameDataHindrancesMessage::SetPagingStats);

        let mut paging_sorting_and_filter = new_fetch_admin_params();

        paging_sorting_and_filter.login_token = Some(login_token);
        paging_sorting_and_filter.needs_book_list = true;
        paging_sorting_and_filter.number_per_page = get_local_storage_u32("admin_page_count", paging_sorting_and_filter.number_per_page);
        paging_sorting_and_filter.filter_book = get_local_storage_u32("admin_selected_book", paging_sorting_and_filter.filter_book);
        paging_sorting_and_filter.hide_no_select = get_local_storage_bool("admin_hide_no_select", paging_sorting_and_filter.filter_book);
        let paging = paging_sorting_and_filter.clone();
        spawn_local (
            async move {
                _get_data(
                    global_vars,
                    paging_sorting_and_filter,
                    set_items,
                    set_paging,

                ).await;

            }
        );

        AdminGameDataHindrances {
            paging_sorting_and_filter: paging,
            items: Vec::new(),
            paging_data: None,
            loading: true,
            editing_item: None,
            is_adding: false,
            is_editing: false,
        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: AdminGameDataHindrancesMessage
    ) -> bool {

        match msg {

            AdminGameDataHindrancesMessage::ViewItem( id ) => {
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

            AdminGameDataHindrancesMessage::EditItemDialog( id ) => {
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

            AdminGameDataHindrancesMessage::AddItemDialog( _nv ) => {
                log!("AdminGameDataHindrancesMessage::AddItemDialog");
                let mut new_item = Hindrance::default();
                new_item.book_id = self.paging_sorting_and_filter.filter_book;
                new_item.active = true;
                self.editing_item = Some( new_item );

                self.is_editing = false;
                self.is_adding = true;
                return true;
            }

            AdminGameDataHindrancesMessage::SaveItem( as_new ) => {
                log!("AdminGameDataHindrancesMessage::SaveItem");
                let self_editing_item = self.editing_item.clone();
                let self_is_adding = self.is_adding;
                match self_editing_item {
                    Some( mut editing_item ) => {
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
                        let set_items = ctx.link().callback(AdminGameDataHindrancesMessage::SetItems);
                        spawn_local (
                            async move {
                                let result = fetch_api_save_game_data_row(
                                    (api_root + "/admin/game-data/hindrances/save").to_owned(),
                                    req,

                                ).await;

                                match result {
                                    Ok( value ) => {
                                        let save_result: Result<AdminSaveReturn, Error> = JsValueSerdeExt::into_serde(&value);
                                        match save_result {
                                            Ok( save_result_data) => {
                                                match save_result_data.game_data {
                                                    Some( vec_val ) => {

                                                        let mut rv: Vec<Hindrance> = Vec::default();
                                                        for mut data in vec_val.into_iter() {
                                                            data.created_by_user = None;
                                                            data.updated_by_user = None;
                                                            data.updated_by_user = None;

                                                            // log!("data", format!("{:?}", data) );
                                                            let item = data.to_hindrance().unwrap();
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
                                                            rv.push( item )
                                                        }
                                                        set_items.emit( rv );

                                                        let alert_def: AlertDefinition = AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some( save_result_data.message ),
                                                            ..Default::default()
                                                        };
                                                        global_vars.add_alert.emit( alert_def );
                                                    }

                                                    None => {
                                                        set_items.emit( Vec::new() );
                                                        let alert_def: AlertDefinition = AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some( save_result_data.message ),
                                                            ..Default::default()
                                                        };
                                                        global_vars.add_alert.emit( alert_def );                                                    }
                                                }
                                            }
                                            Err( err ) => {
                                                let err_string: String = format!("SaveItem Serde Err(): {}", &err);
                                                // set_paging.emit( None );
                                                set_items.emit( Vec::new() );
                                                error!( &err_string  );
                                                let alert_def: AlertDefinition = AlertDefinition {
                                                    level: AlertLevel::Danger,
                                                    text: Some( format!("{:?}", err ) ),
                                                    ..Default::default()
                                                };

                                                global_vars.add_alert.emit( alert_def );
                                            }
                                        }

                                    }

                                    Err( err ) => {
                                        set_items.emit( Vec::new() );
                                        error!("get_items paging Err()", &err );
                                        let alert_def: AlertDefinition = AlertDefinition {
                                            level: AlertLevel::Danger,
                                            text: Some( format!("{:?}", err ) ),
                                            ..Default::default()
                                        };

                                        global_vars.add_alert.emit( alert_def );
                                    }
                                }
                            }
                        );

                        self.editing_item = None;
                    }
                    None => {}
                }
            }

            AdminGameDataHindrancesMessage::NewItem( book_id ) => {
                let self_editing_item = self.editing_item.clone();
                let mut hind = Hindrance::default();
                match self_editing_item {
                    Some( editing_item ) => {
                        hind.active = editing_item.active;
                        hind.book_id = editing_item.book_id;
                    }
                    None => {
                        hind.active = true;
                        hind.book_id = book_id;
                    }
                }

                self.editing_item = Some(hind);

                return true;
            }

            AdminGameDataHindrancesMessage::SaveItemAndLeaveOpen( _unused ) => {
                log!("AdminGameDataHindrancesMessage::SaveItemAndLeaveOpen");
                let self_editing_item = self.editing_item.clone();
                let self_is_adding = self.is_adding;

                match self_editing_item {
                    Some( mut editing_item ) => {
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
                        let set_items = ctx.link().callback(AdminGameDataHindrancesMessage::SetItems);
                        let new_item_callback = ctx.link().callback(AdminGameDataHindrancesMessage::NewItem);

                        let update_hindrance_callback = ctx.link().callback(AdminGameDataHindrancesMessage::UpdateHindranceAndRefresh);

                        spawn_local (
                            async move {
                                let result = fetch_api_save_game_data_row(
                                    (api_root + "/admin/game-data/hindrances/save").to_owned(),
                                    req,

                                ).await;

                                match result {
                                    Ok( value ) => {
                                        let save_result: Result<AdminSaveReturn, Error> = JsValueSerdeExt::into_serde(&value);
                                        match save_result {
                                            Ok( save_result_data) => {
                                                match save_result_data.game_data {
                                                    Some( vec_val ) => {

                                                        if edit_item_id == 0 {
                                                            new_item_callback.emit( edit_item_book_id );
                                                        }

                                                        let mut rv: Vec<Hindrance> = Vec::new();
                                                        for mut data in vec_val.into_iter() {
                                                            data.created_by_user = None;
                                                            data.updated_by_user = None;
                                                            data.updated_by_user = None;

                                                            let item = data.to_hindrance().unwrap();

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
                                                            rv.push( item )
                                                        }
                                                        set_items.emit( rv );
                                                        let alert_def: AlertDefinition = AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some( save_result_data.message ),
                                                            ..Default::default()
                                                        };
                                                        global_vars.add_alert.emit( alert_def );

                                                    let mut new_item = Hindrance::default();
                                                    new_item.book_id = edit_item_book_id;
                                                    new_item.active = edit_item_active;
                                                    new_item.page = edit_item_book_page;
                                                    update_hindrance_callback.emit( new_item );

                                                    }

                                                    None => {
                                                        set_items.emit( Vec::new() );
                                                        let alert_def: AlertDefinition = AlertDefinition {
                                                            level: save_result_data.level,
                                                            text: Some( save_result_data.message ),
                                                            ..Default::default()
                                                        };
                                                        global_vars.add_alert.emit( alert_def );                                                    }
                                                }
                                            }
                                            Err( err ) => {
                                                let err_string: String = format!("SaveItem Serde Err(): {}", &err);
                                                // set_paging.emit( None );
                                                set_items.emit( Vec::new() );
                                                error!( &err_string  );
                                                let alert_def: AlertDefinition = AlertDefinition {
                                                    level: AlertLevel::Danger,
                                                    text: Some( format!("{:?}", err ) ),
                                                    ..Default::default()
                                                };

                                                global_vars.add_alert.emit( alert_def );

                                            }
                                        }

                                    }

                                    Err( err ) => {
                                        set_items.emit( Vec::new() );
                                        error!("get_items paging Err()", &err );
                                        let alert_def: AlertDefinition = AlertDefinition {
                                            level: AlertLevel::Danger,
                                            text: Some( format!("{:?}", err ) ),
                                            ..Default::default()
                                        };

                                        global_vars.add_alert.emit( alert_def );
                                    }
                                }
                            }
                        );

                        // self.editing_item = None;
                    }
                    None => {}
                }
            }

            AdminGameDataHindrancesMessage::DeleteItem( id ) => {
                log!("AdminGameDataHindrancesMessage::DeleteItem ", id);

                let api_root = ctx.props().global_vars.api_root.to_owned();
                let global_vars = ctx.props().global_vars.clone();
                let login_token = Some(ctx.props().global_vars.login_token.to_owned());
                let set_items = ctx.link().callback(AdminGameDataHindrancesMessage::SetItems);
                let paging_sorting_and_filter = self.paging_sorting_and_filter.clone();

                for item in self.items.clone().into_iter() {
                    if item.id == id {
                        let open_confirmation_dialog = ctx.props().global_vars.open_confirmation_dialog.clone();
                        let global_vars = global_vars.clone();
                        let set_items = set_items.clone();
                        let api_root = api_root.clone();
                        let paging_sorting_and_filter = paging_sorting_and_filter.clone();
                        let login_token = login_token.clone();
                        let dialog = ConfirmationDialogDefinition {
                            title: Some("Deletion Confirmation".to_owned()),

                            html: None,
                            text: Some( "Are you sure you would like to delete the hindrance '".to_owned() + &item.name + &"'?" ),
                            label_yes: None,
                            label_no: None,
                            callback: Callback::from( move |_clicked_yes| {

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
                                spawn_local (
                                    async move {
                                        let result = fetch_api_delete_game_data_row(
                                            (api_root + "/admin/game-data/hindrances/delete").to_owned(),
                                            req,
                                        ).await;

                                        match result {
                                            Ok( value ) => {
                                                let save_result: Result<AdminSaveReturn, Error> = JsValueSerdeExt::into_serde(&value);
                                                match save_result {
                                                    Ok( save_result_data) => {
                                                        match save_result_data.game_data {
                                                            Some( vec_val ) => {

                                                                let mut rv: Vec<Hindrance> = Vec::new();
                                                                for mut data in vec_val.into_iter() {
                                                                    data.created_by_user = None;
                                                                    data.updated_by_user = None;
                                                                    data.updated_by_user = None;

                                                                    let item = data.to_hindrance().unwrap();

                                                                    rv.push( item )
                                                                }
                                                                set_items.emit( rv );
                                                                let alert_def: AlertDefinition = AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some(save_result_data.message),
                                                                    ..Default::default()
                                                                };
                                                                global_vars.add_alert.emit( alert_def );
                                                            }

                                                            None => {
                                                                set_items.emit( Vec::new() );
                                                                // error!("get_items Err()", &err );
                                                                let alert_def: AlertDefinition = AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some(save_result_data.message),
                                                                    ..Default::default()
                                                                };
                                                                global_vars.add_alert.emit( alert_def );
                                                            }
                                                        }
                                                    }
                                                    Err( err ) => {
                                                        let err_string: String = format!("SaveItem Serde Err(): {}", &err);
                                                        // set_paging.emit( None );
                                                        set_items.emit( Vec::new() );
                                                        error!( &err_string  );
                                                        let alert_def: AlertDefinition = AlertDefinition {
                                                            level: AlertLevel::Danger,
                                                            text: Some( format!("{:?}", err ) ),
                                                            ..Default::default()
                                                        };

                                                        global_vars.add_alert.emit( alert_def );
                                                    }
                                                }

                                            }

                                            Err( err ) => {
                                                set_items.emit( Vec::new() );
                                                error!("get_items paging Err()", &err );
                                                let alert_def: AlertDefinition = AlertDefinition {
                                                    level: AlertLevel::Danger,
                                                    text: Some( format!("{:?}", err ) ),
                                                    ..Default::default()
                                                };

                                                global_vars.add_alert.emit( alert_def );
                                            }
                                        }
                                    }
                                );
                                // return false;
                            }),
                        };

                        open_confirmation_dialog.emit( dialog );
                    }
                }
            }

            AdminGameDataHindrancesMessage::DuplicateItem( id ) => {
                log!("AdminGameDataHindrancesMessage::DuplicateItem", id);

                for item in self.items.clone().into_iter() {
                    if item.id == id {

                        let open_confirmation_dialog = ctx.props().global_vars.open_confirmation_dialog.clone();

                        let api_root = ctx.props().global_vars.api_root.to_owned();
                        let login_token = Some(ctx.props().global_vars.login_token.to_owned());
                        let set_items = ctx.link().callback(AdminGameDataHindrancesMessage::SetItems);
                        let paging_sorting_and_filter = self.paging_sorting_and_filter.clone();
                        let item = item.clone();
                        let global_vars = ctx.props().global_vars.clone();
                        let item_name = item.name.clone();

                        // let editing_item_name = item.name.to_owned();
                        let dialog = ConfirmationDialogDefinition {
                            title: Some("Duplication Confirmation".to_owned()),

                            html: None,
                            text: Some( "Are you sure you would like to duplicate the hindrance '".to_owned() + &item_name + &"'?" ),
                            label_yes: None,
                            label_no: None,
                            callback: Callback::from( move |_clicked_yes| {

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

                                spawn_local (
                                    async move {

                                        let result = fetch_api_save_game_data_row(
                                            (api_root + "/admin/game-data/hindrances/save").to_owned(),
                                            req,

                                        ).await;

                                        match result {
                                            Ok( value ) => {
                                                let save_result: Result<AdminSaveReturn, Error> = JsValueSerdeExt::into_serde(&value);
                                                match save_result {
                                                    Ok( save_result_data) => {
                                                        match save_result_data.game_data {
                                                            Some( vec_val ) => {

                                                                let mut rv: Vec<Hindrance> = Vec::new();
                                                                for mut data in vec_val.into_iter() {
                                                                    data.created_by_user = None;
                                                                    data.updated_by_user = None;
                                                                    data.updated_by_user = None;

                                                                    let item = data.to_hindrance().unwrap();

                                                                    rv.push( item )
                                                                }
                                                                set_items.emit( rv );

                                                                let alert_def: AlertDefinition = AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some("Hindrance '".to_owned() + &item_name.to_owned() + &"' has been duplicated."),
                                                                    ..Default::default()
                                                                };
                                                                global_vars.add_alert.emit( alert_def );

                                                            }

                                                            None => {
                                                                set_items.emit( Vec::new() );
                                                                let alert_def: AlertDefinition = AlertDefinition {
                                                                    level: save_result_data.level,
                                                                    text: Some("No Data received!!".to_owned()),
                                                                    ..Default::default()
                                                                };
                                                                global_vars.add_alert.emit( alert_def );
                                                                // error!("get_items Err()", &err );
                                                            }
                                                        }
                                                    }
                                                    Err( err ) => {
                                                        let err_string: String = format!("SaveItem Serde Err(): {}", &err);
                                                        // set_paging.emit( None );
                                                        set_items.emit( Vec::new() );

                                                        let alert_def: AlertDefinition = AlertDefinition {
                                                            level: AlertLevel::Danger,
                                                            text: Some( format!("{:?}", err ) ),
                                                            ..Default::default()
                                                        };

                                                        global_vars.add_alert.emit( alert_def );

                                                        error!( &err_string  );
                                                    }
                                                }

                                            }

                                            Err( err ) => {
                                                set_items.emit( Vec::new() );
                                                error!("get_items paging Err()", &err );
                                                let alert_def: AlertDefinition = AlertDefinition {
                                                    level: AlertLevel::Danger,
                                                    text: Some( format!("{:?}", err ) ),
                                                    ..Default::default()
                                                };

                                                global_vars.add_alert.emit( alert_def );
                                            }
                                        }
                                    }
                                );
                                // return false;
                            }),
                        };

                        open_confirmation_dialog.emit( dialog );

                        return false;
                    }
                }

            }

            AdminGameDataHindrancesMessage::Cancel( _new_value ) => {
                log!("AdminGameDataHindrancesMessage::Cancel");
                self.editing_item = None;
            }

            AdminGameDataHindrancesMessage::UpdateHindrance( new_value ) => {
                self.editing_item = Some(new_value);
                return false;

            }
            AdminGameDataHindrancesMessage::UpdateHindranceAndRefresh( new_value ) => {
                self.editing_item = Some(new_value);
                return true;

            }

            AdminGameDataHindrancesMessage::SetItems( new_value ) => {
                self.items = new_value;
                self.loading = false;
            }

            AdminGameDataHindrancesMessage::SetPagingStats( new_value ) => {

                match new_value {
                    Some(mut nv) => {

                        match nv.book_list {
                            Some( bl) => {
                                nv.book_list = Some(bl);
                            }
                            None => {
                                match &self.paging_data {
                                    Some( pg ) => {
                                        nv.book_list = pg.book_list.clone();
                                    }
                                    None => {}
                                }
                            }
                        }
                        self.paging_data = Some(nv.clone());

                    }
                    None => {
                        self.paging_data = None;
                    }
                }

                self.loading = false;

            }

            AdminGameDataHindrancesMessage::SetFetchAdminParams( new_value ) => {
                let mut paging_sorting_and_filter = new_value.clone();
                self.paging_sorting_and_filter = new_value.clone();

                let global_vars = ctx.props().global_vars.clone();

                let login_token = global_vars.login_token.clone();
                let set_items = ctx.link().callback(AdminGameDataHindrancesMessage::SetItems);
                let set_paging = ctx.link().callback(AdminGameDataHindrancesMessage::SetPagingStats);

                set_local_storage_u32("admin_page_count", paging_sorting_and_filter.number_per_page);

                paging_sorting_and_filter.login_token = Some(login_token);

                paging_sorting_and_filter.needs_book_list = true;
                match &self.paging_data {
                    Some( paging ) => {
                        match &paging.book_list {
                            Some( book_list ) => {
                                if book_list.len() > 0 {
                                    paging_sorting_and_filter.needs_book_list = false;
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
                spawn_local (
                    async move {
                        _get_data(
                            global_vars,
                            paging_sorting_and_filter,
                            set_items,
                            set_paging,
                        ).await;

                    }
                );

            }

        }
        true
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {

        let callback_fetch_admin_params = ctx.link().callback( AdminGameDataHindrancesMessage::SetFetchAdminParams ).clone();
        let callback_fetch_admin_params_2 = ctx.link().callback( AdminGameDataHindrancesMessage::SetFetchAdminParams ).clone();

        let mut non_filtered_count: u32 = 0;
        let mut filtered_count: u32= 0;

        match &self.paging_data {
            Some( paging_data ) => {
                non_filtered_count = paging_data.non_filtered_count;
                filtered_count = paging_data.filtered_count;

            }
            None => {

            }
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
            Some( pg ) => {
                book_list = pg.book_list.clone();
            }
            None => {}
        }

        let mut edit_modal = html!{<></>};
        match &self.editing_item {
            Some( editing_item ) => {
                let mut editing_title = Some("Viewing Hindrance".to_owned());

                let mut save_callback:Option<Callback<bool>> = None;
                let mut add_callback: Option<Callback<bool>>= None;
                let mut save_as_new_callback: Option<Callback<bool>>= None;
                let mut save_and_leave_open_callback: Option<Callback<bool>>= None;

                let mut read_only = true;

                if self.is_adding {
                    editing_title = Some("Adding Hindrance".to_owned());
                    add_callback = Some(ctx.link().callback(AdminGameDataHindrancesMessage::SaveItem).clone());
                    save_and_leave_open_callback = Some(ctx.link().callback(AdminGameDataHindrancesMessage::SaveItemAndLeaveOpen).clone());
                    read_only = false;
                }

                if self.is_editing {
                    editing_title = Some("Editing Hindrance".to_owned());
                    save_callback = Some(ctx.link().callback(AdminGameDataHindrancesMessage::SaveItem).clone());
                    save_as_new_callback = Some(ctx.link().callback(AdminGameDataHindrancesMessage::SaveItem).clone());
                    save_and_leave_open_callback = Some(ctx.link().callback(AdminGameDataHindrancesMessage::SaveItemAndLeaveOpen).clone());
                    read_only = false;
                }

                let mut book_list: Vec<Book> = Vec::new();

                match self.paging_data.clone() {
                    Some( paging_data ) => {
                        book_list = paging_data.book_list.unwrap_or( Vec::new() );
                    }
                    None => {}
                }

                edit_modal = html!{
                <StandardModal
                    xl={true}
                    title={editing_title}
                    close_cancel_callback={Some(ctx.link().callback(AdminGameDataHindrancesMessage::Cancel).clone())}
                    save_callback={save_callback}
                    add_callback={add_callback}
                    save_as_new_callback={save_as_new_callback}
                    save_and_leave_open_callback={save_and_leave_open_callback}
                >
                    <EditHindrance
                        for_admin={true}
                        global_vars={global_vars.clone()}
                        readonly={read_only}
                        edit_item={editing_item.clone()}
                        book_list={book_list}
                        on_changed_callback={ctx.link().callback(AdminGameDataHindrancesMessage::UpdateHindrance).clone()}
                    />

                </StandardModal>
                };
            }
            None => {}
        }

        let add_item = ctx.link().callback(AdminGameDataHindrancesMessage::AddItemDialog);

        html! {
        <UIPage
            global_vars={global_vars.clone()}
            page_title="Admin Hindrances"

            modal={Some(edit_modal)}
        >

        <TertiaryLinksMenu
            server_side_renderer={global_vars.server_side_renderer}
            menu_items={ctx.props().sub_menu_items.clone()}

            current_tag={"hindrances".to_owned()}
        />
        <div class="pull-right">
            <AdminTableFilterSearch
                callback_fetch_admin_params={callback_fetch_admin_params_2}
                paging_sorting_and_filter={self.paging_sorting_and_filter.clone()}
                stats={self.paging_data.clone()}
                global_vars={global_vars.clone()}
                show_no_select={true}
            />
        </div>
                <h2><i class="fa fa-items" /><Nbsp />{"Admin Hindrances"}</h2>

                    <table class="admin-table">
                    <thead>
                        <tr>

                            if show_book_column {
                                <th class="min-width">
                                    {"Book"}
                                </th>
                            }
                            <th>
                                {"Active"}
                            </th>
                            <th>
                                {"Name"}
                            </th>
                            // <th>
                            //     {"Email"}
                            // </th>
                            <th>
                                {"Updated"}
                            </th>
                            <th class="min-width">
                            if global_vars.current_user.admin_can_write_book(
                                &book_list,
                                current_book_id,
                            ) {

                                <button
                                    type="button"
                                    class="btn btn-xs full-width no-margins btn-success"
                                    onclick={move |e: MouseEvent| {
                                        let add_item = add_item.clone();
                                        e.prevent_default();
                                        add_item.emit(true);
                                    }}
                                >
                                    <i class="fa fa-plus" /><Nbsp />{"Add"}
                                </button>
                            }
                            </th>
                        </tr>
                    </thead>

                        if self.loading {
                            <tbody>
                                <tr>
                                    <td colspan="5" class="text-center">
                                        <br />
                                        {"Loading..."}<br />
                                        <br />

                                    </td>
                                </tr>
                            </tbody>
                        } else {
                            if self.items.len() == 0 {
                                if non_filtered_count != filtered_count {
                                    <tbody>
                                        <tr>
                                            <td colspan="5" class="text-center">
                                                <br />
                                                {"There are no items with this filter result. Please revise your filter term."}<br />
                                                <br />

                                            </td>
                                        </tr>
                                    </tbody>
                                } else {
                                    <tbody>
                                        <tr>
                                            <td colspan="5" class="text-center">
                                                <br />
                                                {"There are no items."}<br />
                                                <br />
                                            </td>
                                        </tr>
                                    </tbody>
                                }
                            } else {
                            {self.items.clone().into_iter().map( move |row| {
                                let mut callback_edit_item: Option<Callback<u32>> = None;
                                let mut callback_view_item: Option<Callback<u32>> = None;
                                let mut callback_delete_item: Option<Callback<u32>> = None;
                                let mut callback_duplicate_item: Option<Callback<u32>> = None;

                                let row_name = &row.name.to_owned();

                                if global_vars.current_user.admin_can_read_item (
                                    &book_list,
                                    row.created_by,
                                    row.book_id,
                                ) {
                                    callback_view_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::ViewItem));
                                }
                                if global_vars.current_user.admin_can_write_item (
                                    &book_list,
                                    row.created_by,
                                    row.book_id,
                                ) {
                                    callback_edit_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::EditItemDialog));
                                    callback_duplicate_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::DuplicateItem));
                                }
                                if global_vars.current_user.admin_can_delete_item (
                                    &book_list,
                                    row.created_by,
                                    row.book_id,
                                ) {
                                    callback_delete_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::DeleteItem));
                                }
                                let row_summary = row.get_summary();
                                html!{
                                    <tbody>
                                    <tr>

                                    if show_book_column {
                                        <AdminTableFieldText
                                            value={row.book_short_name.unwrap_or("???".to_owned())}
                                        />
                                    }
                                    <AdminTableFieldActive
                                        active={row.active}
                                        rowspan={2}
                                        no_select={row.no_select}
                                        td_class="larger-icon min-width text-center"
                                    />
                                    <AdminTableFieldText
                                        value={row.name}
                                    />

                                    // <AdminTableFieldText
                                    //     value={row.email}
                                    // />

                                    // <AdminTableFieldText
                                    //     value={row.username.to_owned()}
                                    // />
                                    <td class="min-width no-wrap">
                                        <AdminTableOwnershipBadge
                                            current_user={ctx.props().global_vars.current_user.clone()}

                                            created_by={row.created_by_obj}
                                            created_on={row.created_on}

                                            updated_by={row.updated_by_obj}
                                            updated_on={row.updated_on}

                                            deleted_by={row.deleted_by_obj}
                                            deleted_on={row.deleted_on}
                                        />
                                    </td>

                                    <td rowspan={2}>
                                        <EditViewDeleteButtons
                                            id={row.id}
                                            name={row_name.to_owned()}

                                            view_callback={callback_view_item}
                                            edit_callback={callback_edit_item}
                                            delete_callback={callback_delete_item}
                                            duplicate_callback={callback_duplicate_item}

                                            // callback_add_item
                                            // callback_delete_item
                                        />
                                    </td>

                                </tr>
                                <tr>
                                    <td colspan={2} class="small-text">
                                        {row_summary}
                                    </td>
                                </tr>
                                </tbody>
                                }
                            }).collect::<Html>()}
                            }
                        }
                    <tfoot>
                        <tr>
                            <th colspan="5">
                                <AdminTablePaging
                                    callback_fetch_admin_params={callback_fetch_admin_params}
                                    paging_sorting_and_filter={self.paging_sorting_and_filter.clone()}
                                    stats={self.paging_data.clone()}
                                />
                            </th>
                        </tr>
                    </tfoot>
                </table>
            </UIPage>
        }
    }

}

async fn _get_data(
    global_vars: GlobalVars,
    paging_sorting_and_filter: FetchAdminParameters,
    set_items: Callback<Vec<Hindrance>>,
    set_paging: Callback<Option<AdminPagingStatistics>>,
) {
    let api_root = global_vars.api_root.clone();

    let result = fetch_admin_api(
        (api_root.to_owned() + "/admin/game-data/hindrances/get").to_owned(),
        paging_sorting_and_filter.clone(),
    ).await;

    match result {
        Ok( value ) => {
            let vec_val_result: Result<Vec<GameDataRow>, Error> = JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok( vec_val ) => {

                    let mut rv: Vec<Hindrance> = Vec::new();
                    for data in vec_val.into_iter() {
                        let item = data.to_hindrance().unwrap();
                        // log!( format!("item {} {}", &item.name, mem::size_of_val(&item) ) );
                        rv.push( item );
                    }
                    // log!( format!("rv {}", mem::size_of_val(&rv) ) );
                    set_items.emit( rv );
                }
                Err( err ) => {
                    let err_string: String = format!("get_items Serde Err(): {}", &err);
                    set_items.emit( Vec::new() );
                    error!( &err_string );
                }
            }

        }
        Err( err ) => {
            set_items.emit( Vec::new() );
            error!("get_items Err()", &err );
        }
    }

    let result = fetch_admin_api(
        (api_root + "/admin/game-data/hindrances/paging").to_owned(),
        paging_sorting_and_filter.clone(),

    ).await;

    match result {
        Ok( value ) => {
            // let vec_val_result = value.into_serde::< Vec<GameDataRow> >();
            let vec_val_result: Result<AdminPagingStatistics, Error> = JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok( vec_val ) => {
                    // log!( format!("vec_val {}", mem::size_of_val(&vec_val) ) );
                    // log!( format!("vec_val.book_list {}", mem::size_of_val(&vec_val.book_list) ) );
                    set_paging.emit( Some(vec_val) );
                }
                Err( err ) => {
                    let err_string: String = format!("get_items paging Serde Err(): {}", &err);
                    set_paging.emit( None );
                    error!( &err_string  );
                }
            }

        }
        Err( err ) => {
            set_paging.emit( None );
            error!("get_items paging Err()", &err );
        }
    }
}

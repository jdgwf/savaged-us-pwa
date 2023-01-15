use crate::components::admin::admin_filter_search::AdminTableFilterSearch;
use crate::components::admin::admin_table_field::bool::AdminTableFieldBool;
use crate::components::admin::admin_table_ownership_badge::AdminTableOwnershipBadge;
use crate::components::admin::admin_table_paging::AdminTablePaging;
use crate::components::admin::edit_view_delete_buttons::EditViewDeleteButtons;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::components::edit_forms::hindrance::EditHindrance;
use crate::components::standard_modal::StandardModal;
use crate::components::ui_page::UIPage;
use crate::libs::fetch_api::fetch_api;
use crate::libs::global_vars::GlobalVars;
use crate::{components::admin::admin_table_field::text::AdminTableFieldText, libs::fetch_api::fetch_admin_api};
use gloo_console::{ error, log };
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::book::Book;
use savaged_libs::admin_libs::AdminPagingStatistics;
use savaged_libs::game_data::GameData;
use savaged_libs::player_character::hindrance::Hindrance;
use savaged_libs::{ admin_libs::FetchAdminParameters, admin_libs::new_fetch_admin_params};
use serde_json::Error;
use standard_components::libs::local_storage_shortcuts::{set_local_storage_string, get_local_storage_u32, set_local_storage_u32};
use standard_components::ui::nbsp::Nbsp;
use standard_components::ui::standard_form_save_buttons::StandardFormSaveButtons;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminGameDataHindrancesProps {
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum AdminGameDataHindrancesMessage {
    SetItems(Vec<Hindrance>),
    SetPagingStats(Option<AdminPagingStatistics>),
    SetFetchAdminParams(FetchAdminParameters),
    UpdateHindrance(Hindrance),
    ViewItem( u32 ),
    EditItem( u32 ),
    DeleteItem( u32 ),
    DuplicateItem( u32 ),
    Cancel(bool),
    AddItem(bool),
}
pub struct AdminGameDataHindrances {
    global_vars: GlobalVars,
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
            global_vars: ctx.props().global_vars.clone(),
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

            AdminGameDataHindrancesMessage::EditItem( id ) => {
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

            AdminGameDataHindrancesMessage::AddItem( _nv ) => {
                log!("AdminGameDataHindrancesMessage::AddItem");
            }

            AdminGameDataHindrancesMessage::DeleteItem( id ) => {
                log!("AdminGameDataHindrancesMessage::DeleteItem ", id);
            }

            AdminGameDataHindrancesMessage::DuplicateItem( id ) => {
                log!("AdminGameDataHindrancesMessage::DuplicateItem", id);
            }

            AdminGameDataHindrancesMessage::Cancel( new_value ) => {
                log!("AdminGameDataHindrancesMessage::Cancel");
                self.editing_item = None;
            }

            AdminGameDataHindrancesMessage::UpdateHindrance( new_value ) => {
                self.editing_item = Some(new_value);

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

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &AdminGameDataHindrancesProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();

        self.global_vars.current_sub_menu = "admin-items".to_owned();

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
        let mut current_book_id: u32 = 0;
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

        if self.paging_sorting_and_filter.filter_book > 0 {
            show_book_column = false;
            current_book_id = self.paging_sorting_and_filter.filter_book;
        } else {
            current_book_id = 0;
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

                let mut save_callback:Option<Callback<bool>> = Some( Callback::noop() );
                let mut add_callback: Option<Callback<bool>>= None;
                let mut save_as_new_callback: Option<Callback<bool>>= None;

                let mut read_only = true;
                if self.is_adding {
                    editing_title = Some("Adding Hindrance".to_owned());
                    read_only = false;
                }
                if self.is_editing {
                    editing_title = Some("Editing Hindrance".to_owned());
                    read_only = false;
                }
                edit_modal = html!{
                <StandardModal
                    xl={true}
                    title={editing_title}
                >
                    <EditHindrance
                        global_vars={ctx.props().global_vars.clone()}
                        readonly={read_only}
                        edit_item={editing_item.clone()}

                        on_changed_callback={ctx.link().callback(AdminGameDataHindrancesMessage::UpdateHindrance).clone()}
                    />

                    <StandardFormSaveButtons
                        close_cancel_callback={ctx.link().callback(AdminGameDataHindrancesMessage::Cancel).clone()}
                        save_callback={save_callback}
                        add_callback={add_callback}
                        save_as_new_callback={save_as_new_callback}
                    />
                </StandardModal>
                };
            }
            None => {}
        }

        let add_item = ctx.link().callback(AdminGameDataHindrancesMessage::AddItem);

        html! {
        <UIPage
            global_vars={global_vars.clone()}
            page_title="Admin Hindrances"
            submenu_tag={"admin".to_owned()}
            modal={Some(edit_modal)}
        >

        <div class="pull-right">
            <AdminTableFilterSearch
                callback_fetch_admin_params={callback_fetch_admin_params_2}
                paging_sorting_and_filter={self.paging_sorting_and_filter.clone()}
                stats={self.paging_data.clone()}
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
                                &current_book_id,
                            ) {

                                <button
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
                    <tbody>
                        if self.loading {
                            <tr>
                            <td colspan="5" class="text-center">
                                <br />
                                {"Loading..."}<br />
                                <br />

                            </td>
                        </tr>
                        } else {
                            if self.items.len() == 0 {
                                if non_filtered_count != filtered_count {
                                    <tr>
                                        <td colspan="5" class="text-center">
                                            <br />
                                            {"There are no items with this filter result. Please revise your filter term."}<br />
                                            <br />

                                        </td>
                                    </tr>

                                } else {
                                    <tr>
                                        <td colspan="5" class="text-center">
                                            <br />
                                            {"There are no items."}<br />
                                            <br />
                                        </td>
                                    </tr>
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
                                    &row.created_by,
                                    &row.book_id,
                                ) {
                                    callback_view_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::ViewItem));
                                }
                                if global_vars.current_user.admin_can_write_item (
                                    &book_list,
                                    &row.created_by,
                                    &row.book_id,
                                ) {
                                    callback_edit_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::EditItem));
                                    callback_duplicate_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::DuplicateItem));
                                }
                                if global_vars.current_user.admin_can_delete_item (
                                    &book_list,
                                    &row.created_by,
                                    &row.book_id,
                                ) {
                                    callback_delete_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::DeleteItem));
                                }
                                html!{<tr>

                                    if show_book_column {
                                        <AdminTableFieldText
                                            value={row.book_short_name.unwrap_or("???".to_owned())}
                                        />
                                    }
                                    <AdminTableFieldBool
                                        value={row.active}
                                        td_class="min-width text-center"
                                    />
                                    <AdminTableFieldText
                                        value={row.name}
                                    />

                                    // <AdminTableFieldText
                                    //     value={row.email}
                                    // />

                                    // <AdminTableFieldText
                                    //     value={row.username}
                                    // />
                                    <td class="min-width no-wrap">
                                        <AdminTableOwnershipBadge
                                            global_vars={self.global_vars.clone()}

                                            created_by={row.created_by_obj}
                                            created_on={row.created_on}

                                            updated_by={row.updated_by_obj}
                                            updated_on={row.updated_on}

                                            deleted_by={row.deleted_by_obj}
                                            deleted_on={row.deleted_on}
                                        />
                                    </td>

                                    <td>
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
                                }
                            }).collect::<Html>()}
                            }
                        }
                    </tbody>
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
    let mut paging_sorting_and_filter = paging_sorting_and_filter.clone();

    let result = fetch_admin_api(
        (api_root.to_owned() + "/admin/game-data/hindrances/get").to_owned(),
        paging_sorting_and_filter.clone(),
    ).await;

    match result {
        Ok( value ) => {
            // let vec_val_result = value.into_serde::< Vec<GameData> >();
            let vec_val_result: Result<Vec<GameData>, Error> = JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok( vec_val ) => {

                    let mut rv: Vec<Hindrance> = Vec::new();
                    for data in vec_val.into_iter() {
                        // log!("data", format!("{:?}", data) );
                        let hind = data.to_hindrance().unwrap();
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
                        rv.push( hind )
                    }
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
            // let vec_val_result = value.into_serde::< Vec<GameData> >();
            let vec_val_result: Result<AdminPagingStatistics, Error> = JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok( vec_val ) => {
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

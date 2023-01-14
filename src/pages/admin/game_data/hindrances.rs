use std::vec;

use savaged_libs::admin_libs::AdminPagingStatistics;
use savaged_libs::game_data::GameData;
use savaged_libs::player_character::hindrance::Hindrance;
use savaged_libs::{ admin_libs::FetchAdminParameters, admin_libs::new_fetch_admin_params};
use standard_components::libs::local_storage_shortcuts::{set_local_storage_string, get_local_storage_u32, set_local_storage_u32};
use yew_router::prelude::*;
use yew::prelude::*;
use crate::components::admin::admin_filter_search::AdminTableFilterSearch;
use crate::components::admin::admin_table_paging::AdminTablePaging;
use crate::components::admin::admin_table_ownership_badge::AdminTableOwnershipBadge;
use crate::{components::admin::admin_table_field::text::AdminTableFieldText, libs::fetch_api::fetch_admin_api};
use crate::components::admin::admin_table_field::bool::AdminTableFieldBool;
use crate::components::admin::edit_view_delete_buttons::EditViewDeleteButtons;
use crate::components::ui_page::UIPage;
use crate::components::standard_modal::StandardModal;
use standard_components::ui::standard_form_save_buttons::StandardFormSaveButtons;
use crate::components::edit_forms::hindrance::EditHindrance;
use crate::libs::global_vars::GlobalVars;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use gloo_utils::format::JsValueSerdeExt;
use serde_json::Error;
use standard_components::ui::nbsp::Nbsp;
use wasm_bindgen_futures::spawn_local;
use crate::libs::fetch_api::fetch_api;
use gloo_console::{ error, log };


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
    Cancel(bool),
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
                        self.is_editing = false;
                        self.is_adding = false;
                        return true;
                    }
                }
                return false;
            }

            AdminGameDataHindrancesMessage::Cancel( new_value ) => {
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
                self.paging_data = new_value;
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
        }



        let mut edit_modal = html!{<></>};
        match &self.editing_item {
            Some( editing_item ) => {
                let mut editing_title = "Viewing Hindrance";

                let mut save_callback:Option<Callback<bool>> = Some( Callback::noop() );
                let mut add_callback: Option<Callback<bool>>= None;
                let mut save_as_new_callback: Option<Callback<bool>>= None;

                if self.is_adding {
                    editing_title = "Adding Hindrance";
                }
                if self.is_editing {
                    editing_title = "Editing Hindrance";
                }
                edit_modal = html!{
                <StandardModal
                    xl={true}
                >
                    <EditHindrance
                        global_vars={ctx.props().global_vars.clone()}
                        // edit_save={save.clone()}
                        readonly={false}
                        edit_item={editing_item.clone()}
                        form_title={editing_title}
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
                <h2><i class="fa fa-items" /><Nbsp />{"Admin Hindrances List TODO"}</h2>

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
                                {"Last Updated"}
                            </th>
                            <th class="min-width"></th>
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
                                callback_edit_item = Some(ctx.link().callback(AdminGameDataHindrancesMessage::EditItem));
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

                                            view_callback={Some(ctx.link().callback(AdminGameDataHindrancesMessage::ViewItem))}
                                            edit_callback={callback_edit_item}

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
    let paging_sorting_and_filter = paging_sorting_and_filter.clone();
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

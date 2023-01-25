use crate::components::admin::admin_filter_search::AdminTableFilterSearch;
use crate::components::admin::admin_table_field::bool::AdminTableFieldBool;
use crate::components::admin::admin_table_paging::AdminTablePaging;
use crate::components::admin::edit_view_delete_buttons::EditViewDeleteButtons;
use crate::components::tertiary_links_menu::{TertiaryLinksMenuItem, TertiaryLinksMenu};
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::{
    components::admin::admin_table_field::text::AdminTableFieldText,
    libs::fetch_api::fetch_admin_api,
};
use gloo_console::error;
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::admin_libs::AdminPagingStatistics;
use savaged_libs::{
    admin_libs::new_fetch_admin_params, admin_libs::FetchAdminParameters, user::User,
};
use serde_json::Error;
use standard_components::libs::local_storage_shortcuts::{
    get_local_storage_u32, set_local_storage_u32,
};
use standard_components::ui::nbsp::Nbsp;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminUsersListProps {
    pub global_vars: GlobalVars,
    pub sub_menu_items: Vec<TertiaryLinksMenuItem>,
}

pub enum AdminUsersListMessage {
    SetUsers(Vec<User>),
    SetPagingStats(Option<AdminPagingStatistics>),
    SetFetchAdminParams(FetchAdminParameters),
}
pub struct AdminUsersList {
    users: Vec<User>,
    paging_data: Option<AdminPagingStatistics>,
    paging_sorting_and_filter: FetchAdminParameters,
    loading: bool,
}

impl Component for AdminUsersList {
    type Message = AdminUsersListMessage;
    type Properties = AdminUsersListProps;

    fn create(ctx: &Context<Self>) -> Self {
        let global_vars = ctx.props().global_vars.clone();

        let login_token = global_vars.login_token.clone();
        let set_users = ctx.link().callback(AdminUsersListMessage::SetUsers);
        let set_paging = ctx.link().callback(AdminUsersListMessage::SetPagingStats);

        let mut paging_sorting_and_filter = new_fetch_admin_params();

        paging_sorting_and_filter.login_token = Some(login_token);
        paging_sorting_and_filter.number_per_page = get_local_storage_u32(
            "admin_page_count",
            paging_sorting_and_filter.number_per_page,
        );
        paging_sorting_and_filter.filter_book =
            get_local_storage_u32("admin_selected_book", paging_sorting_and_filter.filter_book);
        let paging = paging_sorting_and_filter.clone();
        spawn_local(async move {
            _get_data(
                global_vars,
                paging_sorting_and_filter,
                set_users,
                set_paging,
            )
            .await;
        });

        AdminUsersList {
            paging_sorting_and_filter: paging,
            users: Vec::new(),
            paging_data: None,
            loading: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: AdminUsersListMessage) -> bool {
        match msg {
            AdminUsersListMessage::SetUsers(new_value) => {
                self.users = new_value;
                self.loading = false;
            }

            AdminUsersListMessage::SetPagingStats(new_value) => {
                self.paging_data = new_value;
                self.loading = false;
            }

            AdminUsersListMessage::SetFetchAdminParams(new_value) => {
                let mut paging_sorting_and_filter = new_value.clone();
                self.paging_sorting_and_filter = new_value.clone();

                let global_vars = ctx.props().global_vars.clone();

                let login_token = global_vars.login_token.clone();
                let set_users = ctx.link().callback(AdminUsersListMessage::SetUsers);
                let set_paging = ctx.link().callback(AdminUsersListMessage::SetPagingStats);

                set_local_storage_u32(
                    "admin_page_count",
                    paging_sorting_and_filter.number_per_page,
                );

                paging_sorting_and_filter.login_token = Some(login_token);
                // let paging = paging_sorting_and_filter.clone();

                spawn_local(async move {
                    _get_data(
                        global_vars,
                        paging_sorting_and_filter,
                        set_users,
                        set_paging,
                    )
                    .await;
                });
            } // AdminUsersListMessage::ChangeFolder( folder_name ) => {
              //     // log!("ChangeFolder", folder);
              //     set_local_storage_string( "saves_folder", folder_name);
              // }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback_fetch_admin_params = ctx
            .link()
            .callback(AdminUsersListMessage::SetFetchAdminParams)
            .clone();
        let callback_fetch_admin_params_2 = ctx
            .link()
            .callback(AdminUsersListMessage::SetFetchAdminParams)
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
        global_vars.current_sub_menu = "admin-users".to_owned();

        html! {
        <UIPage
            global_vars={global_vars.clone()}
            page_title="Admin Users"

        >
        <TertiaryLinksMenu
            server_side_renderer={global_vars.server_side_renderer}
            menu_items={ctx.props().sub_menu_items.clone()}

            current_tag={"users-list".to_owned()}
        />
        <div class="pull-right">
            <AdminTableFilterSearch
                callback_fetch_admin_params={callback_fetch_admin_params_2}
                paging_sorting_and_filter={self.paging_sorting_and_filter.clone()}
                stats={self.paging_data.clone()}
                global_vars={global_vars.clone()}
            />
        </div>
                <h2><i class="fa fa-users" /><Nbsp />{"Admin Users"}</h2>

                    <table class="admin-table">
                    <thead>
                        <tr>
                            <th>
                                {"Active"}
                            </th>
                            <th>
                                {"Name"}
                            </th>
                            <th>
                                {"Email"}
                            </th>
                            <th>
                                {"Username"}
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
                            if self.users.len() == 0 {
                                if non_filtered_count != filtered_count {
                                    <tr>
                                        <td colspan="5" class="text-center">
                                            <br />
                                            {"There are no users with this filter result. Please revise your filter term."}<br />
                                            <br />

                                        </td>
                                    </tr>

                                } else {
                                    <tr>
                                        <td colspan="5" class="text-center">
                                            <br />
                                            {"There are no users."}<br />
                                            <br />
                                        </td>
                                    </tr>
                                }
                            } else {
                            {self.users.clone().into_iter().map( move |row| {
                                let row_name = &row.get_admin_name().to_owned();
                                html!{<tr>

                                    <AdminTableFieldBool
                                        value={row.activated}
                                        td_class="min-width text-center"
                                    />

                                    <AdminTableFieldText
                                        value={row.get_admin_name()}
                                    />

                                    <AdminTableFieldText
                                        value={row.email}
                                    />

                                    <AdminTableFieldText
                                        value={row.username}
                                    />

                                    <td>
                                        <EditViewDeleteButtons
                                            id={row.id}
                                            name={row_name.to_owned()}
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
    set_users: Callback<Vec<User>>,
    set_paging: Callback<Option<AdminPagingStatistics>>,
) {
    let api_root = global_vars.api_root.clone();
    let paging_sorting_and_filter = paging_sorting_and_filter.clone();
    let result = fetch_admin_api(
        (api_root.to_owned() + "/admin/users/get").to_owned(),
        paging_sorting_and_filter.clone(),
    )
    .await;

    match result {
        Ok(value) => {
            // let vec_val_result = value.into_serde::< Vec<User> >();
            let vec_val_result: Result<Vec<User>, Error> = JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    set_users.emit(vec_val.clone());
                }
                Err(err) => {
                    let err_string: String = format!("get_users Serde Err(): {}", &err);
                    set_users.emit(Vec::new());
                    error!(&err_string);
                }
            }
        }
        Err(err) => {
            set_users.emit(Vec::new());
            error!("get_users Err()", &err);
        }
    }

    let result = fetch_admin_api(
        (api_root + "/admin/users/paging").to_owned(),
        paging_sorting_and_filter.clone(),
    )
    .await;

    match result {
        Ok(value) => {
            // let vec_val_result = value.into_serde::< Vec<User> >();
            let vec_val_result: Result<AdminPagingStatistics, Error> =
                JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    set_paging.emit(Some(vec_val));
                }
                Err(err) => {
                    let err_string: String = format!("get_users paging Serde Err(): {}", &err);
                    set_paging.emit(None);
                    error!(&err_string);
                }
            }
        }
        Err(err) => {
            set_paging.emit(None);
            error!("get_users paging Err()", &err);
        }
    }
}

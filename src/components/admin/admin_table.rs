use savaged_libs::{user::User, admin_libs::{FetchAdminParameters, AdminPagingStatistics}, book::Book, row_edit_data::RowEditData};
use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

use crate::components::admin::{admin_table_paging::AdminTablePaging, admin_table_field::active::AdminTableFieldActive, admin_table_field::text::AdminTableFieldText, admin_table_field::bool::AdminTableFieldBool, admin_table_ownership_badge::AdminTableOwnershipBadge, edit_view_delete_buttons::EditViewDeleteButtons};

#[derive(Properties, PartialEq)]
pub struct AdminTableProps {
    #[prop_or_default]
    pub title: String,

    pub current_user: User,
    pub show_no_select: bool,
    pub paging_sorting_and_filter: FetchAdminParameters,
    pub callback_fetch_admin_params: Callback<FetchAdminParameters>,
    #[prop_or_default]
    pub stats: Option<AdminPagingStatistics>,

    pub add_item: Callback<bool>,
    pub paging_data: Option<AdminPagingStatistics>,

    pub current_book_id: u32,
    pub book_list: Option<Vec<Book>>,
    pub show_book_column: bool,
    pub loading: bool,

    pub edit_callback: Callback<u32>,
    pub duplicate_callback: Callback<u32>,
    pub delete_callback: Callback<u32>,
    pub view_callback: Callback<u32>,

    pub non_filtered_count: u32,
    pub filtered_count: u32,

    pub items: Vec<dyn RowEditData>,
}

#[function_component(AdminTable)]
pub fn admin_table(props: &AdminTableProps) -> Html {
    return html!{
        <>
        <h2><i class="fa fa-items" /><Nbsp />{"Admin Armor"}</h2>

        <table class="admin-table">
        <thead>
            <tr>

                if props.show_book_column {
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
                if props.current_user.admin_can_write_book(
                    &props.book_list,
                    props.current_book_id,
                ) {

                    <button
                        type="button"
                        class="btn btn-xs full-width no-margins btn-success"
                        onclick={move |e: MouseEvent| {
                            let add_item = props.add_item.clone();
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

            if props.loading {
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
                if props.items.len() == 0 {
                    if props.non_filtered_count != props.filtered_count {
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
                {props.items.clone().into_iter().map( move |row| {
                    let mut callback_edit_item: Option<Callback<u32>> = None;
                    let mut callback_view_item: Option<Callback<u32>> = None;
                    let mut callback_delete_item: Option<Callback<u32>> = None;
                    let mut callback_duplicate_item: Option<Callback<u32>> = None;

                    let row_name = &row.name.to_owned();

                    if props.current_user.admin_can_read_item (
                        & props.book_list,
                        row.created_by,
                        row.book_id,
                    ) {
                        callback_view_item = Some(props.view_callback);
                    }
                    if  props.current_user.admin_can_write_item (
                        & props.book_list,
                        row.created_by,
                        row.book_id,
                    ) {
                        callback_edit_item = Some(props.edit_callback);
                        callback_duplicate_item = Some(props.duplicate_callback);
                    }
                    if  props.current_user.admin_can_delete_item (
                        & props.book_list,
                        row.created_by,
                        row.book_id,
                    ) {
                        callback_delete_item = Some(props.delete_callback);
                    }

                    let row_summary = row.basic_info();
                    html!{
                        <tbody>
                        <tr>

                        if props.show_book_column {
                            <AdminTableFieldText
                                rowspan={2}
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
                                current_user={props.current_user.clone()}

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

                                view_callback={props.edit_callback}
                                edit_callback={props.duplicate_callback}
                                delete_callback={props.delete_callback}
                                duplicate_callback={props.view_callback}

                                // callback_add_item
                                // callback_delete_item
                            />
                        </td>

                    </tr>
                    <tr>
                        <td colspan="2" class="small-text">
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
                        callback_fetch_admin_params={props.callback_fetch_admin_params}
                        paging_sorting_and_filter={props.paging_sorting_and_filter.clone()}
                        stats={props.paging_data.clone()}
                    />
                </th>
            </tr>
        </tfoot>
    </table>
    </>
    }
}
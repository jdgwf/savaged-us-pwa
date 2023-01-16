use savaged_libs::{admin_libs::{AdminPagingStatistics, FetchAdminParameters}, book::Book};
use standard_components::{ui::{input_text::InputText}, libs::local_storage_shortcuts::set_local_storage_u32};
use crate::{components::admin::book_select::BookSelect, libs::global_vars::GlobalVars};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFilterSearchProps {

    pub global_vars: GlobalVars,
    pub paging_sorting_and_filter: FetchAdminParameters,
    pub callback_fetch_admin_params: Callback<FetchAdminParameters>,
    #[prop_or_default]
    pub stats: Option<AdminPagingStatistics>,
}

#[function_component(AdminTableFilterSearch)]
pub fn edit_view_delete_buttons(
    props: &AdminTableFilterSearchProps,
) -> Html {

    let callback_fetch_admin_params_1 = props.callback_fetch_admin_params.clone();
    let callback_fetch_admin_params_2= props.callback_fetch_admin_params.clone();
    let paging_sorting_and_filter_1 = props.paging_sorting_and_filter.clone();
    let paging_sorting_and_filter_2 = props.paging_sorting_and_filter.clone();
    let search_value_opt = props.paging_sorting_and_filter.filter.clone();
    let search_value = search_value_opt.unwrap_or("".to_owned());

    let callback_text_new_changed = Callback::from(
        move |new_value: String | {

            let mut nv = paging_sorting_and_filter_1.clone();

            nv.filter = Some(new_value);
            nv.current_page = 0;

            callback_fetch_admin_params_1.emit(nv)

        }
    );

    let mut book_list: Vec<Book> = Vec::new();

    match &props.stats {
        Some( stats ) => {
            match &stats.book_list {
                Some( bl ) => {
                    book_list = bl.clone();
                }
                None => {}
            }
            // book_list = Some(stats.book_list.clone());
        }
        None => {}
    }

    let callback_set_filter_book = Callback::from(
        move | new_value: u32 | {


            let mut nv = paging_sorting_and_filter_2.clone();

            nv.filter_book = new_value;
            nv.current_page = 0;

            set_local_storage_u32("admin_selected_book", nv.filter_book);

            callback_fetch_admin_params_2.emit(nv)

        }
    );

    let global_vars = props.global_vars.clone();

    return html!{

        <div class="admin-filter">
            if book_list.len() > 0 {
                <BookSelect
                    global_vars={global_vars}
                    onchange={callback_set_filter_book}
                    value={props.paging_sorting_and_filter.filter_book}
                    book_list={book_list}
                />
            }
            <InputText
                placeholder="Filter Results"
                input_type="search"
                onchange={callback_text_new_changed}
                value={search_value}
            />

        </div>
    };

}
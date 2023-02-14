use crate::{components::admin::book_select::BookSelect, libs::global_vars::GlobalVars};
use savaged_libs::{
    admin_libs::{AdminPagingStatistics, FetchAdminParameters},
    book::Book, user::User,
};
use standard_components::{
    libs::local_storage_shortcuts::set_local_storage_u32,
    ui::{input_text::InputText, input_checkbox::InputCheckbox},
    libs::local_storage_shortcuts::set_local_storage_bool,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFilterSearchProps {
    pub current_user: User,
    pub show_no_select: bool,
    pub paging_sorting_and_filter: FetchAdminParameters,
    pub callback_fetch_admin_params: Callback<FetchAdminParameters>,
    #[prop_or_default]
    pub stats: Option<AdminPagingStatistics>,
}

#[function_component(AdminTableFilterSearch)]
pub fn edit_view_delete_buttons(props: &AdminTableFilterSearchProps) -> Html {
    let callback_fetch_admin_params_1 = props.callback_fetch_admin_params.clone();
    let callback_fetch_admin_params_2 = props.callback_fetch_admin_params.clone();
    let callback_fetch_admin_params_3 = props.callback_fetch_admin_params.clone();
    let paging_sorting_and_filter_1 = props.paging_sorting_and_filter.clone();
    let paging_sorting_and_filter_2 = props.paging_sorting_and_filter.clone();
    let paging_sorting_and_filter_3 = props.paging_sorting_and_filter.clone();
    let search_value_opt = props.paging_sorting_and_filter.filter.clone();
    let search_value = search_value_opt.unwrap_or("".to_owned());

    let callback_text_new_changed = Callback::from(move |new_value: String| {
        let mut nv = paging_sorting_and_filter_1.clone();

        nv.filter = Some(new_value);
        nv.current_page = 0;

        callback_fetch_admin_params_1.emit(nv)
    });

    let mut book_list: Vec<Book> = Vec::new();

    match &props.stats {
        Some(stats) => {
            match &stats.book_list {
                Some(bl) => {
                    book_list = bl.clone();
                }
                None => {}
            }
            // book_list = Some(stats.book_list.clone());
        }
        None => {}
    }

    let callback_set_filter_book = Callback::from(move |new_value: u32| {
        let mut nv = paging_sorting_and_filter_2.clone();

        nv.filter_book = new_value;
        nv.current_page = 0;

        set_local_storage_u32("admin_selected_book", nv.filter_book);

        callback_fetch_admin_params_2.emit(nv)
    });

    let callback_set_hide_no_select = Callback::from(move |new_value: bool| {
        let mut nv = paging_sorting_and_filter_3.clone();

        nv.hide_no_select = new_value;
        nv.current_page = 0;

        set_local_storage_bool("admin_hide_no_select", nv.hide_no_select);

        callback_fetch_admin_params_3.emit(nv)
    });

    // let global_vars = props.global_vars.clone();

    return html! {

        <div class="admin-filter">
        if props.show_no_select {
            <InputCheckbox
                checked={props.paging_sorting_and_filter.hide_no_select}
                onchange={callback_set_hide_no_select}
                label="Hide No Select"
            />
        }
            if book_list.len() > 0 {
                <BookSelect
                    current_user={props.current_user.clone()}
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

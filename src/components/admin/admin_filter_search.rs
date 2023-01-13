use savaged_libs::admin_libs::{AdminPagingStatistics, FetchAdminParameters, BookList};
use standard_components::{ui::{nbsp::Nbsp, input_text::InputText}, libs::local_storage_shortcuts::set_local_storage_u32};
use stdweb::web::event::SelectionChangeEvent;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFilterSearchProps {

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

    let mut book_list: Vec<BookList> = Vec::new();

    match &props.stats {
        Some( stats ) => {
            book_list = stats.book_list.clone();
        }
        None => {}
    }


    let callback_set_filter_book= Callback::from(
        move |e: Event | {
            e.prevent_default();

            let mut nv = paging_sorting_and_filter_2.clone();

            let input: HtmlSelectElement = e.target_unchecked_into();

            nv.filter_book = input.value().parse().unwrap_or(0);
            nv.current_page = 0;

            set_local_storage_u32("admin_selected_book", nv.filter_book);

            callback_fetch_admin_params_2.emit(nv)

        }
    );

    return html!{

        <div class="admin-filter">
            if book_list.len() > 0 {
                <select
                    onchange={callback_set_filter_book}
                >
                    <option selected={props.paging_sorting_and_filter.filter_book == 0} value="0">{"- All Books -"}</option>
                    <optgroup label="Core Books">
                    {book_list.clone().into_iter().map( |book | {
                        if book.primary {
                            html! {
                                <option selected={props.paging_sorting_and_filter.filter_book == book.id} value={book.id.to_string()}>{book.name}</option>
                            }
                        } else {
                            html! {<></>}
                        }
                    }).collect::<Html>()}
                    </optgroup>
                    <optgroup label="Companion Books">
                    {book_list.clone().into_iter().map( |book | {
                        if book.core && !book.primary {
                            html! {
                                <option selected={props.paging_sorting_and_filter.filter_book == book.id} value={book.id.to_string()}>{book.name}</option>
                            }
                        } else {
                            html! {<></>}
                        }
                    }).collect::<Html>()}
                    </optgroup>
                    <optgroup label="Setting Books">
                    {book_list.clone().into_iter().map( |book | {
                        if !book.core && !book.primary {
                            html! {
                                <option selected={props.paging_sorting_and_filter.filter_book == book.id} value={book.id.to_string()}>{book.name}</option>
                            }
                        } else {
                            html! {<></>}
                        }
                    }).collect::<Html>()}
                    </optgroup>
                </select>
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
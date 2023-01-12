use savaged_libs::admin_libs::{AdminPagingStatistics, FetchAdminParameters};
use standard_components::ui::{nbsp::Nbsp, input_text::InputText};
use stdweb::web::event::SelectionChangeEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFilterSearchProps {

    pub paging_sorting_and_filter: FetchAdminParameters,
    pub callback_fetch_admin_params: Callback<FetchAdminParameters>,

}

#[function_component(AdminTableFilterSearch)]
pub fn edit_view_delete_buttons(
    props: &AdminTableFilterSearchProps,
) -> Html {

    let callback_fetch_admin_params_1 = props.callback_fetch_admin_params.clone();
    let paging_sorting_and_filter_1 = props.paging_sorting_and_filter.clone();
    let search_value_opt = props.paging_sorting_and_filter.filter.clone();
    let search_value = search_value_opt.unwrap_or("".to_owned());


    let callback_text_new_changed = Callback::from(
        move |new_value: String | {

            let mut nv = paging_sorting_and_filter_1.clone();

            nv.filter = Some(new_value);

            callback_fetch_admin_params_1.emit(nv)

        }
    );


    return html!{

        <div class="admin-filter">
            <InputText
                placeholder="Filter Results"
                input_type="search"
                onchange={callback_text_new_changed}
                value={search_value}
            />


        </div>
    };



}
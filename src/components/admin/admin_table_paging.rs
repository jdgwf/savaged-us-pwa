use math::round;
use savaged_libs::admin_libs::{AdminPagingStatistics, FetchAdminParameters};
use standard_components::ui::nbsp::Nbsp;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTablePagingProps {
    pub paging_sorting_and_filter: FetchAdminParameters,

    #[prop_or_default]
    pub stats: Option<AdminPagingStatistics>,

    pub callback_fetch_admin_params: Callback<FetchAdminParameters>,
}

#[function_component(AdminTablePaging)]
pub fn edit_view_delete_buttons(props: &AdminTablePagingProps) -> Html {
    let callback_fetch_admin_params_1 = props.callback_fetch_admin_params.clone();
    let callback_fetch_admin_params_2 = props.callback_fetch_admin_params.clone();
    let callback_fetch_admin_params_3 = props.callback_fetch_admin_params.clone();
    let callback_fetch_admin_params_4 = props.callback_fetch_admin_params.clone();
    let paging_sorting_and_filter_1 = props.paging_sorting_and_filter.clone();
    let paging_sorting_and_filter_2 = props.paging_sorting_and_filter.clone();
    let paging_sorting_and_filter_3 = props.paging_sorting_and_filter.clone();
    let paging_sorting_and_filter_4 = props.paging_sorting_and_filter.clone();

    let callback_set_page_count = Callback::from(move |e: Event| {
        e.prevent_default();

        let mut nv = paging_sorting_and_filter_1.clone();

        let input: HtmlSelectElement = e.target_unchecked_into();

        nv.number_per_page = input.value().parse().unwrap_or(25);

        callback_fetch_admin_params_1.emit(nv)
    });

    let callback_set_current_page = Callback::from(move |e: Event| {
        e.prevent_default();

        let mut nv = paging_sorting_and_filter_2.clone();

        let input: HtmlSelectElement = e.target_unchecked_into();

        nv.current_page = input.value().parse().unwrap_or(0);

        callback_fetch_admin_params_2.emit(nv)
    });

    let callback_next_page = Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        let mut nv = paging_sorting_and_filter_3.clone();

        nv.current_page += 1;

        callback_fetch_admin_params_3.emit(nv)
    });
    let callback_previous_page = Callback::from(move |e: MouseEvent| {
        e.prevent_default();

        let mut nv = paging_sorting_and_filter_4.clone();

        nv.current_page -= 1;

        callback_fetch_admin_params_4.emit(nv)
    });

    match &props.stats {
        Some(paging_stats) => {
            let number_of_pages = round::ceil(
                (paging_stats.filtered_count / props.paging_sorting_and_filter.number_per_page)
                    as f64,
                0,
            ) as u32;

            let mut number_per_page: Vec<u32> = Vec::new();

            for count in 0..(number_of_pages + 1) {
                number_per_page.push(count);
            }

            return html! {
                <div class="admin-paging">
                    <div class="row_count">
                        <select
                            onchange={callback_set_page_count}
                        >
                            <option selected={props.paging_sorting_and_filter.number_per_page == 10} value={10}>{10}</option>
                            <option selected={props.paging_sorting_and_filter.number_per_page == 25} value={15}>{15}</option>
                            <option selected={props.paging_sorting_and_filter.number_per_page == 25} value={20}>{20}</option>
                            <option selected={props.paging_sorting_and_filter.number_per_page == 25} value={25}>{25}</option>
                            <option selected={props.paging_sorting_and_filter.number_per_page == 50} value={50}>{50}</option>
                            <option selected={props.paging_sorting_and_filter.number_per_page == 100} value={100}>{100}</option>
                        </select>
                    </div>
                    <div class="paging">
                        if number_per_page.len() > 0 {
                    <div class="btn-ph text-right">
                        if props.paging_sorting_and_filter.current_page > 0 {
                            <button
                                type="button"
                                onclick={callback_previous_page}
                                class="btn btn-primary btn-sm"
                            >
                                <i class="fa fa-circle-arrow-left" />
                            </button>
                        }
                    </div>
                        <label>
                        {"Page: "}<Nbsp />
                        <select
                            onchange={callback_set_current_page}
                            value={props.paging_sorting_and_filter.current_page.to_string()}
                        >
                            {number_per_page.into_iter().map( | count | {
                                if props.paging_sorting_and_filter.current_page == count {
                                    html! {
                                        <option selected={true} value={count.to_string()}>{count+1}</option>
                                    }
                                } else {
                                    html! {
                                        <option value={count.to_string()}>{count+1}</option>
                                    }
                                }
                            }).collect::<Html>()}
                        </select>
                        </label>
                    <div class="btn-ph text-left">
                        if props.paging_sorting_and_filter.current_page < number_of_pages {
                            <button
                                type="button"
                                onclick={callback_next_page}
                                class="btn btn-primary btn-sm"
                            >
                                <i class="fa fa-circle-arrow-right" />
                            </button>
                        }
                    </div>
                        }
                    </div>
                    <div class="total">
                        if paging_stats.filtered_count != paging_stats.non_filtered_count {
                        {paging_stats.filtered_count}<Nbsp />{"/"}<Nbsp />{paging_stats.non_filtered_count}
                        } else {
                        {paging_stats.non_filtered_count}
                        }
                    </div>

                </div>
            };
        }
        None => {
            return html! {<></>};
        }
    }
}

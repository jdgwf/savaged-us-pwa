use savaged_libs::book::Book;
use standard_components::ui::input_label::InputLabel;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use crate::libs::global_vars::GlobalVars;

#[derive(Properties, PartialEq)]
pub struct BookSelectProps {
    pub book_list: Vec<Book>,
    pub onchange: Callback<u32>,
    pub value: u32,

    pub global_vars: GlobalVars,

    #[prop_or_default]
    pub show_all_books: bool,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub inline: bool,

    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub label_class: String,
}

#[function_component(BookSelect)]
pub fn book_select(
    props: &BookSelectProps,
) -> Html {

    let onchange = props.onchange.clone();

    // let mut book_list: Vec<Book> = props.book_list.clone();

    let callback_set_filter_book= Callback::from(
        move |e: Event | {
            e.prevent_default();

            let nv = onchange.clone();

            let input: HtmlSelectElement = e.target_unchecked_into();

            let filter_book = input.value().parse().unwrap_or(0);


            onchange.emit( filter_book )

        }
    );

    if props.readonly {
        return html!{
            <>
            if props.book_list.len() > 0 {
                <label>
                    if !props.label.is_empty() {
                        <>{props.label.to_owned()}{":"}<br /></>
                    }
                    <InputLabel
                        label={props.label.to_owned()}
                        inline={props.inline}
                    />
                    if props.value == 0 {
                        <>{"(No book selected)"}</>
                    } else {
                        <></>
                    }
                    {props.book_list.clone().into_iter().map( | book | {
                        if props.value == book.id {
                            html!{<>{book.name}</>}
                        } else {
                            html!{<></>}
                        }
                    }).collect::<Html>()}

                </label>
            }
            </>
        };
    }

    let global_vars = props.global_vars.clone();

    let book_list_option = Some(props.book_list.clone());
    return html!{
            <>

            if props.book_list.len() > 0 {
                <label>
                    <InputLabel
                        label={props.label.to_owned()}
                        inline={props.inline}
                    />
                <select
                    onchange={callback_set_filter_book}
                >
                    if props.show_all_books {
                        <option selected={props.value == 0} value="0">{"- All Books -"}</option>
                    } else {
                        <option selected={props.value == 0} value="0">{"- No Book Selected -"}</option>
                    }
                    <optgroup label="Core Books">
                    {props.book_list.clone().into_iter().map( | book | {
                        if book.primary {
                            html! {
                                <option disabled={global_vars.current_user.admin_can_write_book(&book_list_option, book.id) == false} selected={props.value == book.id} value={book.id.to_string()}>{book.name}</option>
                            }
                        } else {
                            html! {<></>}
                        }
                    }).collect::<Html>()}
                    </optgroup>
                    <optgroup label="Companion Books">
                    {props.book_list.clone().into_iter().map( | book | {
                        if book.core && !book.primary {
                            html! {
                                <option disabled={global_vars.current_user.admin_can_write_book(&book_list_option, book.id) == false} selected={props.value == book.id} value={book.id.to_string()}>{book.name}</option>
                            }
                        } else {
                            html! {<></>}
                        }
                    }).collect::<Html>()}
                    </optgroup>
                    <optgroup label="Setting Books">
                    {props.book_list.clone().into_iter().map( | book | {
                        if !book.core && !book.primary {
                            html! {
                                <option disabled={global_vars.current_user.admin_can_write_book(&book_list_option, book.id) == false} selected={props.value == book.id} value={book.id.to_string()}>{book.name}</option>
                            }
                        } else {
                            html! {<></>}
                        }
                    }).collect::<Html>()}
                    </optgroup>
                </select>
                </label>
            }


        </>
    };

}
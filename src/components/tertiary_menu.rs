use standard_components::libs::local_storage_shortcuts::{get_local_storage_string, set_local_storage_string};
use web_sys::MouseEvent;
use yew::{function_component, Properties, Html, html, Callback};

use crate::libs::global_vars::GlobalVars;


#[derive(PartialEq, Clone)]
pub struct TertiaryMenuItem {
    pub tag: String,
    pub label: String,
    pub callback: Option<Callback<String>>,
    pub class: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct TertiaryMenuProps {
    pub global_vars: GlobalVars,
    pub local_storage_variable: String,
    pub menu_items: Vec<TertiaryMenuItem>,
    pub menu_changed_callback: Callback<String>,
}

#[function_component(TertiaryMenu)]
pub fn tertiary_menu(
    props: &TertiaryMenuProps,
) -> Html {

    let mut filter_type = "character".to_owned();

    if !props.global_vars.server_side_renderer {
        filter_type = get_local_storage_string( props.local_storage_variable.as_str(), "character".to_string());
    }

    let menu_changed_callback = props.menu_changed_callback.clone();
    let local_storage_variable = props.local_storage_variable.clone();
    // let on_click =  move | menu_tag: String | {
    //     set_local_storage_string( local_storage_variable.as_str(), menu_tag.to_owned());
    //     menu_changed_callback.emit(menu_tag.to_owned());
    // };

    let menu_items = props.menu_items.clone();
    html! {
        <ul class="tertiary-menu">
            {menu_items.into_iter().map(  move | item | {
                let local_storage_variable = local_storage_variable.to_owned();
                let filter_type = filter_type.to_owned();
                let menu_changed_callback = menu_changed_callback.clone();
                let tag = item.tag.clone();
                let on_click= move | e: MouseEvent | {
                    e.prevent_default();
                    // on_click("character".to_owned());
                    set_local_storage_string( local_storage_variable.as_str(), item.tag.clone());
                    menu_changed_callback.emit(item.tag.to_owned());
                };

                html!{<li
                    class={class_is_active( filter_type, &tag.as_ref())}
                >
                    <a
                        href="#"
                        onclick={on_click}
                    >
                        {item.label.to_owned()}
                    </a>
                </li>}
            }).collect::<Html>()}


        </ul>
    }
}



fn class_is_active(
    current_select: String,
    current_menu_item: &str
) -> String {


    if  current_select.as_str() == current_menu_item {
        return "active".to_string();
    }
    return "".to_string();
}
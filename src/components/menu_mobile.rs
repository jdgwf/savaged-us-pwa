// use gloo_console::log;
use crate::libs::site_vars::SiteVars;
use crate::menu_items::{get_menu_items,user_can_see_menu_item, MenuItem};

use savaged_libs::user::User;
use standard_components::ui::nbsp::Nbsp;
use web_sys::MouseEvent;
use yew::{function_component, html, Html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct MenuMobileProps {
    pub site_vars: SiteVars,
}

#[function_component(MenuMobile)]
pub fn menu_mobile(props: &MenuMobileProps) -> Html {
    let active_class = "".to_owned();

    html! {
        <div class={active_class}>
            <ul onclick={props.site_vars.hide_popup_menus_callback.clone()} class={"main-menu"}>
            {get_menu_items(&props.site_vars).into_iter().map( | menu | {

                // log!("&menu.sub_menu_tag.clone().unwrap() == &props.site_vars.current_menu", &menu.menu_tag.clone().unwrap(), &props.site_vars.current_menu);
                // log!("&props.site_vars.current_sub_menu", &props.site_vars.current_sub_menu);
                // log!("&menu.sub_menu_tag.clone().unwrap()", &menu.sub_menu_tag.clone().unwrap());
                // if user_can_see_menu_item( &props.site_vars.current_user, &menu)
                // {
                    let mut li_class = "".to_string();
                    if menu.menu_tag != None
                        && props.site_vars.current_sub_menu.is_empty()
                        && &menu.menu_tag.clone().unwrap() == &props.site_vars.current_menu {
                        li_class = "active".to_string();
                    }
                    match &menu.link_class {
                        Some( link_class ) => {
                            li_class = link_class.to_owned() + &" " + &li_class;
                        }
                        None => {}
                    }
                    let submenu = make_submenu( menu.clone(), &props.site_vars.current_user, props.site_vars.current_sub_menu.clone(), props.site_vars.current_menu.clone() );
                    if menu.show_mobile && user_can_see_menu_item( &props.site_vars.current_user, &menu) {
                        match menu.html {
                            Some( html ) => {
                                return html! {
                                    <li class={li_class} title={menu.title}>
                                    {html}

                                    {submenu}
                                    </li>
                                };
                            }
                            None => {
                                return html! {
                                    <li class={li_class} title={menu.title}>
                                        <i class={menu.icon_class.clone()} /><Nbsp />
                                        {menu.label}
                                        {submenu}

                                    </li>
                                };
                            }
                        }
                    } else {
                        return html!{<></>}
                    }

                // } else {
                //     return html!{<></>};
                // }

            }).collect::<Html>()}

            </ul>

        </div>
    }
}

fn make_submenu(
    menu: MenuItem,
    current_user: &User,
    current_sub_menu: String,
    current_menu: String,
) -> Html {
    match &menu.submenu {
        Some(submenu_items) => {
            return html! {

                <ul class="sub-menu">
            {submenu_items.into_iter().map( | sub_item | {

                let sub_item = sub_item.clone();

                let mut li_class = "".to_string();
                if sub_item.sub_menu_tag != None
                    && !current_sub_menu.is_empty()
                    && sub_item.sub_menu_tag.unwrap() == current_sub_menu
                {
                    li_class = "active".to_string();
                }
                match &sub_item.link_class {
                    Some( link_class ) => {
                        li_class = link_class.to_owned() + &" " + &li_class;
                    }
                    None => {}
                }

                if sub_item.show_mobile {
                    match sub_item.html.clone() {
                        Some( html ) => {
                            return html! {
                                <li class={li_class} title={sub_item.title.clone()}>
                                {html}
                                // <br />{&sub_item.sub_menu_tag}
                                // <br />{&props.site_vars.current_sub_menu}
                                </li>
                            };
                        }
                        None => {
                            return html! {
                                <li class={li_class} title={sub_item.title.clone()}>
                                    <i class={sub_item.icon_class.clone()} /><Nbsp />
                                    {sub_item.label.clone()}
                                </li>
                            };
                        }
                    }
                } else {
                    return html!{<></>}
                }
                }).collect::<Html>()}
                </ul>
            };
        }
        None => {
            return html! {<></>};
        }
    }
}

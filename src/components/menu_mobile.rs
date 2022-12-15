use yew::{function_component, Properties, Html, html};
use crate::libs::global_vars::GlobalVars;
use crate::menu_items::{get_menu_items, user_can_see_menu_item, MenuItem};
// use standard_components::libs::set_document_title::set_document_title;

use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct MenuMobileProps {
    pub global_vars: GlobalVars,
}
#[function_component(MenuMobile)]
pub fn menu_mobile(
    props: &MenuMobileProps,
) -> Html {


    let active_class = "".to_owned();

    html! {
        <div class={active_class}>
            <ul onclick={props.global_vars.hide_popup_menus_callback.clone()} class={"main-menu"}>
            {get_menu_items(&props.global_vars).into_iter().map( | menu | {

                // log!("submenu_tag == &props.submenu_tag", submenu_tag, &props.submenu_tag);
                if user_can_see_menu_item( &props.global_vars.current_user, &menu)
                {
                    let mut li_class = "".to_string();
                    if &menu.sub_menu_tag == &props.global_vars.current_menu {
                        li_class = "active".to_string();
                    }
                    match &menu.link_class {
                        Some( link_class ) => {
                            li_class = link_class.to_owned() + &" " + &li_class;
                        }
                        None => {}
                    }
                    let submenu = make_submenu( menu.clone(), props.global_vars.clone() );
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
                    return html!{<></>};
                }

            }).collect::<Html>()}
            //     <li class={home_class_active}>
            //         <Link<MainRoute> to={MainRoute::Home}><i class="fa fa-house" /><Nbsp />{"Home"}</Link<MainRoute>>
            //         {home_submenu}
            //     </li>
            //     if global_vars.current_user.id > 0 {
            //         <li class={user_data_class_active}>
            //             <Link<UserDataRoute> to={UserDataRoute::Saves}><i class="fa fa-boxes-stacked" /><Nbsp />{"My Stuff"}</Link<UserDataRoute>>
            //             {user_data_submenu}
            //         </li>
            //     }
            //     <li class={about_class_active}>
            //         <Link<MainRoute> to={MainRoute::About}><i class="fa fa-circle-info" /><Nbsp />{"About"}</Link<MainRoute>>
            //     </li>
            //     <li class={tech_class_active}>
            //         <Link<MainRoute> to={MainRoute::Tech}><i class="fa fa-microchip" /><Nbsp />{"Tech"}</Link<MainRoute>>
            //     </li>
            //     <li class={todos_class_active}>
            //         <Link<MainRoute> to={MainRoute::ToDos}><i class="fa fa-list" /><Nbsp />{"To-Dos"}</Link<MainRoute>>
            //         {todos_submenu}
            //     </li>
            //     // <li class={playground_class_active}>
            //     //     <Link<MainRoute> to={MainRoute::Playground}><i class="fa fa-list" /><Nbsp />{"Playground"}</Link<MainRoute>>
            //     // </li>
            //     if global_vars.current_user.id > 0 {
            //         <li class={settings_class_active}>
            //             <Link<UserRoute> to={UserRoute::SettingsPrivate}>{"Settings"}</Link<UserRoute>>
            //             {settings_submenu}
            //         </li>
            //     }

            </ul>

        </div>
    }
}


fn make_submenu(
    menu: MenuItem,
    global_vars: GlobalVars,
) -> Html {

    match &menu.submenu {
        Some( submenu_items ) => {

            return html! {

                <ul class="sub-menu">
            {submenu_items.into_iter().map( | sub_item | {

                let mut li_class = "".to_string();
                if &sub_item.sub_menu_tag == &global_vars.current_sub_menu
                    && !global_vars.current_sub_menu.is_empty()
                    && !sub_item.sub_menu_tag.is_empty()
                {
                    li_class = "active".to_string();
                }
                match &sub_item.link_class {
                    Some( link_class ) => {
                        li_class = link_class.to_owned() + &" " + &li_class;
                    }
                    None => {}
                }
                match sub_item.html.clone() {
                    Some( html ) => {
                        return html! {
                            <li class={li_class} title={sub_item.title.clone()}>
                            {html}
                            // <br />{&sub_item.sub_menu_tag}
                            // <br />{&props.global_vars.current_sub_menu}
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
                }).collect::<Html>()}
                </ul>
            }
        }
        None => {
            return html!{<></>};
        }
    }
}
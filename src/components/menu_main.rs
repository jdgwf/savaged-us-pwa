use crate::components::lds_spinner::LDSSpinner;
use crate::libs::global_vars::GlobalVars;
use crate::main_app::MainRoute;
use crate::menu_items::{get_menu_items, user_can_see_menu_item};
use crate::pages::user::UserRoute;
use yew::{function_component, Properties, Html, html};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct MenuMainProps {
    pub global_vars: GlobalVars,
}
#[function_component(MenuMain)]
pub fn menu_main(
    props: &MenuMainProps,
) -> Html {

    let current_submenu_tag = props.global_vars.current_sub_menu.clone();
    let current_menu_tag = props.global_vars.current_menu.clone();

    let mut show_submenu = false;

    let submenu = get_menu_items(&props.global_vars).into_iter().map( | menu | {
        match &menu.menu_tag {
            Some( menu_tag ) => {
                // log!("menu_tag == &props.submenu_tag", submenu_tag, &current_submenu_tag);
                if
                    current_menu_tag.as_str() == menu_tag.as_str()
                    && user_can_see_menu_item( &props.global_vars.current_user, &menu)
                {
                    match menu.submenu {
                        Some( submenu_items ) => {

                            return submenu_items.into_iter().map( | sub_item | {

                                let mut li_class = "".to_string();
                                if sub_item.sub_menu_tag != None
                                    && &sub_item.sub_menu_tag.unwrap() == &current_submenu_tag
                                    && !current_submenu_tag.is_empty()
                                {
                                    li_class = "active".to_string();
                                }
                                match sub_item.link_class {
                                    Some( link_class ) => {
                                        li_class = link_class + &" " + &li_class;
                                    }
                                    None => {}
                                }
                                match sub_item.html {
                                    Some( html ) => {
                                        show_submenu = true;
                                        return html! {
                                            <li class={li_class} title={sub_item.title}>
                                            {html}
                                        //    <br />{&sub_item.sub_menu_tag}
                                        //    <br />{&props.global_vars.current_sub_menu}
                                            </li>
                                        };
                                    }
                                    None => {
                                        show_submenu = true;
                                        return html! {
                                            <li class={li_class} title={sub_item.title}>{sub_item.label}</li>
                                        };
                                    }
                                }
                                }).collect::<Html>();

                        }
                        None => {
                            return html!{<></>};
                        }
                    }
                } else {
                    return html!{<></>};
                }
            }
            None => {
                return html!{<></>};
            }
        }

    }).collect::<Html>();

    let mut login_class_active = "login-item".to_owned();

    if current_menu_tag == "main-userlogin".to_owned()
    || current_menu_tag == "main-register".to_owned()
    || current_menu_tag == "main-userforgotpassword".to_owned()
    || current_menu_tag == "main-userrouter".to_owned()
    {
        login_class_active = "login-item active".to_owned();
    }

    // log!( format!("props.global_vars.current_user.updated_on {:?}", props.global_vars.current_user.updated_on) );
    html! {
        <>
        // {"props.global_vars.current_menu: "}{&props.global_vars.current_menu}<br />
        <div class={"top-menu-bottom"}>
        <div class={"width-limit"}>
        <ul class={"top-menu"}>
            <li class={"mobile-menu-button"}>
                <svg
                    onclick={props.global_vars.toggle_mobile_menu_callback.clone()}
                    stroke="currentColor"
                    fill="currentColor"
                    stroke-width="0"
                    viewBox="0 0 448 512"
                    height="1em"
                    width="1em"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path d="M16 132h416c8.837 0 16-7.163 16-16V76c0-8.837-7.163-16-16-16H16C7.163 60 0 67.163 0 76v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16z"></path>
                </svg>
            </li>
            {get_menu_items(&props.global_vars).into_iter().map( | menu | {
                if user_can_see_menu_item( &props.global_vars.current_user, &menu) {
                    let mut li_class = "".to_string();
                    if menu.hardcoded {
                        return html!(<></>);
                    }
                    if menu.menu_tag != None
                    && menu.menu_tag.unwrap() == current_menu_tag {
                        li_class = "active".to_string();
                    }
                    match menu.link_class {
                        Some( link_class ) => {
                            li_class = link_class + &" " + &li_class;
                        }
                        None => {}
                    }



                    match menu.html {
                        Some( html ) => {
                            return html! {
                                <li class={li_class} title={menu.title}>
                                    {html}
                                    // {menu.menu_tag.as_ref().unwrap()}<br/>{current_menu_tag.clone()}<br />
                                </li>
                            };
                        }
                        None => {
                            return html! {
                                <li class={li_class} title={menu.title}>{menu.label}</li>
                            };
                        }
                    }
                } else {
                    return html!{<></>};
                }

            }).collect::<Html>()}

            if !props.global_vars.server_side_renderer {
                <li class={login_class_active}>
                    if props.global_vars.offline {
                        <div style={"margin-top: -2rem; margin-right: .5rem;text-align: center;"}>
                            {"OFFLINE"}
                            <div class="small-text">{"For now refresh the page"}<br />{"to try to connect again"}</div>
                        </div>
                    }
                    if props.global_vars.current_user.id > 0 && !props.global_vars.offline {
                        <div class="user-login-badge">

                            if props.global_vars.current_user.unread_notifications > 0 {
                                <Link<UserRoute> to={UserRoute::Notifications}>
                                    <div class={"unread-notifications"}>{props.global_vars.current_user.unread_notifications}</div>
                                </Link<UserRoute>>
                            }
                            <Link<UserRoute> to={UserRoute::SettingsPrivate}><img
                            src={props.global_vars.current_user.get_image( &props.global_vars.server_root )}
                            /></Link<UserRoute>>

                        </div>
                    } else {
                        <>
                            if !props.global_vars.offline {
                                <Link<MainRoute> to={MainRoute::UserLogin}>{"Login/Register"}</Link<MainRoute>>
                            }
                        </>
                    }
                </li>
            } else {
                <li class={login_class_active}>
                    <LDSSpinner />
                    // <LDSRoller />
                </li>
            }

        </ul>

        </div>
        </div>
        if show_submenu {
            <div class="width-limit">
                <ul class="sub-menu">
                    {submenu}
                </ul>
            </div>
        }

        </>
    }
}


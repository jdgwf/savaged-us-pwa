use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::AdminRoute;
use crate::pages::info::InfoRoute;
use crate::{main_app::MainRoute, pages::user::UserRoute};
use savaged_libs::user::User;
use standard_components::ui::nbsp::Nbsp;
use yew::{html, Html};
use yew_router::prelude::Link;
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub html: Option<Html>,
    pub registered_only: bool,
    pub wildcard_only: bool,
    pub developer_only: bool,
    pub admin_only: bool,

    pub submenu: Option<Vec<MenuItem>>,

    pub link_class: Option<String>,

    pub label: String,
    pub url: Option<String>,
    pub title: String,
    pub icon_class: Option<String>,

    pub menu_tag: Option<String>,
    pub sub_menu_tag: Option<String>,

    pub hardcoded: bool,
}

pub fn get_menu_items(global_vars: &GlobalVars) -> Vec<MenuItem> {
    let mut menu = vec![
        MenuItem {
            hardcoded: false,
            html: Some(html! {
                <Link<MainRoute>
                    to={MainRoute::Home}
                >
                    <i class="fa fa-house" /><Nbsp />
                    {"Home"}
                </Link<MainRoute>>
            }),
            registered_only: false,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,

            submenu: None,

            link_class: None,

            title: "The Home Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "Home".to_owned(),
            url: None,

            menu_tag: Some("main-home".to_owned()),
            sub_menu_tag: None,
        },
        MenuItem {
            hardcoded: false,
            html: Some(html! {
                <Link<UserRoute>
                    to={UserRoute::UserSavesList}
                >
                    <i class="fa fa-boxes-stacked" /><Nbsp />
                    {"My Stuff"}
                </Link<UserRoute>>
            }),
            registered_only: true,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,

            link_class: None,

            title: "Your Data".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "My Stuff".to_owned(),
            url: None,

            menu_tag: Some("main-my-stuff".to_owned()),
            sub_menu_tag: None,

            submenu: Some(vec![
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<UserRoute>
                            to={UserRoute::UserSavesList}
                        >
                            <i class="fa fa-boxes-stacked" /><Nbsp />
                            {"My Saves"}
                        </Link<UserRoute>>
                    }),
                    registered_only: true,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    submenu: None,

                    link_class: None,

                    title: "The My Saves Page".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "My Saves".to_owned(),
                    url: None,

                    menu_tag: None,
                    sub_menu_tag: Some("user-data-saves".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<UserRoute>
                            to={UserRoute::UserCampaigns}
                        >
                            <i class="fa fa-dice" /><Nbsp />
                            {"Campaigns"}
                        </Link<UserRoute>>
                    }),
                    registered_only: true,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    submenu: None,

                    link_class: None,

                    title: "The Campaigns Page".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Campaigns".to_owned(),
                    url: None,

                    menu_tag: None,
                    sub_menu_tag: Some("user-data-campaigns".to_owned()),
                },
            ]),
        },
        MenuItem {
            hardcoded: false,
            html: Some(html! {
                <Link<InfoRoute>
                    to={InfoRoute::InfoAbout}
                >
                    <i class="fa fa-circle-info" /><Nbsp />
                    {"Info"}
                </Link<InfoRoute>>
            }),
            registered_only: false,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,
            link_class: None,
            title: "The About Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "About".to_owned(),
            url: None,
            menu_tag: Some("main-info".to_owned()),
            sub_menu_tag: None,

            submenu: Some(vec![
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<InfoRoute>
                            to={InfoRoute::InfoAbout}
                        >
                            <i class="fa fa-circle-info" /><Nbsp />
                            {"About"}
                        </Link<InfoRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,
                    link_class: None,

                    submenu: None,

                    title: "The About Page".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Info".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("info-about".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<InfoRoute>
                            to={InfoRoute::InfoContactUs}
                        >
                            <i class="fa fa-envelope" /><Nbsp />
                            {"Contact Us"}
                        </Link<InfoRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    link_class: None,

                    submenu: None,

                    title: "Contact Us".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Contact Us".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("info-contact-us".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<InfoRoute>
                            to={InfoRoute::InfoPartners}
                        >
                            <i class="fa fa-handshake" /><Nbsp />
                            {"Partners"}
                        </Link<InfoRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    link_class: None,

                    submenu: None,

                    title: "Our Partners".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Partners".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("info-partners".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<InfoRoute>
                            to={InfoRoute::InfoPrivacyPolicy}
                        >
                            <i class="fa fa-user-secret" /><Nbsp />
                            {"Privacy Policy"}
                        </Link<InfoRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    link_class: None,

                    submenu: None,

                    title: "Our Privacy Policy".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Privacy Policy".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("info-privacy-policy".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<InfoRoute>
                            to={InfoRoute::InfoTech}
                        >
                            <i class="fa fa-microchip" /><Nbsp />
                            {"Tech"}
                        </Link<InfoRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    link_class: None,

                    submenu: None,

                    title: "The Technology Used Page".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Tech".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("info-tech".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<InfoRoute>
                            to={InfoRoute::InfoTodos}
                        >
                            <i class="fa fa-list" /><Nbsp />
                            {"To-Dos"}
                        </Link<InfoRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,

                    link_class: None,

                    submenu: None,

                    title: "The To-Dos Page".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "To-Dos".to_owned(),
                    url: None,

                    menu_tag: None,
                    sub_menu_tag: Some("info-todos".to_owned()),
                },
            ]),
        },
    ];

    menu = _add_admin_tab(&global_vars, menu);

    menu.push(

        MenuItem {
            hardcoded: true,

            html: Some( html!{
                <Link<UserRoute>
                    to={UserRoute::SettingsPrivate}
                >
                    <i class={"fa-solid fa-cogs"}></i><Nbsp />
                    {"User Settings"}
                </Link<UserRoute>>
            }),

            registered_only: true,
            wildcard_only: true,
            developer_only: true,
            admin_only: true,

            link_class: None,

            title: "Settings".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "Settings".to_owned(),
            url: None,

            menu_tag: Some("main-user-login".to_owned()),
            sub_menu_tag: None,

            submenu: Some(
                vec![
                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserRoute>
                                to={UserRoute::SettingsPrivate}
                            >
                                <i class={"fa-solid fa-user-secret"}></i><Nbsp />
                                {"Private Settings"}
                            </Link<UserRoute>>
                        }),
                        registered_only: false,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        link_class: None,

                        submenu: None,

                        title: "The Private Settings Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Private Settings".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-private".to_owned()),
                    },
                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserRoute>
                                to={UserRoute::SettingsPublic}
                            >
                                <i class={"fa-solid fa-globe"}></i><Nbsp />
                                {"Public Settings"}
                            </Link<UserRoute>>
                        }),
                        registered_only: false,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        link_class: None,

                        submenu: None,

                        title: "The Public Settings Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Public Settings".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-public".to_owned()),
                    },

                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserRoute> to={UserRoute::Notifications}>
                                <i class={"fa-solid fa-radio"}></i><Nbsp />{"Notifications"}

                                if global_vars.current_user.unread_notifications > 0 {
                                    <>
                                        <div class={"notification-spacer"} />
                                        <div id="unread-notifications" class={"unread-notifications"}>
                                            {global_vars.current_user.unread_notifications}
                                        </div>

                                    </>
                                }

                            </Link<UserRoute>>
                        }),
                        registered_only: true,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        link_class: None,

                        submenu: None,

                        title: "The Notifications Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Notifications".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-notifications".to_owned()),
                    },
                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserRoute> to={UserRoute::Subscription}><i class={"fa-solid fa-credit-card"}></i><Nbsp />{"Subscriptions"}</Link<UserRoute>>
                        }),
                        registered_only: true,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        link_class: None,

                        submenu: None,

                        title: "The Subscription Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Subscription".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-subscription".to_owned()),
                    },
                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserRoute> to={UserRoute::Devices}><i class={"fa-solid fa-computer"}></i><Nbsp />{"Devices"}</Link<UserRoute>>
                        }),
                        registered_only: true,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        link_class: None,

                        submenu: None,

                        title: "The Devices Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Devices Settings".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-devices".to_owned()),
                    },

                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserRoute> to={UserRoute::SettingsAPIKey}><i class={"fa-solid fa-key"}></i><Nbsp />{"API Key"}</Link<UserRoute>>
                        }),
                        registered_only: true,
                        wildcard_only: true,
                        developer_only: false,
                        admin_only: false,

                        link_class: None,

                        submenu: None,

                        title: "The API Key Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "API Key".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-apikey".to_owned()),
                    },

                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <a href="#" onclick={&global_vars.logout_callback}><i class={"fa-solid fa-sign-out"}></i><Nbsp />{"Logout"}</a>
                        }),
                        registered_only: true,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        link_class: Some("logout-item".to_string()),

                        submenu: None,

                        title: "Click here to log out".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Logout".to_owned(),
                        url: None,

                        menu_tag: None,
                        sub_menu_tag: Some("settings-logout".to_owned()),
                    },
                ]
            ),

        },
    );

    return menu;
}

fn _add_admin_tab(global_vars: &GlobalVars, mut menu: Vec<MenuItem>) -> Vec<MenuItem> {
    if global_vars.current_user.has_developer_access() {
        menu.push(MenuItem {
            hardcoded: false,
            html: Some(html! {
                <Link<AdminRoute>
                    to={AdminRoute::AdminHome}
                >
                    <i class="fa fa-lock" /><Nbsp />
                    {"Admin"}
                </Link<AdminRoute>>
            }),
            registered_only: false,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,
            link_class: None,

            title: "The Administration Section".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "Admin".to_owned(),
            url: None,
            menu_tag: Some("main-admin".to_owned()),
            sub_menu_tag: None,

            submenu: Some(vec![
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<AdminRoute>
                            to={AdminRoute::AdminHome}
                        >
                            <i class="fa fa-lock" /><Nbsp />
                            {"Home"}
                        </Link<AdminRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,
                    link_class: None,

                    submenu: None,

                    title: "Administration Home".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Administration Home".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("admin-home".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<AdminRoute>
                            to={AdminRoute::AdminUsersList}
                        >
                            <i class="fa fa-users" /><Nbsp />
                            {"Users"}
                        </Link<AdminRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,
                    link_class: None,

                    submenu: None,

                    title: "Users Administration".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Users".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("admin-users".to_owned()),
                },
                MenuItem {
                    hardcoded: false,
                    html: Some(html! {
                        <Link<AdminRoute>
                            to={AdminRoute::AdminGameDataHome}
                        >
                            <i class="fa fa-dice" /><Nbsp />
                            {"Game Data"}
                        </Link<AdminRoute>>
                    }),
                    registered_only: false,
                    wildcard_only: false,
                    developer_only: false,
                    admin_only: false,
                    link_class: None,

                    title: "Game Data Administration".to_owned(),
                    icon_class: None, // "fa fa-house".to_owned(),
                    label: "Game Data".to_owned(),
                    url: None,
                    menu_tag: None,
                    sub_menu_tag: Some("admin-game-data".to_owned()),

                    submenu: None,
                },
            ]),
        });
    }

    return menu;
}

pub fn user_can_see_menu_item(user: &User, item: &MenuItem) -> bool {
    if !item.registered_only && !item.wildcard_only && !item.developer_only && !item.admin_only {
        return true;
    }

    if user.is_admin && item.admin_only {
        return true;
    }

    if user.is_developer && item.developer_only {
        return true;
    }

    if user.is_premium && item.wildcard_only {
        return true;
    }

    if user.id > 0 && item.registered_only {
        return true;
    }

    return false;
}

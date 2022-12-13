use standard_components::ui::nbsp::Nbsp;
use yew::{Html, html};
use yew_router::prelude::Link;
use savaged_libs::user::User;
use crate::libs::global_vars::GlobalVars;
use crate::{main_app::MainRoute, pages::{user_data::user_data_router::UserDataRoute, user::user_router::UserRoute}};

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub html: Option<Html>,
    pub registered_only: bool,
    pub wildcard_only: bool,
    pub developer_only: bool,
    pub admin_only: bool,

    pub submenu_tag: Option<String>,
    pub submenu: Option<Vec<MenuItem>>,

    pub link_class: Option<String>,

    pub label: String,
    pub url: Option<String>,
    pub title: String,
    pub icon_class: Option<String>,

    pub menu_tag: String,
    pub sub_menu_tag: String,

    pub hardcoded: bool,
}

pub fn get_menu_items(
    global_vars: &GlobalVars,
) -> Vec<MenuItem> {

    return vec!(
        MenuItem {

            hardcoded: false,
            html: Some( html!{
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

            submenu_tag: None,
            submenu: None,

            link_class: None,

            title: "The Home Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "Home".to_owned(),
            url: None,

            menu_tag: "main-home".to_owned(),
            sub_menu_tag: "".to_owned(),
        },
        MenuItem {
            hardcoded: false,
            html: Some( html!{
                <Link<UserDataRoute>
                    to={UserDataRoute::Saves}
                >
                    <i class="fa fa-boxes-stacked" /><Nbsp />
                    {"My Stuff"}
                </Link<UserDataRoute>>
            }),
            registered_only: true,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,

            submenu_tag: Some("my-stuff".to_owned()),
            submenu: Some(
                vec![
                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserDataRoute>
                                to={UserDataRoute::Saves}
                            >
                                <i class="fa fa-boxes-stacked" /><Nbsp />
                                {"My Saves"}
                            </Link<UserDataRoute>>
                        }),
                        registered_only: true,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        submenu_tag: None,
                        submenu: None,

                        link_class: None,

                        title: "The My Saves Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "My Saves".to_owned(),
                        url: None,

                        menu_tag: "main-my-stuff".to_owned(),
                        sub_menu_tag: "user-data-saves".to_owned(),
                    },
                    MenuItem {
                        hardcoded: false,
                        html: Some( html!{
                            <Link<UserDataRoute>
                                to={UserDataRoute::Campaigns}
                            >
                                <i class="fa fa-dice" /><Nbsp />
                                {"Campaigns"}
                            </Link<UserDataRoute>>
                        }),
                        registered_only: true,
                        wildcard_only: false,
                        developer_only: false,
                        admin_only: false,

                        submenu_tag: None,
                        submenu: None,

                        link_class: None,

                        title: "The Campaigns Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Campaigns".to_owned(),
                        url: None,

                        menu_tag: "main-my-stuff".to_owned(),
                        sub_menu_tag: "user-data-campaigns".to_owned(),
                    },
                ]
            ),
            link_class: None,

            title: "The About Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "About".to_owned(),
            url: None,

            menu_tag: "main-my-stuff".to_owned(),
            sub_menu_tag: "".to_owned(),
        },
        MenuItem {
            hardcoded: false,
            html: Some( html!{
                <Link<MainRoute>
                    to={MainRoute::About}
                >
                    <i class="fa fa-circle-info" /><Nbsp />
                    {"About"}
                </Link<MainRoute>>
            }),
            registered_only: false,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,
            link_class: None,

            submenu_tag: None,
            submenu: None,

            title: "The About Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "About".to_owned(),
            url: None,
            menu_tag: "main-about".to_owned(),
            sub_menu_tag: "".to_owned(),
        },
        MenuItem {
            hardcoded: false,
            html: Some( html!{
                <Link<MainRoute>
                    to={MainRoute::Tech}
                >
                    <i class="fa fa-microchip" /><Nbsp />
                    {"Tech"}
                </Link<MainRoute>>
            }),
            registered_only: false,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,

            link_class: None,

            submenu_tag: None,
            submenu: None,

            title: "The Technology Used Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "Tech".to_owned(),
            url: None,
            menu_tag: "main-tech".to_owned(),
            sub_menu_tag: "".to_owned(),
        },
        MenuItem {
            hardcoded: false,
            html: Some( html!{
                <Link<MainRoute>
                    to={MainRoute::ToDos}
                >
                    <i class="fa fa-list" /><Nbsp />
                    {"To-Dos"}
                </Link<MainRoute>>
            }),
            registered_only: false,
            wildcard_only: false,
            developer_only: false,
            admin_only: false,

            link_class: None,

            submenu_tag: None,
            submenu: None,

            title: "The To-Dos Page".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "To-Dos".to_owned(),
            url: None,

            menu_tag: "main-todos".to_owned(),
            sub_menu_tag: "".to_owned(),
        },

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

            submenu_tag: Some("user".to_owned()),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "The Private Settings Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Private Settings".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_private".to_owned(),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "The Public Settings Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Public Settings".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_public".to_owned(),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "The Notifications Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Notifications".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_notifications".to_owned(),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "The Subscription Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Subscription".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_subscription".to_owned(),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "The Devices Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Devices Settings".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_devices".to_owned(),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "The API Key Page".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "API Key".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_apikey".to_owned(),
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

                        submenu_tag: None,
                        submenu: None,

                        title: "Click here to log out".to_owned(),
                        icon_class: None, // "fa fa-house".to_owned(),
                        label: "Logout".to_owned(),
                        url: None,

                        menu_tag: "user-login".to_owned(),
                        sub_menu_tag: "settings_logout".to_owned(),
                    },
                ]
            ),

            title: "Settings".to_owned(),
            icon_class: None, // "fa fa-house".to_owned(),
            label: "Settings".to_owned(),
            url: None,

            menu_tag: "user-login".to_owned(),
            sub_menu_tag: "".to_owned(),
        },
    );

}



pub fn user_can_see_menu_item(
    user: &User,
    item: &MenuItem,
) -> bool {

    if !item.registered_only
        && !item.wildcard_only
        && !item.developer_only
        && !item.admin_only
    {
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
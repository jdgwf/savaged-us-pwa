use yew_router::prelude::*;
use yew::prelude::*;

use yew::{function_component, html};

// use savaged_libs::user::User;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::main_app::SubmenuData;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;
use super::settings_public::SettingsPublic;
use super::settings_private::SettingsPrivate;
use super::settings_devices::SettingsDevices;
use super::settings_api_key::SettingsAPIKey;
use super::subscription::UserSubscription;
use super::notifications::UserNotifications;
// use gloo_console::log;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Clone, Routable, PartialEq)]
pub enum UserRoute {
    #[at("/me/settings-private")]
    SettingsPrivate,
    #[at("/me/settings-public")]
    SettingsPublic,
    #[at("/me/devices")]
    Devices,
    #[at("/me/notifications")]
    Notifications,
    #[at("/me/subscription")]
    Subscription,
    #[at("/me/api-key")]
    SettingsAPIKey,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: UserRoute,
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
    open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
) -> Html {

    match routes {

        UserRoute::SettingsAPIKey => html! {
            <SettingsAPIKey
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
    },

        UserRoute::SettingsPrivate => html! {
            <SettingsPrivate
                global_vars={global_vars}
                update_global_vars={update_global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        UserRoute::SettingsPublic => html! {
            <SettingsPublic
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        UserRoute::Devices => html! {
            <SettingsDevices
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        UserRoute::Notifications => html! {
            <UserNotifications
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },
        UserRoute::Subscription => html! {
            <UserSubscription
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },
        UserRoute::NotFound => html! { <h1>{ "UserRoute 404" }</h1> },
    }
}

pub fn top_menu_switch(
    routes: UserRoute,
    global_vars: GlobalVars,
    onclick: &yew::Callback<yew::MouseEvent>,
) -> Html {
    let mut private_class_active = "".to_owned();
    let mut public_class_active = "".to_owned();
    let mut notifications_class_active = "".to_owned();
    let mut devices_class_active = "".to_owned();
    let mut subscriptions_class_active = "".to_owned();
    let mut api_class_active = "".to_owned();

    // log!("top_menu_switch", global_vars.current_user.unread_notifications);
    match routes {
        UserRoute::SettingsAPIKey => {
            api_class_active = "active".to_owned();

        },
        UserRoute::SettingsPrivate => {
            private_class_active = "active".to_owned();

        },
        UserRoute::Notifications => {
            notifications_class_active = "active".to_owned();
        },
        UserRoute::Subscription => {
            subscriptions_class_active = "active".to_owned();
        },

        UserRoute::SettingsPublic => {
            public_class_active = "active".to_owned();
        },

        UserRoute::Devices => {
            devices_class_active = "active".to_owned();
        },
        UserRoute::NotFound => {

        },
    }

    html! {
        <ul id={"user-menu"} class={"sub-menu"}>
            <li class={private_class_active}>
                <Link<UserRoute> to={UserRoute::SettingsPrivate}><i class={"fa-solid fa-user-secret"}></i><Nbsp />{"Private Settings"}</Link<UserRoute>>
            </li>
            <li class={public_class_active}>
                <Link<UserRoute> to={UserRoute::SettingsPublic}><i class={"fa-solid fa-globe"}></i><Nbsp />{"Public Settings"}</Link<UserRoute>>
            </li>
            <li class={"position-relative ".to_owned() + &notifications_class_active}>
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
            </li>
            <li class={subscriptions_class_active}>
                <Link<UserRoute> to={UserRoute::Subscription}><i class={"fa-solid fa-credit-card"}></i><Nbsp />{"Subscriptions"}</Link<UserRoute>>
            </li>
            <li class={devices_class_active}>
                <Link<UserRoute> to={UserRoute::Devices}><i class={"fa-solid fa-computer"}></i><Nbsp />{"Devices"}</Link<UserRoute>>
            </li>

            if global_vars.current_user.is_premium {
                <li class={api_class_active}>
                    <Link<UserRoute> to={UserRoute::SettingsAPIKey}><i class={"fa-solid fa-key"}></i><Nbsp />{"API Key"}</Link<UserRoute>>
                </li>
            }

            <li class={"logout-item"}>
                <a {onclick}>
                    <i class={"fa-solid fa-right-from-bracket"}></i>
                    <Nbsp />{"Log Out"}</a>
            </li>
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct UserRouterProps {
    #[prop_or_default]
    pub set_submenu: Callback<SubmenuData>,
    pub on_logout_action: Callback<MouseEvent>,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub struct UserRouterMessage {

}

pub struct UserRouter {
    global_vars: GlobalVars,
}

impl Component for UserRouter {
    type Message = UserRouterMessage;
    type Properties = UserRouterProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        let mut submenu_data =
            SubmenuData {
                html: html! (<></>),
                menu: "user-menu-no-user".to_owned(),
                unread_notifications: ctx.props().global_vars.current_user.unread_notifications,
            };

        if ctx.props().global_vars.current_user.id > 0  {
            let on_logout_action = ctx.props().on_logout_action.clone();

            // log!("user_router", global_vars1.current_user.unread_notifications);

            submenu_data.menu = "user-menu".to_owned();
            submenu_data.html = html! {
                <BrowserRouter>
                    <Switch<UserRoute>
                        render={
                            move |routes|
                            top_menu_switch(
                                routes,
                                global_vars.clone(),
                                &on_logout_action,
                            )
                        }

                    />
                </BrowserRouter>
            };

        }
        let _ = ctx.props().set_submenu.emit( submenu_data.clone() );

        let global_vars = ctx.props().global_vars.clone();
        UserRouter {
            global_vars: global_vars.clone(),
        }
    }

    // fn update(
    //     &mut self,
    //    ctx: &Context<Self>,
    //     msg: UserRouterMessage,
    // ) -> bool {

    //     let global_vars = ctx.props().global_vars.clone();

    //     let mut submenu_data =
    //         SubmenuData {
    //             html: html! (<></>),
    //             menu: "user-menu-no-user".to_owned(),
    //             unread_notifications: ctx.props().global_vars.current_user.unread_notifications,
    //         };

    //     if ctx.props().global_vars.current_user.id > 0  {
    //         let on_logout_action = ctx.props().on_logout_action.clone();

    //         // log!("user_router", global_vars1.current_user.unread_notifications);

    //         submenu_data.menu = "user-menu".to_owned();
    //         submenu_data.html = html! {
    //             <BrowserRouter>
    //                 <Switch<UserRoute>
    //                     render={
    //                         move |routes|
    //                         top_menu_switch(
    //                             routes,
    //                             global_vars.clone(),
    //                             &on_logout_action,
    //                         )
    //                     }

    //                 />
    //             </BrowserRouter>
    //         };

    //     }
    //     let _ = ctx.props().set_submenu.emit( submenu_data.clone() );

    //     true

    // }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserRouterProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();

        let mut submenu_data =
            SubmenuData {
                html: html! (<></>),
                menu: "user-menu-no-user".to_owned(),
                unread_notifications: self.global_vars.current_user.unread_notifications,
            };

        if ctx.props().global_vars.current_user.id > 0  {
            let global_vars = self.global_vars.clone();
            let on_logout_action = ctx.props().on_logout_action.clone();

            // log!("user_router", global_vars1.current_user.unread_notifications);

            submenu_data.menu = "user-menu".to_owned();
            submenu_data.html = html! {
                <BrowserRouter>
                    <Switch<UserRoute>
                        render={
                            move |routes|
                            top_menu_switch(
                                routes,
                                global_vars.clone(),
                                &on_logout_action,
                            )
                        }

                    />
                </BrowserRouter>
            };

        }
        let _ = ctx.props().set_submenu.emit( submenu_data.clone() );

        true
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {
    let update_global_vars = ctx.props().update_global_vars.clone();
    let open_confirmation_dialog = ctx.props().open_confirmation_dialog.clone();
        let global_vars = ctx.props().global_vars.clone();
        html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<UserRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    global_vars.clone(),
                                    update_global_vars.clone(),
                                    open_confirmation_dialog.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
        }
    }
}
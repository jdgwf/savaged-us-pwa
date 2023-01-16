pub mod login;
pub mod settings_public;
pub mod settings_private;
pub mod settings_devices;
mod subscription;
mod notifications;
pub mod forgot_password;
pub mod register;
pub mod settings_api_key;
pub mod campaigns;
pub mod saves;

use yew_router::prelude::*;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};

use yew::{function_component, html};

// use savaged_libs::user::User;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::main_app::SubmenuData;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;
use settings_public::SettingsPublic;
use settings_private::SettingsPrivate;
use settings_devices::SettingsDevices;
use settings_api_key::SettingsAPIKey;
use subscription::UserSubscription;
use notifications::UserNotifications;
use saves::list::UserSavesList;
use saves::UserSavesRouter;
use campaigns::UserCampaigns;
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

    #[at("/me/saves/*")]
    UserSavesRouter,

    #[at("/me/saves")]
    UserSavesList,

    #[at("/me/campaigns")]
    UserCampaigns,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: UserRoute,
    global_vars: GlobalVars,
) -> Html {

    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_sub_menu = "user".to_owned();
    } else {
        global_vars.current_sub_menu = "".to_owned();
    }

    match routes {

        UserRoute::UserCampaigns => html! {
            <UserCampaigns
                global_vars={global_vars}
            />
        },
        UserRoute::UserSavesList => html! {
            <UserSavesList
                global_vars={global_vars}
            />
        },
        UserRoute::UserSavesRouter => html! {
            <UserSavesRouter
                global_vars={global_vars}
            />
        },
        UserRoute::SettingsAPIKey => html! {
            <SettingsAPIKey
                global_vars={global_vars}
            />
        },

        UserRoute::SettingsPrivate => html! {
            <SettingsPrivate
                global_vars={global_vars}
            />
        },

        UserRoute::SettingsPublic => html! {
            <SettingsPublic
                global_vars={global_vars}
            />
        },

        UserRoute::Devices => html! {
            <SettingsDevices
                global_vars={global_vars}
            />
        },

        UserRoute::Notifications => html! {
            <UserNotifications
                global_vars={global_vars}
            />
        },
        UserRoute::Subscription => html! {
            <UserSubscription
                global_vars={global_vars}
            />
        },
        UserRoute::NotFound => html! { <h1>{ "UserRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct UserRouterProps {
    #[prop_or_default]
    // pub on_logout_action: Callback<MouseEvent>,
    pub global_vars: GlobalVars,
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

        UserRouter {
            global_vars: global_vars.clone(),
        }
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserRouterProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();

        true
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {
        let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
        let open_confirmation_dialog = ctx.props().global_vars.open_confirmation_dialog.clone();

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let global_vars = ctx.props().global_vars.clone();

            html! {

                <Router
                    history={history}
                >
                    <div class={"main-content"}>
                        <Switch<UserRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    global_vars.clone(),
                                )
                            }
                        />
                    </div>
                </Router>
        }
        } else {
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
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }

    }
}
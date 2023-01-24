mod notifications;
mod subscription;
pub mod campaigns;
pub mod forgot_password;
pub mod login;
pub mod register;
pub mod saves;
pub mod settings_api_key;
pub mod settings_devices;
pub mod settings_private;
pub mod settings_public;
use campaigns::UserCampaigns;
use crate::libs::global_vars::GlobalVars;
use crate::pages::error404::Error404;
use notifications::UserNotifications;
use saves::UserSavesRouter;
use saves::list::UserSavesList;
use settings_api_key::SettingsAPIKey;
use settings_devices::SettingsDevices;
use settings_private::SettingsPrivate;
use settings_public::SettingsPublic;
use subscription::UserSubscription;
use yew::prelude::*;
use yew::{ html};
use yew_router::prelude::*;

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
        UserRoute::NotFound => html! {
            <Error404
                global_vars={global_vars}
            />
        },
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
}

impl Component for UserRouter {
    type Message = UserRouterMessage;
    type Properties = UserRouterProps;

    fn create(
        _ctx: &Context<Self>
    ) -> Self {

        UserRouter {
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {
        // let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
        // let open_confirmation_dialog = ctx.props().global_vars.open_confirmation_dialog.clone();

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let mut global_vars = ctx.props().global_vars.clone();
            global_vars.current_menu = "main-register".to_string();
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
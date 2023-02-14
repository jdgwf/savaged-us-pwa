pub mod campaigns;
pub mod forgot_password;
pub mod login;
mod notifications;
pub mod register;
pub mod saves;
pub mod settings_api_key;
pub mod settings_devices;
pub mod settings_private;
pub mod settings_public;
mod subscription;
use crate::libs::global_vars::GlobalVars;
use crate::libs::site_vars::SiteVars;
use crate::pages::error404::Error404;
use campaigns::UserCampaigns;
use notifications::UserNotifications;
use saves::list::UserSavesList;
use saves::UserSavesRouter;
use settings_api_key::SettingsAPIKey;
use settings_devices::SettingsDevices;
use settings_private::SettingsPrivate;
use settings_public::SettingsPublic;
use subscription::UserSubscription;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;
use savaged_libs::{
    player_character::game_data_package::GameDataPackage,
    save_db_row::SaveDBRow,
};

#[derive(Clone, PartialEq, Routable)]
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
    let mut site_vars = global_vars.site_vars.clone();

    if site_vars.current_user.id > 0 {
        site_vars.current_sub_menu = "user".to_owned();
    } else {
        site_vars.current_sub_menu = "".to_owned();
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
                site_vars={site_vars}
            />
        },

        UserRoute::SettingsPrivate => html! {
            <SettingsPrivate
                site_vars={site_vars}
            />
        },

        UserRoute::SettingsPublic => html! {
            <SettingsPublic
                site_vars={site_vars}
            />
        },

        UserRoute::Devices => html! {
            <SettingsDevices
                site_vars={site_vars}
            />
        },

        UserRoute::Notifications => html! {
            <UserNotifications
                site_vars={site_vars}
            />
        },
        UserRoute::Subscription => html! {
            <UserSubscription
                site_vars={site_vars}
            />
        },
        UserRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct UserRouterProps {
    pub global_vars: GlobalVars,
}

pub struct UserRouterMessage {}

pub struct UserRouter {}

impl Component for UserRouter {
    type Message = UserRouterMessage;
    type Properties = UserRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UserRouter {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let update_site_vars = ctx.props().site_vars.update_site_vars.clone();
        // let open_confirmation_dialog = ctx.props().site_vars.open_confirmation_dialog.clone();

        if ctx.props().global_vars.site_vars.server_side_renderer {
            let history = ctx
                .props()
                .global_vars
                .site_vars
                .server_side_renderer_history
                .as_ref()
                .unwrap()
                .clone();
            let mut global_vars = ctx.props().global_vars.clone();
            global_vars.site_vars.current_menu = "main-register".to_string();
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
            let mut global_vars = ctx.props().global_vars.clone();
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

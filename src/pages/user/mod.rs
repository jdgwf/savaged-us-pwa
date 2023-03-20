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
    mut site_vars: &SiteVars,
    game_data: &Option<GameDataPackage>,
    saves: &Option<Vec<SaveDBRow>>,
) -> Html {
    let mut site_vars = site_vars.clone();

    if site_vars.current_user.id > 0 {
        site_vars.current_sub_menu = "user".to_owned();
    } else {
        site_vars.current_sub_menu = "".to_owned();
    }

    match routes {
        UserRoute::UserCampaigns => html! {
            <UserCampaigns
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
            />
        },
        UserRoute::UserSavesList => html! {
            <UserSavesList
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
            />
        },
        UserRoute::UserSavesRouter => html! {
            <UserSavesRouter
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
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
    pub site_vars: SiteVars,
    pub game_data: Option<GameDataPackage>,
    pub saves: Option<Vec<SaveDBRow>>,
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

        let site_vars = ctx.props().site_vars.clone();
        let game_data = ctx.props().game_data.clone();
        let saves = ctx.props().saves.clone();
        if ctx.props().site_vars.server_side_renderer {
            let history = ctx
                .props()
                .site_vars
                .server_side_renderer_history
                .as_ref()
                .unwrap()
                .clone();

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
                                        &site_vars,
                                        &game_data,
                                        &saves,
                                    )
                                }
                            />
                        </div>
                    </Router>
            }
        } else {

            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<UserRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    &site_vars,
                                    &game_data,
                                    &saves,

                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }
    }
}

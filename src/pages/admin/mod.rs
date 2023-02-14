pub mod game_data;
pub mod home;
pub mod users;

use self::game_data::get_game_data_submenu_items;
use self::users::get_admin_users_submenu_items;
use self::users::list::AdminUsersList;
use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use crate::pages::admin::game_data::home::AdminGameDataHome;
use crate::pages::admin::game_data::AdminGameDataRouter;
use crate::pages::admin::users::AdminUsersRouter;
use crate::pages::error404::Error404;
use home::AdminHome;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AdminRoute {
    #[at("/admin/")]
    AdminHome,

    #[at("/admin/users/*")]
    AdminUsersRouter,

    #[at("/admin/users/")]
    AdminUsersList,

    #[at("/admin/game-data/")]
    AdminGameDataHome,

    #[at("/admin/game-data/*")]
    AdminGameDataRouter,

    #[at("/404")]
    NotFound,
}

fn content_switch(routes: AdminRoute, site_vars: SiteVars) -> Html {
    if site_vars.user_loading || site_vars.server_side_renderer {
        return html! {
            <UIPage
                site_vars={site_vars}

                page_title="Admin"
            >
                <h1>{ "Verifying user..." }</h1>
            </UIPage>
        };
    }

    if !site_vars.current_user.has_developer_access() {
        return html! {
            <UIPage
                site_vars={site_vars}
                page_title="Admin"
            >
                <h1>{ "Access Denied" }</h1>
            </UIPage>
        };
    }

    match routes {
        AdminRoute::AdminUsersRouter => html! {
            <AdminUsersRouter
                site_vars={site_vars}
            />
        },

        AdminRoute::AdminUsersList => html! {
            <AdminUsersList
                site_vars={site_vars}
                sub_menu_items={get_admin_users_submenu_items()}
            />
        },

        AdminRoute::AdminGameDataHome => {
            let sub_menu_items = get_game_data_submenu_items();

            html! {
                <AdminGameDataHome
                    site_vars={site_vars}
                    sub_menu_items={sub_menu_items}
                />
            }
        }

        AdminRoute::AdminGameDataRouter => html! {
            <AdminGameDataRouter
                site_vars={site_vars}
            />
        },
        AdminRoute::AdminHome => html! {
            <AdminHome
                site_vars={site_vars}
            />

        },

        AdminRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminRouterProps {
    pub site_vars: SiteVars,
}

pub enum AdminRouterMessage {}
pub struct AdminRouter {}

impl Component for AdminRouter {
    type Message = AdminRouterMessage;
    type Properties = AdminRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        AdminRouter {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().site_vars.clone();
        site_vars.current_menu = "main-admin".to_owned();

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
                            <Switch<AdminRoute>
                                render={
                                    move |routes|
                                    content_switch(
                                        routes,
                                        site_vars.clone(),
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
                        <Switch<AdminRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    site_vars.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }
    }
}

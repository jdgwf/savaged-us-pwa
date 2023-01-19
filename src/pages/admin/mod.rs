pub mod game_data;
pub mod home;
pub mod users;

use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::game_data::AdminGameDataRouter;
use crate::pages::admin::game_data::home::AdminGameDataHome;
use crate::pages::admin::users::AdminUsersRouter;
use crate::pages::error404::Error404;
use home::AdminHome;
use self::game_data::get_game_data_submenu_items;
use self::users::list::AdminUsersList;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
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

fn content_switch(
    routes: AdminRoute,
    global_vars: GlobalVars,
) -> Html {

    if global_vars.user_loading || global_vars.server_side_renderer {
        return html! {
            <UIPage
                global_vars={global_vars}
                page_title="Admin"
            >
                <h1>{ "Verifying user..." }</h1>
            </UIPage>
        }
    }

    if !global_vars.current_user.has_developer_access() {
        return html! {
            <UIPage
                global_vars={global_vars}
                page_title="Admin"
            >
                <h1>{ "Access Denied" }</h1>
            </UIPage>
        }
    }

    match routes {

        AdminRoute::AdminUsersRouter => html! {
            <AdminUsersRouter
                global_vars={global_vars}
            />
        },

        AdminRoute::AdminUsersList => html! {
            <AdminUsersList
                global_vars={global_vars}
            />
        },

        AdminRoute::AdminGameDataHome => {
            let sub_menu_items = get_game_data_submenu_items();

            html! {
                <AdminGameDataHome
                    global_vars={global_vars}
                    sub_menu_items={sub_menu_items}
                />
            }
        },

        AdminRoute::AdminGameDataRouter => html! {
            <AdminGameDataRouter
                global_vars={global_vars}
            />
        },
        AdminRoute::AdminHome => html! {
            <AdminHome
                global_vars={global_vars}
            />

        },

        AdminRoute::NotFound => html! {
            <Error404
                global_vars={global_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminRouterProps {
    pub global_vars: GlobalVars,
}

pub enum AdminRouterMessage {

}
pub struct AdminRouter {
}

impl Component for AdminRouter {
    type Message = AdminRouterMessage;
    type Properties = AdminRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {

        AdminRouter {
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let global_vars = ctx.props().global_vars.clone();

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
                        <Switch<AdminRoute>
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


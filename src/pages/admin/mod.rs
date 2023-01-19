pub mod game_data;
pub mod home;
pub mod users;

use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::game_data::AdminGameDataRouter;
use crate::pages::admin::game_data::home::AdminGameDataHome;
use crate::pages::admin::users::AdminUsersRouter;
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

    if !global_vars.current_user.has_developer_access() {
        return html! { <h1>{ "Access Denied" }</h1> }
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

        AdminRoute::NotFound => html! { <h1>{ "AdminRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminRouterProps {
    pub global_vars: GlobalVars,
}

pub enum AdminRouterMessage {

}
pub struct AdminRouter {
    global_vars: GlobalVars,
}

impl Component for AdminRouter {
    type Message = AdminRouterMessage;
    type Properties = AdminRouterProps;

    fn create(ctx: &Context<Self>) -> Self {

        AdminRouter {
            global_vars: ctx.props().global_vars.clone(),
        }
    }

    // fn update(
    //     &mut self, ctx: &Context<Self>,
    //     msg: AdminRouterMessage
    // ) -> bool {

    //     match msg {
    //         // AdminRouterMessage::ChangeFilter( filter_type ) => {
    //         //     // log!("ChangeFilter", filter_type);
    //         //     set_local_storage_string( "saves_filter", filter_type);
    //         // }

    //         // AdminRouterMessage::ChangeFolder( folder_name ) => {
    //         //     // log!("ChangeFolder", folder);
    //         //     set_local_storage_string( "saves_folder", folder_name);
    //         // }
    //     }
    //     // true
    // }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &AdminRouterProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();

        true
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


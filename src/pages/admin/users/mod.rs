pub mod list;

use crate::libs::global_vars::GlobalVars;
use yew::prelude::*;
use yew::{html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AdminUsersRoute {
    // #[at("/admin/users/*")]
    // AdminUsersUsersRouter,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: AdminUsersRoute,
    global_vars: GlobalVars,
) -> Html {

    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_sub_menu = "user".to_owned();
    } else {
        global_vars.current_sub_menu = "".to_owned();
    }

    match routes {

        // AdminUsersRoute::Users => html! {
        //     <AdminUsersList
        //         update_global_vars={update_global_vars}
        //         global_vars={global_vars}
        //         open_confirmation_dialog={open_confirmation_dialog}
        //     />
        // },

        AdminUsersRoute::NotFound => html! { <h1>{ "AdminUsersRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminUsersRouterProps {
    pub global_vars: GlobalVars,
}

pub enum AdminUsersRouterMessage {

}
pub struct AdminUsersRouter {
}

impl Component for AdminUsersRouter {
    type Message = AdminUsersRouterMessage;
    type Properties = AdminUsersRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {

        AdminUsersRouter {
        }
    }

    fn update(
        &mut self,
        _ctx: &Context<Self>,
        msg: AdminUsersRouterMessage
    ) -> bool {

        match msg {
            // AdminUsersRouterMessage::ChangeFilter( filter_type ) => {
            //     // log!("ChangeFilter", filter_type);
            //     set_local_storage_string( "saves_filter", filter_type);
            // }

            // AdminUsersRouterMessage::ChangeFolder( folder_name ) => {
            //     // log!("ChangeFolder", folder);
            //     set_local_storage_string( "saves_folder", folder_name);
            // }
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
                        <Switch<AdminUsersRoute>
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
                        <Switch<AdminUsersRoute>
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


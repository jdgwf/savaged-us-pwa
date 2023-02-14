pub mod list;
pub mod activity;

use crate::components::tertiary_links_menu::TertiaryLinksMenuItem;
use crate::libs::site_vars::SiteVars;
use crate::pages::error404::Error404;
use activity::AdminUsersActivity;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

use super::AdminRoute;

#[derive(Clone, PartialEq, Routable)]
pub enum AdminUsersRoute {
    #[at("/admin/users/activity")]
    Activity,
    #[at("/404")]
    NotFound,
}

fn content_switch(routes: AdminUsersRoute, site_vars: SiteVars) -> Html {
    let mut site_vars = site_vars.clone();

    if site_vars.current_user.id > 0 {
        site_vars.current_sub_menu = "user".to_owned();
    } else {
        site_vars.current_sub_menu = "".to_owned();
    }

    match routes {
        AdminUsersRoute::Activity => html! {
            <AdminUsersActivity
                // update_site_vars={update_site_vars}
                site_vars={site_vars}
                sub_menu_items={get_admin_users_submenu_items()}
                // open_confirmation_dialog={open_confirmation_dialog}
            />
        },
        AdminUsersRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminUsersRouterProps {
    pub site_vars: SiteVars,
}

pub enum AdminUsersRouterMessage {}
pub struct AdminUsersRouter {}

impl Component for AdminUsersRouter {
    type Message = AdminUsersRouterMessage;
    type Properties = AdminUsersRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        AdminUsersRouter {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: AdminUsersRouterMessage) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        if ctx.props().site_vars.server_side_renderer {
            let history = ctx
                .props()
                .site_vars
                .server_side_renderer_history
                .as_ref()
                .unwrap()
                .clone();
            let site_vars = ctx.props().site_vars.clone();

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
                                        site_vars.clone(),
                                    )
                                }
                            />
                        </div>
                    </Router>
            }
        } else {
            let site_vars = ctx.props().site_vars.clone();
            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<AdminUsersRoute>
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

pub fn get_admin_users_submenu_items() -> Vec<TertiaryLinksMenuItem> {
    return vec![
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminRoute> to={AdminRoute::AdminUsersList}>{"Users"}</Link<AdminRoute>>},
            tag: "users-list".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminUsersRoute> to={AdminUsersRoute::Activity}>{"Activity"}</Link<AdminUsersRoute>>},
            tag: "users-activity".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        // TertiaryLinksMenuItem {
        //     link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Edges}>{"Edges"}</Link<AdminGameDataRoute>>},
        //     tag: "edges".to_owned(),
        //     class: None,
        //     title: None,
        //     icon_class: None,
        //     separate: false,
        // },
        // TertiaryLinksMenuItem {
        //     link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Armor}>{"Armor"}</Link<AdminGameDataRoute>>},
        //     tag: "armor".to_owned(),
        //     class: None,
        //     title: None,
        //     icon_class: None,
        //     separate: false,
        // },
        // TertiaryLinksMenuItem {
        //     link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Weapons}>{"Weapons"}</Link<AdminGameDataRoute>>},
        //     tag: "weapons".to_owned(),
        //     class: None,
        //     title: None,
        //     icon_class: None,
        //     separate: false,
        // },
        // TertiaryLinksMenuItem {
        //     link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Gear}>{"Gear"}</Link<AdminGameDataRoute>>},
        //     tag: "gear".to_owned(),
        //     class: None,
        //     title: None,
        //     icon_class: None,
        //     separate: false,
        // },
        // TertiaryLinksMenuItem {
        //     link: html!{<Link to={AdminGameDataRoute::Hindrances}>{"Hindrances"}</Link>},
        //     tag: "hindrances".to_owned(),
        //     class: None,
        //     title: None,
        //     icon_class: None,
        //     separate: false,
        // },
    ];
}

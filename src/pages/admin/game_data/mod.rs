pub mod armor;
pub mod edges;
pub mod gear;
pub mod hindrances;
pub mod home;
pub mod weapons;

use crate::components::tertiary_links_menu::{TertiaryLinksMenuItem};
use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::AdminRoute;
use self::armor::AdminGameDataArmor;
use self::edges::AdminGameDataEdges;
use self::gear::AdminGameDataGear;
use self::hindrances::AdminGameDataHindrances;
use self::weapons::AdminGameDataWeapons;
use yew::prelude::*;
use yew::{html};
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AdminGameDataRoute {

    #[at("/admin/game-data/hindrances")]
    Hindrances,

    #[at("/admin/game-data/edges")]
    Edges,

    #[at("/admin/game-data/gear")]
    Gear,

    #[at("/admin/game-data/armor")]
    Armor,

    #[at("/admin/game-data/weapons")]
    Weapons,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: AdminGameDataRoute,
    global_vars: GlobalVars,
) -> Html {

    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_sub_menu = "user".to_owned();
    } else {
        global_vars.current_sub_menu = "".to_owned();
    }

    let sub_menu_items = get_game_data_submenu_items();

    match routes {

        AdminGameDataRoute::Hindrances => html! {
            <AdminGameDataHindrances
                global_vars={global_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Edges => html! {
            <AdminGameDataEdges
                global_vars={global_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Gear => html! {
            <AdminGameDataGear
                global_vars={global_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Armor => html! {
            <AdminGameDataArmor
                global_vars={global_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Weapons => html! {
            <AdminGameDataWeapons
                global_vars={global_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::NotFound => html! { <h1>{ "AdminGameDataRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminGameDataRouterProps {

    pub global_vars: GlobalVars,
}

pub enum AdminGameDataRouterMessage {
}
pub struct AdminGameDataRouter {
}

impl Component for AdminGameDataRouter {
    type Message = AdminGameDataRouterMessage;
    type Properties = AdminGameDataRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {

        AdminGameDataRouter {
        }
    }

    // fn update(
    //     &mut self, ctx: &Context<Self>,
    //     msg: AdminGameDataRouterMessage
    // ) -> bool {

    //     match msg {

    //         // AdminGameDataRouterMessage::ChangeFolder( folder_name ) => {
    //         //     // log!("ChangeFolder", folder);
    //         //     set_local_storage_string( "saves_folder", folder_name);
    //         // }
    //     }

    //     return true;
    // }

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
                        <Switch<AdminGameDataRoute>
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
                        <Switch<AdminGameDataRoute>
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

pub fn get_game_data_submenu_items() -> Vec<TertiaryLinksMenuItem> {

    return vec![
        TertiaryLinksMenuItem {
            link: html!{<Link<AdminRoute> to={AdminRoute::AdminGameDataHome}>{"Home"}</Link<AdminRoute>>},
            tag: "home".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html!{<Link<AdminGameDataRoute> to={AdminGameDataRoute::Hindrances}>{"Hindrances"}</Link<AdminGameDataRoute>>},
            tag: "hindrances".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html!{<Link<AdminGameDataRoute> to={AdminGameDataRoute::Edges}>{"Edges"}</Link<AdminGameDataRoute>>},
            tag: "edges".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem{
            link: html!{<Link<AdminGameDataRoute> to={AdminGameDataRoute::Armor}>{"Armor"}</Link<AdminGameDataRoute>>},
            tag: "armor".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html!{<Link<AdminGameDataRoute> to={AdminGameDataRoute::Weapons}>{"Weapons"}</Link<AdminGameDataRoute>>},
            tag: "weapons".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html!{<Link<AdminGameDataRoute> to={AdminGameDataRoute::Gear}>{"Gear"}</Link<AdminGameDataRoute>>},
            tag: "gear".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
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


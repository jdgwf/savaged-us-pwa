pub mod armor;
pub mod edges;
pub mod gear;
pub mod hindrances;
pub mod home;
pub mod weapons;
pub mod gear_enhancements;

use self::armor::AdminGameDataArmor;
use self::edges::AdminGameDataEdges;
use self::gear::AdminGameDataGear;
use self::gear_enhancements::AdminGameDataGearEnhancements;
use self::hindrances::AdminGameDataHindrances;
use self::weapons::AdminGameDataWeapons;
use crate::components::tertiary_links_menu::TertiaryLinksMenuItem;
use crate::libs::site_vars::SiteVars;
use crate::pages::admin::AdminRoute;
use crate::pages::error404::Error404;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum AdminGameDataRoute {
    #[at("/admin/game-data/hindrances")]
    Hindrances,

    #[at("/admin/game-data/edges")]
    Edges,

    #[at("/admin/game-data/gear")]
    Gear,

    #[at("/admin/game-data/gear-enhancements")]
    GearEnhancements,

    #[at("/admin/game-data/armor")]
    Armor,

    #[at("/admin/game-data/weapons")]
    Weapons,

    #[at("/404")]
    NotFound,
}

fn content_switch(routes: AdminGameDataRoute, site_vars: SiteVars) -> Html {
    let mut site_vars = site_vars.clone();

    if site_vars.current_user.id > 0 {
        site_vars.current_sub_menu = "user".to_owned();
    } else {
        site_vars.current_sub_menu = "".to_owned();
    }

    let sub_menu_items = get_game_data_submenu_items();

    match routes {
        AdminGameDataRoute::Hindrances => html! {
            <AdminGameDataHindrances
                site_vars={site_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Edges => html! {
            <AdminGameDataEdges
                site_vars={site_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Gear => html! {
            <AdminGameDataGear
                site_vars={site_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::GearEnhancements => html! {
            <AdminGameDataGearEnhancements
                site_vars={site_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Armor => html! {
            <AdminGameDataArmor
                site_vars={site_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::Weapons => html! {
            <AdminGameDataWeapons
                site_vars={site_vars}
                sub_menu_items={sub_menu_items}

            />
        },

        AdminGameDataRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminGameDataRouterProps {
    pub site_vars: SiteVars,
}

pub enum AdminGameDataRouterMessage {}
pub struct AdminGameDataRouter {}

impl Component for AdminGameDataRouter {
    type Message = AdminGameDataRouterMessage;
    type Properties = AdminGameDataRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        AdminGameDataRouter {}
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
                            <Switch<AdminGameDataRoute>
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
                        <Switch<AdminGameDataRoute>
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

pub fn get_game_data_submenu_items() -> Vec<TertiaryLinksMenuItem> {
    return vec![
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminRoute> to={AdminRoute::AdminGameDataHome}>{"Home"}</Link<AdminRoute>>},
            tag: "home".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Hindrances}>{"Hindrances"}</Link<AdminGameDataRoute>>},
            tag: "hindrances".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Edges}>{"Edges"}</Link<AdminGameDataRoute>>},
            tag: "edges".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Armor}>{"Armor"}</Link<AdminGameDataRoute>>},
            tag: "armor".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Weapons}>{"Weapons"}</Link<AdminGameDataRoute>>},
            tag: "weapons".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::Gear}>{"Gear"}</Link<AdminGameDataRoute>>},
            tag: "gear".to_owned(),
            class: None,
            title: None,
            icon_class: None,
            separate: false,
        },
        TertiaryLinksMenuItem {
            link: html! {<Link<AdminGameDataRoute> to={AdminGameDataRoute::GearEnhancements}>{"Gear Enhancements"}</Link<AdminGameDataRoute>>},
            tag: "gear-enhancements".to_owned(),
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

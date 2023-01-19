use yew::{function_component, Properties, Html, html};
use yew_router::prelude::Link;
use crate::components::tertiary_links_menu::{TertiaryLinksMenuItem, TertiaryLinksMenu};

use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::game_data::AdminGameDataRoute;
use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct AdminGameDataHomeProps {
    pub global_vars: GlobalVars,
    pub sub_menu_items: Vec<TertiaryLinksMenuItem>,

}
#[function_component(AdminGameDataHome)]
pub fn admin_game_data_home(
    props: &AdminGameDataHomeProps,
) -> Html {

    let mut global_vars = props.global_vars.clone();
    global_vars.current_sub_menu = "admin-game-data".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Game Data Administration"
        submenu_tag={"admin".to_owned()}
    >

    <TertiaryLinksMenu
        server_side_renderer={props.global_vars.server_side_renderer}
        menu_items={props.sub_menu_items.clone()}

        current_tag={"home".to_owned()}
    />
            <h2><i class="fa fa-dice" /><Nbsp />{"Game Data Administration"}</h2>

            <div class="admin-home-links">
                <Link<AdminGameDataRoute> to={AdminGameDataRoute::Hindrances}>
                    {"Hindrances"}
                </Link<AdminGameDataRoute>>

                <Link<AdminGameDataRoute> to={AdminGameDataRoute::Edges}>
                    {"Edges"}
                </Link<AdminGameDataRoute>>

                <Link<AdminGameDataRoute> to={AdminGameDataRoute::Armor}>
                    {"Armor"}
                </Link<AdminGameDataRoute>>

                <Link<AdminGameDataRoute> to={AdminGameDataRoute::Gear}>
                    {"Gear"}
                </Link<AdminGameDataRoute>>

                <Link<AdminGameDataRoute> to={AdminGameDataRoute::Weapons}>
                    {"Weapons"}
                </Link<AdminGameDataRoute>>
            </div>
        </UIPage>
    }
}


use yew::{function_component, Properties, Html, html};
use yew_router::prelude::Link;
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::game_data::AdminGameDataRoute;

use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct AdminGameDataHomeProps {
    pub global_vars: GlobalVars,
}
#[function_component(AdminGameDataHome)]
pub fn info_partners(
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
            <h2><i class="fa fa-dice" /><Nbsp />{"Game Data Administration"}</h2>

            <Link<AdminGameDataRoute> to={AdminGameDataRoute::Hindrances}>{"Hindrances"}</Link<AdminGameDataRoute>>
        </UIPage>
    }
}


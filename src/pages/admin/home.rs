use yew::{function_component, Properties, Html, html};
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;

use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct AdminHomeProps {
    pub global_vars: GlobalVars,
}

#[function_component(AdminHome)]
pub fn admin_home(
    props: &AdminHomeProps,
) -> Html {

    let mut global_vars = props.global_vars.clone();
    global_vars.current_menu = "main-admin".to_owned();
    global_vars.current_sub_menu = "admin-home".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Admin Home"
        submenu_tag={"admin".to_owned()}
    >
            <h2><i class="fa fa-microchip" /><Nbsp />{"Admin Home"}</h2>
            <p class="text-center"><strong>{"Version"}<Nbsp />{env!("CARGO_PKG_VERSION")}</strong></p>

        </UIPage>
    }
}


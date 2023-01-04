use yew::{function_component, Properties, Html, html};
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;

use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct InfoPartnersProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoPartners)]
pub fn info_partners(
    props: &InfoPartnersProps,
) -> Html {

    let mut global_vars = props.global_vars.clone();
    global_vars.current_sub_menu = "info-partners".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Our Partners"
        submenu_tag={"info".to_owned()}
    >
            <h2><i class="fa fa-handshake" /><Nbsp />{"Partners TODO"}</h2>

        </UIPage>
    }
}


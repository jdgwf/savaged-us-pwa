use yew::{function_component, Properties, Html, html};
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;

use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct InfoContactUsProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoContactUs)]
pub fn info_partners(
    props: &InfoContactUsProps,
) -> Html {

    let mut global_vars = props.global_vars.clone();
    global_vars.current_sub_menu = "info-contact-us".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Contact Us"
        submenu_tag={"info".to_owned()}
    >
            <h2><i class="fa fa-envelope" /><Nbsp />{"Contact Us TODO"}</h2>

        </UIPage>
    }
}


use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::ui::nbsp::Nbsp;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct AdminHomeProps {
    pub site_vars: SiteVars,
}

#[function_component(AdminHome)]
pub fn admin_home(props: &AdminHomeProps) -> Html {
    let mut site_vars = props.site_vars.clone();
    site_vars.current_menu = "main-admin".to_owned();
    site_vars.current_sub_menu = "admin-home".to_owned();

    html! {
    <UIPage
        site_vars={site_vars}
        page_title="Admin Home"
    >
            <h2><i class="fa fa-microchip" /><Nbsp />{"Admin Home"}</h2>
            <p class="text-center"><strong>{"Version"}<Nbsp />{env!("CARGO_PKG_VERSION")}</strong></p>

        </UIPage>
    }
}

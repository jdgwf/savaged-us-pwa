use crate::components::tertiary_links_menu::TertiaryLinksMenu;
use crate::components::{ui_page::UIPage, tertiary_links_menu::TertiaryLinksMenuItem};
use crate::libs::global_vars::GlobalVars;
use crate::libs::site_vars::SiteVars;
use standard_components::ui::nbsp::Nbsp;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct AdminUsersActivityProps {
    pub site_vars: SiteVars,
    pub sub_menu_items: Vec<TertiaryLinksMenuItem>,
}

#[function_component(AdminUsersActivity)]
pub fn admin_home(props: &AdminUsersActivityProps) -> Html {
    let mut site_vars = props.site_vars.clone();
    site_vars.current_menu = "main-admin".to_owned();
    site_vars.current_sub_menu = "admin-users".to_owned();
    let site_vars_server_side_renderer = site_vars.server_side_renderer;
    html! {
    <UIPage
        site_vars={site_vars.clone()}

        page_title="Admin Users Activity"
    >
            <TertiaryLinksMenu
                server_side_renderer={site_vars_server_side_renderer}
                menu_items={props.sub_menu_items.clone()}

                current_tag={"users-activity".to_owned()}
            />
            <h2><i class="fa fa-users" /><Nbsp />{"Current Users' Activity"}</h2>

        </UIPage>
    }
}

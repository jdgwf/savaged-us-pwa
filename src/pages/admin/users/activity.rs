use crate::components::tertiary_links_menu::TertiaryLinksMenu;
use crate::components::{ui_page::UIPage, tertiary_links_menu::TertiaryLinksMenuItem};
use crate::libs::global_vars::GlobalVars;
use standard_components::ui::nbsp::Nbsp;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct AdminUsersActivityProps {
    pub global_vars: GlobalVars,
    pub sub_menu_items: Vec<TertiaryLinksMenuItem>,
}

#[function_component(AdminUsersActivity)]
pub fn admin_home(props: &AdminUsersActivityProps) -> Html {
    let mut global_vars = props.global_vars.clone();
    global_vars.current_menu = "main-admin".to_owned();
    global_vars.current_sub_menu = "admin-users".to_owned();
    let global_vars_server_side_renderer = global_vars.server_side_renderer;
    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Admin Users Activity"
    >
            <TertiaryLinksMenu
                server_side_renderer={global_vars_server_side_renderer}
                menu_items={props.sub_menu_items.clone()}

                current_tag={"users-activity".to_owned()}
            />
            <h2><i class="fa fa-users" /><Nbsp />{"Current Users' Activity"}</h2>

        </UIPage>
    }
}

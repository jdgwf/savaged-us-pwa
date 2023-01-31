use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HelpHomeProps {
    pub global_vars: GlobalVars,
}
#[function_component(HelpHome)]
pub fn info_partners(props: &HelpHomeProps) -> Html {
    let mut global_vars = props.global_vars.clone();
    global_vars.current_menu = "main-help".to_owned();
    global_vars.current_sub_menu = "help-home".to_owned();

    let title = html!{<><i class="fa fa-handshake" /><Nbsp />{"Help"}</>};
    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Help"
    >
        <ContentBox label_html={title}>
            {"Help Section TODO"}
        </ContentBox>
        </UIPage>
    }
}

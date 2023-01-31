use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoContactUsProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoContactUs)]
pub fn info_partners(props: &InfoContactUsProps) -> Html {
    let mut global_vars = props.global_vars.clone();

    global_vars.current_sub_menu = "info-contact-us".to_owned();

    let title = html!{<><i class="fa fa-envelope" /><Nbsp />{"Contact Us"}</>};
    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Contact Us"
    >
        <ContentBox label_html={title}>
        {"TODO"}
        </ContentBox>

        </UIPage>
    }
}

use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoContactUsProps {
    pub site_vars: SiteVars,
}
#[function_component(InfoContactUs)]
pub fn info_partners(props: &InfoContactUsProps) -> Html {
    let mut site_vars = props.site_vars.clone();

    site_vars.current_sub_menu = "info-contact-us".to_owned();

    let title = html!{<><i class="fa fa-envelope" /><Nbsp />{"Contact Us"}</>};
    html! {
    <UIPage
        site_vars={site_vars}
        page_title="Contact Us"
    >
        <ContentBox label_html={title}>
        {"TODO"}
        </ContentBox>

        </UIPage>
    }
}

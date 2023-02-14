use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use savaged_libs::help_article::HelpArticleSection;
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct HelpHomeProps {
    pub site_vars: SiteVars,
    pub tag: String,
    pub section: HelpArticleSection,
}
#[function_component(HelpArticle)]
pub fn info_partners(props: &HelpHomeProps) -> Html {
    let mut site_vars = props.site_vars.clone();
    site_vars.current_menu = "main-help".to_owned();
    site_vars.current_sub_menu = "help-".to_owned() + props.section.as_str();

    let title = html!{<><i class="fa fa-handshake" /><Nbsp />{"Help"}</>};
    html! {
    <UIPage
        site_vars={site_vars}
        page_title="Help"
    >
        <ContentBox label_html={title}>
            {"Help Article: TODO "}<br />
            {format!("Section: {:?}", &props.section)}<br />
            {"Tag: "}{&props.tag}<br />
        </ContentBox>
        </UIPage>
    }
}

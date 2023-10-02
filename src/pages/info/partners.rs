use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use savaged_libs::{partner::SimplePartner, web_content::WebContent};
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoPartnersProps {
    pub site_vars: SiteVars,
    pub web_content: Option<WebContent>,
}

#[function_component(InfoPartners)]
pub fn info_partners(props: &InfoPartnersProps) -> Html {
    let mut site_vars = props.site_vars.clone();
    site_vars.current_sub_menu = "info-partners".to_owned();

    let title = html!{<><i class="fa fa-handshake" /><Nbsp />{"Partners"}</>};

    let mut ace_partners: Vec<SimplePartner> = Vec::new();
    let mut guild_partners: Vec<SimplePartner> = Vec::new();
    match &props.web_content {
        Some( web_content ) => {
            match &web_content.partners {
                Some( partners ) => {
                    ace_partners = partners.clone();
                }
                None => {}
            }
        }
        None => {}
    }

    html! {
    <UIPage
        site_vars={site_vars.clone()}
        page_title="Our Partners"
    >
        <ContentBox label_html={title}>
            {"Ace Partners: "}{ace_partners.len()}
        </ContentBox>
        </UIPage>
    }
}

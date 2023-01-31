use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use savaged_libs::partner::SimplePartner;
use standard_components::ui::{nbsp::Nbsp, content_box::ContentBox};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct InfoPartnersProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoPartners)]
pub fn info_partners(props: &InfoPartnersProps) -> Html {
    let mut global_vars = props.global_vars.clone();
    global_vars.current_sub_menu = "info-partners".to_owned();

    let title = html!{<><i class="fa fa-handshake" /><Nbsp />{"Partners"}</>};

    let mut ace_partners: Vec<SimplePartner> = Vec::new();
    let mut guild_partners: Vec<SimplePartner> = Vec::new();
    match global_vars.web_content.clone() {
        Some( web_content ) => {
            match web_content.partners {
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
        global_vars={global_vars}
        page_title="Our Partners"
    >
        <ContentBox label_html={title}>
            {"Ace Partners: "}{ace_partners.len()}
        </ContentBox>
        </UIPage>
    }
}

use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
// use standard_components::ui::nbsp::Nbsp;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Error404Props {
    pub site_vars: SiteVars,
}
#[function_component(Error404)]
pub fn error_404(props: &Error404Props) -> Html {
    html! {
    <UIPage
        site_vars={props.site_vars.clone()}
        page_title="Page Not Found 😭"
    >
        <div class="error-404">
            <h1>{"4😭4"}</h1>
            <p title="Terrible 404 Haiku"><em>{"Your page was not found"}<br />
            {"Sadness Ensues, but look up"}<br />
            {"The menu can help"}<br />
            </em></p>
        </div>
    </UIPage>
    }
}

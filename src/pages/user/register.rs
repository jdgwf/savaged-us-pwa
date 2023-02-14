use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RegisterProps {
    pub site_vars: SiteVars,
}

pub enum RegisterMessage {}

pub struct Register {
    // site_vars: SiteVars,
}

impl Component for Register {
    type Message = RegisterMessage;
    type Properties = RegisterProps;

    fn create(ctx: &Context<Self>) -> Self {
        let site_vars = ctx.props().site_vars.clone();

        set_document_title(
            site_vars.site_title.to_owned(),
            "Register".to_owned(),
            site_vars.server_side_renderer,
        );
        Register {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().site_vars.clone();
        site_vars.current_menu = "main-register".to_string();
        if site_vars.user_loading {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Register"

                >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            };
        }
        if site_vars.current_user.id > 0 {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Register"

                >
                    <div class={"text-center"}>
                        <br />
                        {"You are already logged in!"}
                    </div>
                </UIPage>
            };
        }
        html! {
            <UIPage
site_vars={site_vars}
                page_title="Register"

            >

                <h2><i class={"fa-solid fa-cogs"}></i><Nbsp />{"TODO: Register"}</h2>
            </UIPage>

        }
    }
}

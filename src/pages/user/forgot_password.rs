use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ForgotPasswordProps {
    pub site_vars: SiteVars,
}

pub enum ForgotPasswordMessage {}

pub struct ForgotPassword {
    // site_vars: SiteVars,
}

impl Component for ForgotPassword {
    type Message = ForgotPasswordMessage;
    type Properties = ForgotPasswordProps;

    fn create(ctx: &Context<Self>) -> Self {
        let site_vars = ctx.props().site_vars.clone();

        set_document_title(
            site_vars.site_title.to_owned(),
            "Recover Password".to_owned(),
            site_vars.server_side_renderer,
        );
        ForgotPassword {
            // site_vars: site_vars,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().site_vars.clone();
        site_vars.current_menu = "main-user-forgot-password".to_string();
        if site_vars.user_loading {
            return html! {
                <UIPage
site_vars={site_vars}
                page_title="Forgot Password"

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
                page_title="Forgot Password"

            >                <div class={"text-center"}>
                    <br />
                    {"You are already logged in!"}
                </div>
                </UIPage>
            };
        }
        html! {
            <UIPage
site_vars={site_vars}
                page_title="Forgot Password"

            >
                <h2><i class={"fa-solid fa-cogs"}></i><Nbsp />{"TODO: Forgot Password"}</h2>
            </UIPage>

        }
    }
}

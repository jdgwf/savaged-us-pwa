use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserSubscriptionProps {
    pub site_vars: SiteVars,
}

pub enum UserSubscriptionMessage {}

pub struct UserSubscription {}

impl Component for UserSubscription {
    type Message = UserSubscriptionMessage;
    type Properties = UserSubscriptionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let site_vars = ctx.props().site_vars.clone();

        set_document_title(
            site_vars.site_title.to_owned(),
            "Subscriptions and Purchases".to_owned(),
            site_vars.server_side_renderer,
        );
        UserSubscription {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: UserSubscriptionMessage) -> bool {
        match msg {

            // SettingsPrivateMessage::UpdateCurrentUser( login_result ) => {

            //     return true;
            // }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let site_vars = ctx.props().site_vars.clone();
        let mut site_vars = ctx.props().site_vars.clone();
        site_vars.current_menu = "main-user-login".to_owned();
        if site_vars.user_loading {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Settings"

                >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            };
        }

        if site_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Settings"

                >
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
                </UIPage>
            };
        }

        site_vars.current_sub_menu = "settings-subscription".to_owned();

        html! {
            <UIPage
site_vars={site_vars}
                page_title="Subscriptions"

            >
                <h2><i class={"fa-solid fa-credit-card"}></i><Nbsp />{"TODO: My Subscriptions and Purchases"}</h2>
            </UIPage>
        }
    }
}

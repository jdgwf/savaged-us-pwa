use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::libs::site_vars::SiteVars;
use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserCampaignsProps {



    // pub update_site_vars: Callback<SiteVars>,
    pub global_vars: GlobalVars,
    // pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub struct UserCampaignsMessage {}
pub struct UserCampaigns {}

impl Component for UserCampaigns {
    type Message = UserCampaignsMessage;
    type Properties = UserCampaignsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UserCampaigns {}
    }

    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

    //     true
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().global_vars.site_vars.clone();
        site_vars.current_menu = "main-my-stuff".to_owned();
        site_vars.current_sub_menu = "user-data".to_owned();
        if site_vars.user_loading {
            return html! {
                <UIPage
                  site_vars={site_vars}

                    page_title="My Saves"

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
                    page_title="My Saves"

                >
                    <div class={"text-center"}>
                        <br />
                        {"You are not logged in!"}
                    </div>

                </UIPage>
            };
        }

        site_vars.current_menu = "main-my-stuff".to_owned();
        site_vars.current_sub_menu = "user-data-campaigns".to_owned();

        html! {
            <UIPage
                site_vars={site_vars}
                page_title="My Campaigns"

            >
                <>{"Campaigns"}</>
            </UIPage>
        }
    }
}

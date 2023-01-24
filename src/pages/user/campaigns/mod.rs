use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserCampaignsProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    // pub update_global_vars: Callback<GlobalVars>,
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
        let mut global_vars = ctx.props().global_vars.clone();
        global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data".to_owned();
        if global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="My Saves"

                >
                    <div class={"text-center"}>
                        <br />
                        {"Loading..."}
                    </div>

                </UIPage>
            };
        }

        if global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="My Saves"

                >
                    <div class={"text-center"}>
                        <br />
                        {"You are not logged in!"}
                    </div>

                </UIPage>
            };
        }

        global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data-campaigns".to_owned();

        html! {
            <UIPage
                global_vars={global_vars}
                page_title="My Campaigns"

            >
                <>{"Campaigns"}</>
            </UIPage>
        }
    }
}

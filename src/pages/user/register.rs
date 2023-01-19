use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RegisterProps {
    pub global_vars: GlobalVars,
}

pub enum RegisterMessage {

}

pub struct Register {
    // global_vars: GlobalVars,
}

impl Component for Register {
    type Message = RegisterMessage;
    type Properties = RegisterProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Register".to_owned(), global_vars.server_side_renderer,);
        Register {
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = ctx.props().global_vars.clone();
        if global_vars.user_loading {
            return html! {
                <UIPage
                global_vars={global_vars.clone()}
                page_title="Register"
                submenu_tag={"".to_owned()}
            >                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            }
        }
        if global_vars.current_user.id > 0 {
            return html! {
                <UIPage
                global_vars={global_vars.clone()}
                page_title="Register"
                submenu_tag={"".to_owned()}
            >                <div class={"text-center"}>
                    <br />
                    {"You are already logged in!"}
                </div>
                </UIPage>
            }
        }
        html! {
            <UIPage
                global_vars={global_vars.clone()}
                page_title="Register"
                submenu_tag={"".to_owned()}
            >

                <h2><i class={"fa-solid fa-cogs"}></i><Nbsp />{"TODO: Register"}</h2>
            </UIPage>

        }

    }
}

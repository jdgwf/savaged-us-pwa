use yew_router::prelude::*;
use yew::prelude::*;

use yew::{function_component, html};

// use savaged_libs::user::User;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::main_app::SubmenuData;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;
use super::tech::InfoTech;
use super::about::InfoAbout;
use super::todos::InfoTodos;

// use gloo_console::log;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Clone, Routable, PartialEq)]
pub enum InfoRoute {
    #[at("/info/about")]
    InfoAbout,

    #[at("/info/tech")]
    InfoTech,

    #[at("/info/to-dos")]
    InfoTodos,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: InfoRoute,
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
    open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
) -> Html {


    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_sub_menu = "user".to_owned();
    } else {
        global_vars.current_sub_menu = "".to_owned();
    }

    match routes {

        InfoRoute::InfoAbout => html! {
            <InfoAbout
                // update_global_vars={update_global_vars}
                global_vars={global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
            />
    },

        InfoRoute::InfoTech => html! {
            <InfoTech
                global_vars={global_vars}
                // update_global_vars={update_global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        InfoRoute::InfoTodos => html! {
            <InfoTodos
                // update_global_vars={update_global_vars}
                global_vars={global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        InfoRoute::NotFound => html! { <h1>{ "InfoRoute 404" }</h1> },
    }
}


#[derive(Properties, PartialEq)]
pub struct InfoRouterProps {
    #[prop_or_default]
    pub set_submenu: Callback<SubmenuData>,
    pub on_logout_action: Callback<MouseEvent>,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub struct InfoRouterMessage {

}

pub struct InfoRouter {
    global_vars: GlobalVars,
}

impl Component for InfoRouter {
    type Message = InfoRouterMessage;
    type Properties = InfoRouterProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        InfoRouter {
            global_vars: global_vars.clone(),
        }
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &InfoRouterProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();

        true
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {
    let update_global_vars = ctx.props().update_global_vars.clone();
    let open_confirmation_dialog = ctx.props().open_confirmation_dialog.clone();
        let global_vars = ctx.props().global_vars.clone();

        html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<InfoRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    global_vars.clone(),
                                    update_global_vars.clone(),
                                    open_confirmation_dialog.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
        }
    }
}
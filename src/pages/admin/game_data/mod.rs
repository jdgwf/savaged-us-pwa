pub mod home;
pub mod hindrances;

use savaged_libs::save_db_row::SaveDBRow;
use yew_router::prelude::*;
use yew::prelude::*;

use yew::{function_component, html};

// use savaged_libs::user::User;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::components::tertiary_menu::{
    TertiaryMenuItem,
    TertiaryMenu
};
use crate::components::ui_page::UIPage;
use crate::main_app::SubmenuData;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;
// use super::settings_public::SettingsPublic;
// use super::settings_private::SettingsPrivate;
// use super::settings_devices::SettingsDevices;
// use super::settings_api_key::SettingsAPIKey;
// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;
use gloo_console::log;

use self::hindrances::AdminGameDataHindrances;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Clone, Routable, PartialEq)]
pub enum AdminGameDataRoute {
    #[at("/admin/game-data/hindrances")]
    Hindrances,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: AdminGameDataRoute,
    global_vars: GlobalVars,
) -> Html {

    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_sub_menu = "user".to_owned();
    } else {
        global_vars.current_sub_menu = "".to_owned();
    }

    match routes {

        AdminGameDataRoute::Hindrances => html! {
            <AdminGameDataHindrances
                global_vars={global_vars}
            />
        },

        AdminGameDataRoute::NotFound => html! { <h1>{ "AdminGameDataRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminGameDataRouterProps {

    pub global_vars: GlobalVars,
}

pub enum AdminGameDataRouterMessage {

}
pub struct AdminGameDataRouter {
    global_vars: GlobalVars,
}

impl Component for AdminGameDataRouter {
    type Message = AdminGameDataRouterMessage;
    type Properties = AdminGameDataRouterProps;

    fn create(ctx: &Context<Self>) -> Self {

        AdminGameDataRouter {
            global_vars: ctx.props().global_vars.clone(),
        }
    }

    fn update(
        &mut self, ctx: &Context<Self>,
        msg: AdminGameDataRouterMessage
    ) -> bool {

        match msg {
            // AdminGameDataRouterMessage::ChangeFilter( filter_type ) => {
            //     // log!("ChangeFilter", filter_type);
            //     set_local_storage_string( "saves_filter", filter_type);
            // }

            // AdminGameDataRouterMessage::ChangeFolder( folder_name ) => {
            //     // log!("ChangeFolder", folder);
            //     set_local_storage_string( "saves_folder", folder_name);
            // }
        }

    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &AdminGameDataRouterProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();



        true
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let global_vars = ctx.props().global_vars.clone();

            html! {

                <Router
                    history={history}
                >
                    <div class={"main-content"}>
                        <Switch<AdminGameDataRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    global_vars.clone(),
                                )
                            }
                        />
                    </div>
                </Router>
        }
        } else {
            let global_vars = ctx.props().global_vars.clone();
            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<AdminGameDataRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    global_vars.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }

    }
}


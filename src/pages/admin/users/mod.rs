pub mod list;

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

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Clone, Routable, PartialEq)]
pub enum AdminUsersRoute {
    // #[at("/admin/users/*")]
    // AdminUsersUsersRouter,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: AdminUsersRoute,
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

        // AdminUsersRoute::Users => html! {
        //     <AdminUsersList
        //         update_global_vars={update_global_vars}
        //         global_vars={global_vars}
        //         open_confirmation_dialog={open_confirmation_dialog}
        //     />
        // },

        AdminUsersRoute::NotFound => html! { <h1>{ "AdminUsersRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminUsersRouterProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum AdminUsersRouterMessage {

}
pub struct AdminUsersRouter {
    global_vars: GlobalVars,
}

impl Component for AdminUsersRouter {
    type Message = AdminUsersRouterMessage;
    type Properties = AdminUsersRouterProps;

    fn create(ctx: &Context<Self>) -> Self {

        AdminUsersRouter {
            global_vars: ctx.props().global_vars.clone(),
        }
    }

    fn update(
        &mut self, ctx: &Context<Self>,
        msg: AdminUsersRouterMessage
    ) -> bool {

        match msg {
            // AdminUsersRouterMessage::ChangeFilter( filter_type ) => {
            //     // log!("ChangeFilter", filter_type);
            //     set_local_storage_string( "saves_filter", filter_type);
            // }

            // AdminUsersRouterMessage::ChangeFolder( folder_name ) => {
            //     // log!("ChangeFolder", folder);
            //     set_local_storage_string( "saves_folder", folder_name);
            // }
        }
        true
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &AdminUsersRouterProps,
    ) -> bool {

        self.global_vars = ctx.props().global_vars.clone();

        // read_notifications: self.global_vars.current_user.unread_notifications,
        //     };

        true
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {
        let update_global_vars = ctx.props().update_global_vars.clone();
        let open_confirmation_dialog = ctx.props().open_confirmation_dialog.clone();

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let global_vars = ctx.props().global_vars.clone();

            html! {

                <Router
                    history={history}
                >
                    <div class={"main-content"}>
                        <Switch<AdminUsersRoute>
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
                </Router>
        }
        } else {
            let global_vars = ctx.props().global_vars.clone();
            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<AdminUsersRoute>
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
}


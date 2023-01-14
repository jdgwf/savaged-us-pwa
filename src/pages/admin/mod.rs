pub mod home;
pub mod users;
pub mod game_data;

use savaged_libs::save_db_row::SaveDBRow;
use yew_router::prelude::*;
use yew::prelude::*;
use yew::{function_component, html};

use home::AdminHome;

use crate::pages::admin::users::AdminUsersRouter;
use crate::pages::admin::game_data::AdminGameDataRouter;
use crate::pages::admin::game_data::home::AdminGameDataHome;
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

use self::users::list::AdminUsersList;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin/")]
    AdminHome,

    #[at("/admin/users/*")]
    AdminUsersRouter,

    #[at("/admin/users/")]
    AdminUsersList,

    #[at("/admin/game-data/")]
    AdminGameDataHome,

    #[at("/admin/game-data/*")]
    AdminGameDataRouter,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: AdminRoute,
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
    open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
) -> Html {


    let mut global_vars = global_vars.clone();



    if !global_vars.current_user.has_developer_access() {
        return html! { <h1>{ "Access Denied" }</h1> }
    }
    match routes {

        AdminRoute::AdminUsersRouter => html! {
            <AdminUsersRouter
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        AdminRoute::AdminUsersList => html! {
            <AdminUsersList
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        AdminRoute::AdminGameDataHome => html! {
            <AdminGameDataHome
                // update_global_vars={update_global_vars}
                global_vars={global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
            />
        },

        AdminRoute::AdminGameDataRouter => html! {
            <AdminGameDataRouter
                update_global_vars={update_global_vars}
                global_vars={global_vars}
                open_confirmation_dialog={open_confirmation_dialog}
            />
        },
        AdminRoute::AdminHome => html! {
            <AdminHome
                // update_global_vars={update_global_vars}
                global_vars={global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
            />


        },


        AdminRoute::NotFound => html! { <h1>{ "AdminRoute 404" }</h1> },
    }
}


#[derive(Properties, PartialEq)]
pub struct AdminRouterProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub enum AdminRouterMessage {

}
pub struct AdminRouter {
    global_vars: GlobalVars,
}

impl Component for AdminRouter {
    type Message = AdminRouterMessage;
    type Properties = AdminRouterProps;

    fn create(ctx: &Context<Self>) -> Self {

        AdminRouter {
            global_vars: ctx.props().global_vars.clone(),
        }
    }


    fn update(
        &mut self, ctx: &Context<Self>,
        msg: AdminRouterMessage
    ) -> bool {


        match msg {
            // AdminRouterMessage::ChangeFilter( filter_type ) => {
            //     // log!("ChangeFilter", filter_type);
            //     set_local_storage_string( "saves_filter", filter_type);
            // }

            // AdminRouterMessage::ChangeFolder( folder_name ) => {
            //     // log!("ChangeFolder", folder);
            //     set_local_storage_string( "saves_folder", folder_name);
            // }
        }
        true
    }


    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &AdminRouterProps,
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
                        <Switch<AdminRoute>
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
                        <Switch<AdminRoute>
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


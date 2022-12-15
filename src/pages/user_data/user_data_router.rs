use yew_router::prelude::*;
use yew::prelude::*;

use yew::{function_component, html};

// use savaged_libs::user::User;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use super::saves::UserDataSaves;
use super::campaigns::UserDataCampaigns;
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
// use gloo_console::log;

// use super::subscription::UserSubscription;
// use super::notifications::UserNotifications;

#[derive(Clone, Routable, PartialEq)]
pub enum UserDataRoute {
    #[at("/my-data/saves")]
    Saves,

    #[at("/my-data/campaigns")]
    Campaigns,
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
pub struct UserDataRouterProps {
    #[prop_or_default]
    pub set_submenu: Callback<SubmenuData>,
    pub on_logout_action: Callback<MouseEvent>,
    pub update_global_vars: Callback<GlobalVars>,
    pub global_vars: GlobalVars,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
}

pub struct UserDataRouterMessage {

}
pub struct UserDataRouter {
    global_vars: GlobalVars,
}

impl Component for UserDataRouter {
    type Message = UserDataRouterMessage;
    type Properties = UserDataRouterProps;

    fn create(ctx: &Context<Self>) -> Self {

        UserDataRouter {
            global_vars: ctx.props().global_vars.clone(),
        }
    }

    // fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {

    //     true
    // }


    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &UserDataRouterProps,
    ) -> bool {
        // log!("main_home changed called" );
        self.global_vars = ctx.props().global_vars.clone();

        // let submenu_data =
        //     SubmenuData {
        //         html: html! (<></>),
        //         menu: "user-menu-no-user".to_owned(),
        //         unread_notifications: self.global_vars.current_user.unread_notifications,
        //     };

        true
    }


    fn view(&self, ctx: &Context<Self>) -> Html {

        if self.global_vars.user_loading {
            return html! {
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
            }
        }

        if self.global_vars.current_user.id == 0 {
            return html! {
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
            }
        }

        let mut global_vars = ctx.props().global_vars.clone();

        global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data-saves".to_owned();

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
                        <Switch<UserDataRoute>
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
                        <Switch<UserDataRoute>
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




fn content_switch(
    routes: UserDataRoute,
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

        UserDataRoute::Saves => html! {
            <UserDataSaves
                // update_global_vars={update_global_vars}
                global_vars={global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
            />
    },

        UserDataRoute::Campaigns => html! {
            <UserDataCampaigns
                global_vars={global_vars}
                // update_global_vars={update_global_vars}
                // open_confirmation_dialog={open_confirmation_dialog}
                // update_global_vars={update_global_vars}
            />
        },


        UserDataRoute::NotFound => html! { <h1>{ "UserDataRoute 404" }</h1> },
    }
}

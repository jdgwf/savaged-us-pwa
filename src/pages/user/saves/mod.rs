pub mod edit;
pub mod view;
pub mod list;

// use savaged_libs::save_db_row::SaveDBRow;
use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::user::saves::list::UserSavesList;
use crate::pages::user::saves::edit::UserSavesEdit;
use crate::pages::user::saves::view::UserSavesView;
use yew::{html};

// use savaged_libs::user::User;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
// use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

// use crate::components::tertiary_menu::{
//     TertiaryMenuItem,
//     TertiaryMenu
// };
// use crate::components::ui_page::UIPage;
// use crate::main_app::SubmenuData;
// use standard_components::ui::nbsp::Nbsp;
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
pub enum UserSavesRoute {
    #[at("/me/saves")]
    List,
    #[at("/me/saves/edit/:uuid")]
    Edit {uuid: String},
    #[at("/me/saves/add/:save_type")]
    Add {save_type: String},
    #[at("/me/saves/view/:uuid")]
    View {uuid: String},
    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: UserSavesRoute,
    global_vars: GlobalVars,
) -> Html {

    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_sub_menu = "user".to_owned();
    } else {
        global_vars.current_sub_menu = "".to_owned();
    }

    match routes {

        UserSavesRoute::List => html! {
            <UserSavesList
                global_vars={global_vars}
            />
        },

        UserSavesRoute::Edit {uuid} => html! {
            <UserSavesEdit
                uuid={uuid}
                global_vars={global_vars}
            />
        },

        UserSavesRoute::Add {save_type} => html! {
            <UserSavesEdit
                uuid={""}
                new_save_type={Some(save_type)}
                global_vars={global_vars}
            />
        },

        UserSavesRoute::View {uuid} => html! {
            <UserSavesView
                uuid={uuid}
                global_vars={global_vars}
            />
        },

        UserSavesRoute::NotFound => html! { <h1>{ "UserSavesRoute 404" }</h1> },
    }
}

#[derive(Properties, PartialEq)]
pub struct UserSavesRouterProps {
    pub global_vars: GlobalVars,
}

pub enum UserSavesRouterMessage {
}
pub struct UserSavesRouter {
    // global_vars: GlobalVars,
}

impl Component for UserSavesRouter {
    type Message = UserSavesRouterMessage;
    type Properties = UserSavesRouterProps;

    fn create(ctx: &Context<Self>) -> Self {

        UserSavesRouter {
            // global_vars: ctx.props().global_vars.clone(),
        }
    }

    // fn update(
    //     &mut self,
    //     _ctx: &Context<Self>,
    //     msg: UserSavesRouterMessage
    // ) -> bool {

    //     match msg {
    //         UserSavesRouterMessage::ChangeFilter( filter_type ) => {
    //             // log!("ChangeFilter", filter_type);
    //             set_local_storage_string( "saves_filter", filter_type);
    //         }

    //         UserSavesRouterMessage::ChangeFolder( folder_name ) => {
    //             // log!("ChangeFolder", folder);
    //             set_local_storage_string( "saves_folder", folder_name);
    //         }
    //     }
    //     true
    // }


    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {
        // let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
        // let open_confirmation_dialog = ctx.props().global_vars.open_confirmation_dialog.clone();

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let global_vars = ctx.props().global_vars.clone();

            html! {

                <Router
                    history={history}
                >
                    <div class={"main-content"}>
                        <Switch<UserSavesRoute>
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
                        <Switch<UserSavesRoute>
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


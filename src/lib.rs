use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};

mod pages;
mod main_app;
mod web_sockets;
mod libs;
mod components;
mod menu_items;
use std::collections::HashMap;
use crate::pages::main_home::MainHome;
use crate::pages::main_about::MainAbout;
use crate::pages::main_tech::MainTech;

use crate::pages::main_todos::MainTodos;
use crate::pages::user::login::UserLogin;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::register::Register;
// use serde_json::Error;
// use standard_components::ui::nbsp::Nbsp;
// use gloo_console::log;
// use main_app_server::MainServerApp;
use crate::libs::global_vars::GlobalVars;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
// use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use savaged_libs::user::User;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    // pub queries: HashMap<String, String>,
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum MainServerRoute {
    #[at("/")]
    Home,
    // #[at("/me/*")]
    // UserRouter,
    #[at("/login")]
    UserLogin,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/register")]
    Register,
    #[at("/about")]
    About,
    #[at("/todos")]
    ToDos,
    #[at("/tech")]
    Tech,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
pub struct MainServerAppProps {
    // pub global_vars: GlobalVars,
}
#[derive(Clone, Debug)]
pub struct SubmenuData {
    pub html: Html,
    pub menu: String,
    pub unread_notifications: u32,
}

#[derive(Debug)]
pub enum MainServerAppMessage {
    SetSubmenu( SubmenuData ),

    // ToggleUserMenu( bool ),
    ToggleMobileMenu(bool),
    HidePopupMenus(bool),

    // UpdateCurrentUser( User ),
    UpdateGlobalVars( GlobalVars ),
    // ContextUpdated( GlobalVarsContext ),
    LogOut( String ),

    // OpenConfirmationDialog(
    //     ConfirmationDialogDefinition,
    // ),
    // CloseConfirmationDialog( MouseEvent ),
}

pub struct MainServerApp {
    // submenu: Html,
    // show_mobile_menu: bool,
    // global_vars_context: GlobalVarsContext,
    // global_vars: GlobalVars,
    // current_unread_notifications: u32,
    // current_sub_menu: String,

    // confirmation_dialog_open: bool,
    // confirmation_dialog_properties: ConfirmationDialogDefinition,
}

fn content_switch(
    routes: MainServerRoute,
    // submenu_callback: &Callback<SubmenuData>,
    global_vars: GlobalVars,
    // on_logout_action: &Callback<MouseEvent>,
    update_global_vars: &Callback<GlobalVars>,
    // open_confirmation_dialog: &Callback<ConfirmationDialogDefinition>,
    // _show_mobile_menu: bool,

) -> Html {

    let open_confirmation_dialog = Callback::noop();
    match routes {
        MainServerRoute::Tech => {
            html!(
                <MainTech
                    global_vars={global_vars}
                />
            )
        },
        MainServerRoute::Home => {

            html! {
                <MainHome
                    global_vars={global_vars}
                />
            }
        },
        MainServerRoute::About => {

            html! {
                <MainAbout
                    global_vars={global_vars}
                />
            }
        },
        MainServerRoute::ToDos => {

            html! {
                <MainTodos
                    global_vars={global_vars}
                />
            }
        },
        // MainServerRoute::UserRouterRedirect => {

        //     html! {
        //         <Redirect<MainServerRoute> to={MainServerRoute::SettingsPrivate} />
        //     }
        // },
        // MainServerRoute::UserRouter => {
        //     html! {
        //         <UserRouter
        //             global_vars={global_vars}
        //             set_submenu={submenu_callback}
        //             on_logout_action={on_logout_action}
        //             update_global_vars={update_global_vars}
        //             open_confirmation_dialog={open_confirmation_dialog}
        //         />
        //     }
        // },
        // MainServerRoute::TestSheetRouterRedirect => {
        //     html! {
        //         <Redirect<MainServerRoute> to={MainServerRoute::TestSheetRouter { sub_route: "home".to_owned() }} />
        //     }
        // }
        MainServerRoute::UserLogin => {
            html! {
                <UserLogin
                    global_vars={global_vars}
                    update_global_vars={update_global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        MainServerRoute::ForgotPassword => {
            html! {
                <ForgotPassword
                    global_vars={global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        MainServerRoute::Register => {
            html! {
                <Register
                    global_vars={global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },

        // MainServerRoute::NotFound => {
        //     // set_document_title(self.global_vars.site_title.to_owned(), " Not Found :(".to_owned());
        //     html! { <h1>{ "MainServerRoute 404" }</h1> }
        // }
        MainServerRoute::NotFound => {
            // set_document_title(self.global_vars.site_title.to_owned(), " Not Found :(".to_owned());
            html! { <h1>{ "MainServerRoute 404" }</h1> }
        }

    }

}

#[function_component]
pub fn ServerApp(
    props: &ServerAppProps
) -> Html {

    // let login_token = get_local_storage_string( "login_token", "".to_owned() );
    // let mut user_loading = true;

    // if login_token.is_empty() {
    //     user_loading = false;
    // }

    // let server_root = "http://localhost:5001".to_owned();
    let server_root = "https://v4/savaged.us".to_owned();
    // let server_root = "https://savaged.us".to_owned();
    // let server_root = "https://staging.savaged.us".to_owned();

    let global_vars_state = use_reducer(
        || GlobalVars {
            login_token:  "".to_owned(),
            current_user: User::default(),
            user_loading: false,
            server_root: server_root.to_owned(),
            api_root: server_root + &"/_api",
            site_title: "v4.savaged.us".to_owned(),
            hide_popup_menus_callback: Callback::noop(),
            no_calls: true,
            offline: true,
            // update_global_vars: Callback::noop(),
            send_websocket: Callback::noop(),
            saves: None,
            show_mobile_menu: false,
            chargen_data: None,
            logout_callback: Callback::noop(),
            toggle_mobile_menu_callback: Callback::noop(),
            current_menu: "".to_string(),
            current_sub_menu: "".to_string(),
        }
    );

    // let active_class = "content-pane";

    let history = AnyHistory::from(MemoryHistory::new());
    let blank_hs: HashMap<String, String> = HashMap::new();
    history
        .push_with_query(&*props.url, &blank_hs)
        .unwrap();

    let global_vars: GlobalVars = (*global_vars_state).clone();
    // let global_vars3: GlobalVars = (*global_vars_state).clone();

    let callback_content =
        move |routes| {
            content_switch(
                routes,
                // &set_submenu,
                global_vars.clone(),
                // &on_logout_action,
                &Callback::noop(),
                // &open_confirmation_dialog,
                // show_mobile_menu,
            )
        }
    ;

    // let submenu = html! { <></> };
    // let mobile_submenu = html! { <></> };

    // let global_vars1: GlobalVars = global_vars.clone();
    // let global_vars2: GlobalVars = global_vars1.clone();

    html! {
        // <ContextProvider<GlobalVarsContext>
        //     context={global_vars_state}
        // >
        <>
            // <>{"Moo?"}</>
            // <MainServerApp />
            <Router
                history={history}
            >

            <Switch<MainServerRoute> render={callback_content} />

            </Router>
        // </ContextProvider<GlobalVarsContext>>
        </>
    }
}

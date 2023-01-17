mod components;
mod libs;
mod local_storage;
mod main_app;
mod menu_items;
mod pages;
mod web_sockets;

pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
use crate::libs::global_vars::GlobalVars;
use crate::pages::admin::AdminRouter;
use crate::pages::info::InfoRouter;
use crate::pages::main_home::MainHome;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::login::UserLogin;
use crate::pages::user::register::Register;
use pages::user::UserRouter;
use savaged_libs::user::User;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    // pub queries: HashMap<String, String>,
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum MainServerRoute {
    #[at("/")]
    Home,
    #[at("/me/*")]
    UserRouter,
    #[at("/login")]
    UserLogin,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/register")]
    Register,
    #[at("/info/*")]
    InfoRouter,
    #[at("/admin/*")]
    AdminRouter,
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

pub struct MainServerApp {

}

fn content_switch(
    routes: MainServerRoute,
    global_vars: GlobalVars,
) -> Html {

    match routes {
        // MainServerRoute::Tech => {
        //     html!(
        //         <MainTech
        //             global_vars={global_vars}
        //         />
        //     )
        // },
        MainServerRoute::Home => {

            html! {
                <MainHome
                    global_vars={global_vars}
                />
            }
        },
        MainServerRoute::InfoRouter => {

            html! {
                <InfoRouter
                    global_vars={global_vars}
                />
            }
        },
        // MainServerRoute::ToDos => {

        //     html! {
        //         <MainTodos
        //             global_vars={global_vars}
        //         />
        //     }
        // },
        // MainServerRoute::UserRouterRedirect => {

        //     html! {
        //         <Redirect<MainServerRoute> to={MainServerRoute::SettingsPrivate} />
        //     }
        // },
        MainServerRoute::UserRouter => {
            html! {
                <UserRouter
                    global_vars={global_vars}
                />
            }
        },

        MainServerRoute::AdminRouter => {
            html! {
                <AdminRouter
                    global_vars={global_vars}
                />
            }
        },

        MainServerRoute::UserLogin => {
            html! {
                <UserLogin
                    global_vars={global_vars}
                />
            }
        },
        MainServerRoute::ForgotPassword => {
            html! {
                <ForgotPassword
                    global_vars={global_vars}

                />
            }
        },
        MainServerRoute::Register => {
            html! {
                <Register
                    global_vars={global_vars}

                />
            }
        },

        MainServerRoute::NotFound => {
            html! { <h1>{ "MainServerRoute 404" }</h1> }
        }

    }

}

#[function_component]
pub fn ServerApp(
    props: &ServerAppProps
) -> Html {

    let server_root = "https://v4/savaged.us".to_owned();

    let global_vars_state = use_reducer(
        || GlobalVars {
            api_root: server_root.to_owned() + &"/_api",
            current_menu: "".to_string(),
            current_sub_menu: "".to_string(),
            current_user: User::default(),
            game_data: None,
            hide_popup_menus_callback: Callback::noop(),
            login_token: "".to_owned(),
            logout_callback: Callback::noop(),
            offline: true,
            add_alert: Callback::noop(),
            open_confirmation_dialog: Callback::noop(),
            saves: None,
            send_websocket: Callback::noop(),
            server_root: server_root.to_owned(),
            server_side_renderer: true,
            server_side_renderer_history: None,
            show_mobile_menu: false,
            site_title: "v4.savaged.us".to_owned(),
            toggle_mobile_menu_callback: Callback::noop(),
            update_global_vars: Callback::noop(),
            user_loading: false,
        }
    );

    let history = AnyHistory::from(MemoryHistory::new());
    let blank_hs: HashMap<String, String> = HashMap::new();
    history
        .push_with_query(&*props.url, &blank_hs)
        .unwrap();

    let mut global_vars: GlobalVars = (*global_vars_state).clone();

    global_vars.server_side_renderer_history = Some(history.clone());
    let callback_content =
        move |routes| {
            content_switch(
                routes,
                global_vars.clone(),
            )
        }
    ;

    html! {

        <>
            <Router
                history={history}
            >

            <Switch<MainServerRoute> render={callback_content} />

            </Router>
        </>
    }
}

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
use crate::pages::error404::Error404;
use crate::pages::info::InfoRouter;
use crate::pages::main_home::MainHome;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::login::UserLogin;
use crate::pages::user::register::Register;
use pages::user::UserRouter;
use savaged_libs::user::User;
use savaged_libs::web_content::WebContent;
use crate::libs::site_vars::SiteVars;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Properties ,PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub web_content: WebContent,
    // pub queries: HashMap<String, String>,
}

#[derive(Clone, PartialEq, Routable, Debug)]
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

pub struct MainServerApp {}

fn content_switch(routes: MainServerRoute, global_vars: GlobalVars) -> Html {
    let site_vars = global_vars.site_vars.clone();
    match routes {
        MainServerRoute::Home => {
            html! {
                <MainHome
                    site_vars={site_vars}
                />
            }
        }
        MainServerRoute::InfoRouter => {
            html! {
                <InfoRouter
                    site_vars={site_vars}
                    web_content={global_vars.web_content}
                />
            }
        }

        MainServerRoute::UserRouter => {
            html! {
                <UserRouter
                    site_vars={site_vars}
                    game_data={None}
                    saves={None}

                />
            }
        }

        MainServerRoute::AdminRouter => {
            html! {
                <AdminRouter
                site_vars={site_vars}
                />
            }
        }

        MainServerRoute::UserLogin => {
            html! {
                <UserLogin
                    site_vars={site_vars}
                    game_data={None}
                    saves={None}
                />
            }
        }
        MainServerRoute::ForgotPassword => {
            html! {
                <ForgotPassword
                    site_vars={site_vars}

                />
            }
        }
        MainServerRoute::Register => {
            html! {
                <Register
                    site_vars={site_vars}

                />
            }
        }

        MainServerRoute::NotFound => {
            html! {
                <Error404
                    site_vars={site_vars}
                />
            }
        }


        MainServerRoute::UserLogin => {
            html! {
                <UserLogin
                    site_vars={site_vars}
                    game_data={global_vars.game_data}
                    saves={global_vars.saves}
                />
            }
        }
        MainServerRoute::ForgotPassword => {
            html! {
                <ForgotPassword
                    site_vars={site_vars}

                />
            }
        }
        MainServerRoute::Register => {
            html! {
                <Register
                    site_vars={site_vars}

                />
            }
        }
    }
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    // let server_root = "https://v4-rust.savaged.us".to_owned();

    // let mut user = User::default();

    // match props.web_content.user.clone() {
    //     Some( found_user ) => {
    //         user = found_user;
    //     }
    //     None => {}
    // }

    let global_vars_state = use_reducer(|| GlobalVars {
        game_data: None,
        saves: None,
        site_vars:SiteVars::default(),
        web_content: Some(props.web_content.clone()),
    });

    let history = AnyHistory::from(MemoryHistory::new());
    let blank_hs: HashMap<String, String> = HashMap::new();
    history.push_with_query(&*props.url, &blank_hs).unwrap();

    let mut global_vars: GlobalVars = (*global_vars_state).clone();

    global_vars.site_vars.server_side_renderer_history = Some(history.clone());
    global_vars.site_vars.server_side_renderer = true;

    let mut body_class = "".to_owned();
    if global_vars.site_vars.current_user.id > 0 {
        body_class = global_vars.site_vars.current_user.theme_css.to_owned();
    }

    let callback_content = move |routes| content_switch(routes, global_vars.clone());

    html! {

        <div class={"theme-".to_owned() + &body_class.replace("_default_", "default")}>
            <Router
                history={history}
            >

            <Switch<MainServerRoute> render={callback_content} />

            </Router>
        </div>
    }
}

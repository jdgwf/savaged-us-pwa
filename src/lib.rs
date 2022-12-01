use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};

mod pages;
mod main_app;
// mod main_app_server;
mod libs;
mod components;
use std::collections::HashMap;
use crate::pages::main_home::MainHome;
use crate::pages::main_about::MainAbout;
use crate::pages::main_tech::MainTech;

use crate::pages::main_todos::MainTodos;
use crate::pages::user::login::UserLogin;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::register::Register;
use serde_json::Error;
use standard_components::ui::nbsp::Nbsp;
// use gloo_console::log;
// use main_app_server::MainServerApp;
use crate::libs::global_vars::GlobalVars;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
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
    submenu: Html,
    show_mobile_menu: bool,
    // global_vars_context: GlobalVarsContext,
    global_vars: GlobalVars,
    current_unread_notifications: u32,
    current_sub_menu: String,

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

fn top_menu_switch(
    routes: MainServerRoute,
    submenu: Html,
    // mobile_menu_callback: &Callback<MouseEvent>,
    global_vars: GlobalVars,
    // _show_mobile_menu: bool,
) -> Html {
    let mut home_class_active = "".to_owned();
    let mut login_class_active = "login-item".to_owned();
    let mut about_class_active = "".to_owned();

    let mut todos_class_active = "".to_owned();
    let mut tech_class_active = "".to_owned();
    match routes {

        MainServerRoute::Home => {
            home_class_active = "active".to_owned();

        },
        // MainServerRoute::UserRouter => {
        //     // test_sheet_class_active = "active".to_owned();
        //     login_class_active = "login-item active".to_owned();
        // },
        MainServerRoute::Register => {
            if global_vars.current_user.id == 0 {
                login_class_active = "login-item active".to_owned();
            }
        },
        MainServerRoute::ForgotPassword => {
            if global_vars.current_user.id == 0 {
                login_class_active = "login-item active".to_owned();
            }
        },
        MainServerRoute::UserLogin => {
            if global_vars.current_user.id == 0 {
                login_class_active = "login-item active".to_owned();
            }
        },
        MainServerRoute::About => {
            about_class_active = "active".to_owned();
        },
        MainServerRoute::Tech => {
            tech_class_active = "active".to_owned();
        },
        MainServerRoute::ToDos => {
            todos_class_active = "active".to_owned();
        },

        MainServerRoute::NotFound => {

        },
    }

    html! {
        <header>
            <div class={"width-limit"}>
            <img src="/images/svgd-us.webp" class={"main-logo"} />
            </div>
            <h1>{"Savaged.us v4"}</h1>
            <div class={"top-menu-bottom"}>
            <div class={"width-limit"}>
            <ul class={"top-menu"}>

                <li class={"mobile-menu-button"}>
                    <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M16 132h416c8.837 0 16-7.163 16-16V76c0-8.837-7.163-16-16-16H16C7.163 60 0 67.163 0 76v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16z"></path></svg>
                </li>
                <li class={home_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::Home}><i class="fa fa-house" /><Nbsp />{"Home"}</Link<MainServerRoute>>
                </li>
                <li class={about_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::About}><i class="fa fa-circle-info" /><Nbsp />{"About"}</Link<MainServerRoute>>
                </li>
                <li class={tech_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::Tech}><i class="fa fa-microchip" /><Nbsp />{"Tech"}</Link<MainServerRoute>>
                </li>
                <li class={todos_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::ToDos}><i class="fa fa-list" /><Nbsp />{"To-Dos"}</Link<MainServerRoute>>
                </li>
                <li class={login_class_active}>

                        <>
                            // <Link<MainServerRoute> to={MainServerRoute::UserLogin}>{"Login/Register"}</Link<MainServerRoute>>
                        </>

                </li>

            </ul>
            </div>
            </div>
            <div class={"width-limit"}>
                {submenu}
            </div>
        </header>
    }
}

fn mobile_menu_switch(
    routes: MainServerRoute,
    submenu: Html,
    // hide_popup_menus_callback: &Callback<MouseEvent>,
    global_vars: GlobalVars,
    // show_mobile_menu: bool,
) -> Html {
    let mut home_class_active = "".to_owned();
    // let mut test_sheet_class_active = "".to_owned();
    let mut about_class_active = "".to_owned();
    let mut todos_class_active = "".to_owned();
    let mut settings_class_active: String = "".to_owned();
    let mut home_submenu = html! { <></> };
    // let mut test_sheet_submenu = html! { <></> };
    let mut about_submenu = html! { <></> };
    let mut todos_submenu = html! { <></> };
    let mut settings_submenu = html! { <></> };

    // let mut user_submenu = html! { <></> };
    let mut tech_class_active = "".to_owned();
    match routes {
        MainServerRoute::Tech => {
            // test_sheet_class_active = "active".to_owned();
            tech_class_active = "active".to_owned();
        },
        MainServerRoute::Home => {
            home_class_active = "active".to_owned();
            home_submenu = submenu.clone();
        },

        MainServerRoute::UserLogin => {

        },
        MainServerRoute::Register => {

        },
        MainServerRoute::ForgotPassword => {

        },
        // MainServerRoute::UserRouter => {
        //     settings_class_active = "active".to_owned();
        //     settings_submenu = submenu.clone();
        // },
        MainServerRoute::About => {
            about_class_active = "active".to_owned();
            about_submenu = submenu.clone();
        },
        MainServerRoute::ToDos => {
            todos_class_active = "active".to_owned();
            todos_submenu = submenu.clone();
        },
        MainServerRoute::NotFound => {

        },

    }

    let mut active_class = "mobile-menu";

    // if show_mobile_menu {
    //     active_class = "mobile-menu show-mobile-menu"
    // }

    html! {
        <div class={active_class}>
            <ul class={"main-menu"}>
                <li class={home_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::Home}><i class="fa fa-house" /><Nbsp />{"Home"}</Link<MainServerRoute>>
                    {home_submenu}
                </li>
                // <li class={test_sheet_class_active}>
                //     <Link<MainServerRoute> to={MainServerRoute::TestSheetRouter { sub_route: "home".to_owned() }}>{"Test Sheet"}</Link<MainServerRoute>>
                //     {test_sheet_submenu}
                // </li>
                <li class={about_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::About}><i class="fa fa-circle-info" /><Nbsp />{"About"}</Link<MainServerRoute>>
                </li>
                <li class={tech_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::Tech}><i class="fa fa-microchip" /><Nbsp />{"Tech"}</Link<MainServerRoute>>
                </li>
                <li class={todos_class_active}>
                    <Link<MainServerRoute> to={MainServerRoute::ToDos}><i class="fa fa-list" /><Nbsp />{"To-Dos"}</Link<MainServerRoute>>
                    {todos_submenu}
                </li>

            </ul>

        </div>
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

    let server_root = "http://localhost:5001".to_owned();
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
            no_calls: true,
            offline: true,
            update_global_vars: Callback::noop(),
        }
    );

    let active_class = "content-pane";

    let history = AnyHistory::from(MemoryHistory::new());
    let blank_hs: HashMap<String, String> = HashMap::new();
    history
        .push_with_query(&*props.url, &blank_hs)
        .unwrap();

    let global_vars: GlobalVars = (*global_vars_state).clone();
    let global_vars3: GlobalVars = (*global_vars_state).clone();

    let callback_content =
        move |routes| {
            content_switch(
                routes,
                // &set_submenu,
                global_vars3.clone(),
                // &on_logout_action,
                &Callback::noop(),
                // &open_confirmation_dialog,
                // show_mobile_menu,
            )
        }
    ;

    let submenu = html! { <></> };
    let mobile_submenu = html! { <></> };

    let global_vars1: GlobalVars = global_vars.clone();
    let global_vars2: GlobalVars = global_vars1.clone();

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
                    <Switch<MainServerRoute> render={
                        move |routes|
                        top_menu_switch(
                            routes,
                            submenu.clone(),
                            // &on_click_toggle_mobile_menu,
                            global_vars2.clone(),
                            // show_mobile_menu,
                        )
                    } />

                    <div class={"position-relative"}>
                    <Switch<MainServerRoute> render={
                        move |routes|
                        mobile_menu_switch(
                            routes,
                            mobile_submenu.clone(),
                            // &on_click_hide_popup_menus,
                            global_vars1.clone(),
                            // show_mobile_menu,
                        )
                    } />
                        <div class={active_class}>
                            <Switch<MainServerRoute> render={callback_content} />
                        </div>
                    </div>
            </Router>
        // </ContextProvider<GlobalVarsContext>>
        </>
    }
}

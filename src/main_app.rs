use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
use yew::prelude::*;
use yew_router::prelude::*;

use standard_components::libs::set_document_title::set_document_title;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::fetch_api::fetch_api;
use crate::pages::user::login::_UserLoginProps::update_global_vars;
use gloo_console::error;
use gloo_console::log;
use crate::pages::main_home::MainHome;
use crate::pages::main_about::MainAbout;
use crate::pages::main_tech::MainTech;
use crate::pages::main_todos::MainTodos;
use crate::pages::user::login::UserLogin;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::register::Register;
use crate::web_sockets::WebsocketService;
use crate::web_sockets::handle_message::handle_message;
use serde_json::Error;
use crate::components::confirmation_dialog::ConfirmationDialog;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::libs::global_vars::GlobalVars;
use gloo_utils::format::JsValueSerdeExt;

use wasm_bindgen_futures::spawn_local;
use crate::pages::user::user_router::UserRoute;
use crate::pages::user::user_router::UserRouter;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;

use savaged_libs::user::User;

use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_net::websocket::{
    Message,
    futures::WebSocket,

};

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/me/*")]
    UserRouter,
    #[at("/me")]
    UserRouterRedirect,
    #[at("/login")]
    UserLogin,
    #[at("/forgot-password")]
    ForgotPassword,
    #[at("/register")]
    Register,
    #[at("/about")]
    About,
    #[at("/tech")]
    Tech,
    #[at("/todos")]
    ToDos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
pub struct MainAppProps {
    // pub global_vars: GlobalVars,
}
#[derive(Clone, Debug)]
pub struct SubmenuData {
    pub html: Html,
    pub menu: String,
    pub unread_notifications: u32,
}
pub enum MainAppMessage {
    SetSubmenu( SubmenuData ),

    // ToggleUserMenu( bool ),
    ToggleMobileMenu(bool),
    HidePopupMenus(bool),

    UpdateCurrentUser( User ),
    UpdateGlobalVars( GlobalVars ),
    ContextUpdated( GlobalVarsContext ),
    LogOut( String ),

    SendWebSocket( WebSocketMessage ),
    ReceivedWebSocket( String ),

    OpenConfirmationDialog(
        ConfirmationDialogDefinition,
    ),
    CloseConfirmationDialog( MouseEvent ),
}

pub struct MainApp {
    submenu: Html,
    show_mobile_menu: bool,
    global_vars_context: GlobalVarsContext,
    global_vars: GlobalVars,
    current_unread_notifications: u32,
    current_sub_menu: String,

    confirmation_dialog_open: bool,
    confirmation_dialog_properties: ConfirmationDialogDefinition,

    wss: WebsocketService,
}

fn content_switch(
    routes: MainRoute,
    submenu_callback: &Callback<SubmenuData>,
    global_vars: GlobalVars,
    on_logout_action: &Callback<MouseEvent>,
    base_update_global_vars: &Callback<GlobalVars>,
    open_confirmation_dialog: &Callback<ConfirmationDialogDefinition>,
    _show_mobile_menu: bool,

) -> Html {

    match routes {
        MainRoute::Home => {

            html! {

                <MainHome
                    global_vars={global_vars}
                />

            }
        },
        MainRoute::About => {

            html! {
                <MainAbout
                    global_vars={global_vars}
                />
            }
        },
        MainRoute::UserRouterRedirect => {

            html! {
                <Redirect<UserRoute> to={UserRoute::SettingsPrivate} />
            }
        },
        MainRoute::Tech => {

            html! {
                <MainTech
                    global_vars={global_vars}
                />
            }
        },
        MainRoute::ToDos => {

            html! {
                <MainTodos
                    global_vars={global_vars}
                />
            }
        },
        MainRoute::UserRouter => {
            html! {
                <UserRouter
                    global_vars={global_vars}
                    set_submenu={submenu_callback}
                    on_logout_action={on_logout_action}
                    update_global_vars={base_update_global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },

        MainRoute::UserLogin => {
            html! {
                <UserLogin
                    global_vars={global_vars}
                    update_global_vars={base_update_global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        MainRoute::ForgotPassword => {
            html! {
                <ForgotPassword
                    global_vars={global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        MainRoute::Register => {
            html! {
                <Register
                    global_vars={global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        MainRoute::NotFound => {
            // set_document_title(self.global_vars.site_title.to_owned(), " Not Found :(".to_owned());
            html! { <h1>{ "MainRoute 404" }</h1> }
        }
    }

}

fn top_menu_switch(
    routes: MainRoute,
    submenu: Html,
    mobile_menu_callback: &Callback<MouseEvent>,
    global_vars: GlobalVars,
    _show_mobile_menu: bool,
) -> Html {
    let mut home_class_active = "".to_owned();
    let mut login_class_active = "login-item".to_owned();
    let mut about_class_active = "".to_owned();

    let mut todos_class_active = "".to_owned();
    let mut tech_class_active = "".to_owned();
    match routes {
        MainRoute::Tech => {
            // test_sheet_class_active = "active".to_owned();
            tech_class_active = "active".to_owned();
        },
        MainRoute::Home => {
            home_class_active = "active".to_owned();

        },
        MainRoute::UserRouter => {
            // test_sheet_class_active = "active".to_owned();
            login_class_active = "login-item active".to_owned();
        },

        MainRoute::Register => {
            if global_vars.current_user.id == 0 {
                login_class_active = "login-item active".to_owned();
            }
        },
        MainRoute::ForgotPassword => {
            if global_vars.current_user.id == 0 {
                login_class_active = "login-item active".to_owned();
            }
        },
        MainRoute::UserLogin => {
            if global_vars.current_user.id == 0 {
                login_class_active = "login-item active".to_owned();
            }
        },

        MainRoute::About => {
            about_class_active = "active".to_owned();
        },
        MainRoute::ToDos => {
            todos_class_active = "active".to_owned();
        },

        MainRoute::UserRouterRedirect => {
        }

        MainRoute::NotFound => {

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
                    <svg onclick={mobile_menu_callback} stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 448 512" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M16 132h416c8.837 0 16-7.163 16-16V76c0-8.837-7.163-16-16-16H16C7.163 60 0 67.163 0 76v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16zm0 160h416c8.837 0 16-7.163 16-16v-40c0-8.837-7.163-16-16-16H16c-8.837 0-16 7.163-16 16v40c0 8.837 7.163 16 16 16z"></path></svg>
                </li>
                <li class={home_class_active}>
                    <Link<MainRoute> to={MainRoute::Home}><i class="fa fa-house" /><Nbsp />{"Home"}</Link<MainRoute>>
                </li>
                // <li class={test_sheet_class_active}>
                //     <Link<MainRoute> to={MainRoute::TestSheetRouter { sub_route: "home".to_owned() }}>{"Test Sheet"}</Link<MainRoute>>
                // </li>
                <li class={about_class_active}>
                    <Link<MainRoute> to={MainRoute::About}><i class="fa fa-circle-info" /><Nbsp />{"About"}</Link<MainRoute>>
                </li>
                <li class={tech_class_active}>
                    <Link<MainRoute> to={MainRoute::Tech}><i class="fa fa-microchip" /><Nbsp />{"Tech"}</Link<MainRoute>>
                </li>
                <li class={todos_class_active}>
                    <Link<MainRoute> to={MainRoute::ToDos}><i class="fa fa-list" /><Nbsp />{"To-Dos"}</Link<MainRoute>>
                </li>
                <li class={login_class_active}>
                    if global_vars.offline {
                        {"OFFLINE"}<br /><br />
                    }
                    if global_vars.current_user.id > 0 {
                        <div class="user-login-badge">
                        <Link<UserRoute> to={UserRoute::SettingsPrivate}>
                            if global_vars.current_user.unread_notifications > 0 {
                                <div class={"unread-notifications"}>{global_vars.current_user.unread_notifications}</div>
                            }
                            <img
                            src={global_vars.current_user.get_image( &global_vars.server_root )}
                            />

                        </Link<UserRoute>>
                        </div>
                    } else {
                        <>
                            <Link<MainRoute> to={MainRoute::UserLogin}>{"Login/Register"}</Link<MainRoute>>
                        </>
                    }
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
    routes: MainRoute,
    submenu: Html,
    hide_popup_menus_callback: &Callback<MouseEvent>,
    global_vars: GlobalVars,
    show_mobile_menu: bool,
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
    let mut tech_class_active = "".to_owned();
    match routes {
        MainRoute::Tech => {
            // test_sheet_class_active = "active".to_owned();
            tech_class_active = "active".to_owned();
        },
        MainRoute::Home => {
            home_class_active = "active".to_owned();
            home_submenu = submenu.clone();
        },
        MainRoute::UserRouterRedirect => {
            settings_submenu = submenu.clone();
        }
        MainRoute::UserLogin => {

        },
        MainRoute::Register => {

        },
        MainRoute::ForgotPassword => {

        },
        MainRoute::UserRouter => {
            settings_class_active = "active".to_owned();
            settings_submenu = submenu.clone();
        },
        MainRoute::About => {
            about_class_active = "active".to_owned();
            about_submenu = submenu.clone();
        },
        MainRoute::ToDos => {
            todos_class_active = "active".to_owned();
            todos_submenu = submenu.clone();
        },
        MainRoute::NotFound => {

        },
    }

    let mut active_class = "mobile-menu";

    if show_mobile_menu {
        active_class = "mobile-menu show-mobile-menu"
    }

    html! {
        <div class={active_class}>
            <ul onclick={hide_popup_menus_callback} class={"main-menu"}>
                <li class={home_class_active}>
                    <Link<MainRoute> to={MainRoute::Home}><i class="fa fa-house" /><Nbsp />{"Home"}</Link<MainRoute>>
                    {home_submenu}
                </li>
                <li class={about_class_active}>
                    <Link<MainRoute> to={MainRoute::About}><i class="fa fa-circle-info" /><Nbsp />{"About"}</Link<MainRoute>>
                </li>
                <li class={tech_class_active}>
                    <Link<MainRoute> to={MainRoute::Tech}><i class="fa fa-microchip" /><Nbsp />{"Tech"}</Link<MainRoute>>
                </li>
                <li class={todos_class_active}>
                    <Link<MainRoute> to={MainRoute::ToDos}><i class="fa fa-list" /><Nbsp />{"To-Dos"}</Link<MainRoute>>
                    {todos_submenu}
                </li>
                if global_vars.current_user.id > 0 {
                    <li class={settings_class_active}>
                        <Link<UserRoute> to={UserRoute::SettingsPrivate}>{"Settings"}</Link<UserRoute>>
                        {settings_submenu}
                    </li>
                }

            </ul>

        </div>
    }
}

impl Component for MainApp {
    type Message = MainAppMessage;
    type Properties = MainAppProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let ( global_vars_context, _global_vars_context_handler ) = ctx
            .link()
            .context::<GlobalVarsContext>(
                ctx.link().callback(MainAppMessage::ContextUpdated)
            )
            .expect("global_vars context was not set");

        let mut global_vars = (*global_vars_context).clone();

        let login_token = global_vars.login_token.to_owned();
        let api_root = global_vars.api_root.to_owned();

        let send_websocket = ctx.link().callback(MainAppMessage::SendWebSocket);
        let base_update_global_vars = ctx.link().callback(MainAppMessage::UpdateGlobalVars);
        global_vars.update_global_vars = base_update_global_vars;
        global_vars.send_websocket = send_websocket;


        let login_token = global_vars.login_token.to_owned();

        let mut login_token_send: Option<String> = None;
        if !login_token.is_empty() {
            login_token_send = Some(login_token);
        }
        let msg = WebSocketMessage {
            token: login_token_send,
            kind: WebsocketMessageType::Online,
            user: None,
            payload: None,
        };

        global_vars.send_websocket.emit( msg );

        let login_token = global_vars.login_token.to_owned();

        if !&global_vars.login_token.is_empty() && !global_vars.no_calls {
            let update_current_user = ctx.link().callback(MainAppMessage::UpdateCurrentUser);

            let mut global_vars = global_vars.clone();

            let global_vars_context = global_vars_context.clone();
            spawn_local (
                async move {
                    let result = fetch_api(
                        (api_root + "/auth/get-user-data").to_owned(),
                        "".to_owned(),
                        login_token,
                    ).await;

                    global_vars.user_loading = false;

                    global_vars_context.dispatch( global_vars.to_owned() );

                    match result {
                        Ok( value ) => {
                            // let vec_val_result = value.into_serde::<User>();
                            let vec_val_result: Result<User, Error> = JsValueSerdeExt::into_serde(&value);
                            match vec_val_result {
                                Ok( vec_val ) => {
                                    update_current_user.emit( vec_val.clone() );
                                    // log!("get_data_via_fetch vec_val_result", &vec_val.share_bio );
                                }
                                Err( err ) => {
                                    let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                                    update_current_user.emit( User::default() );
                                    error!( &err_string  );
                                }
                            }

                        }
                        Err( err ) => {
                            update_current_user.emit( User::default() );
                            error!("get_data_via_fetch Err()", &err );
                        }
                    }
                }
            );
        } else {
            let update_current_user = ctx.link().callback(MainAppMessage::UpdateCurrentUser);
            update_current_user.emit( User::default().clone() );
        }

        let received_message_callback = ctx.link().callback(MainAppMessage::ReceivedWebSocket);

        let wss = WebsocketService::new(
            global_vars.server_root.to_owned(),
            received_message_callback,
        );



        MainApp {
            global_vars_context: global_vars_context,
            global_vars: global_vars,
            submenu: html! { <></> },
            show_mobile_menu: false,
            current_sub_menu: "".to_owned(),
            current_unread_notifications: 0,
            confirmation_dialog_open: false,

            confirmation_dialog_properties: ConfirmationDialogDefinition::default().clone(),
            wss: wss,

        }
    }

    fn changed(
        &mut self,
        _ctx: &Context<Self>,
        _props: &MainAppProps,
    ) -> bool {

        self.global_vars = (*self.global_vars_context).clone();
        // log!("main_app changed called", self.global_vars.user_loading );
        true
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: MainAppMessage,
    ) -> bool {


        let ( global_vars_context, _global_vars_context_handler ) = ctx
            .link()
            .context::<GlobalVarsContext>(
                Callback::noop()
            )
            .expect("global_vars context was not set");
        let global_vars = (*global_vars_context).clone();
        // self.global_vars = (*global_vars_context).clone();
        log!("main_app update called", global_vars.current_user.unread_notifications  );

        match msg {
            MainAppMessage::ToggleMobileMenu( _new_value ) => {
                self.show_mobile_menu = !self.show_mobile_menu;
                return true;
            }

            MainAppMessage::HidePopupMenus( _new_value ) => {
                self.show_mobile_menu = false;
                return true;
            }


            MainAppMessage::CloseConfirmationDialog( _event ) => {
                self.confirmation_dialog_open = false;
                self.confirmation_dialog_properties = ConfirmationDialogDefinition::default().clone();
                return true;
            }

            MainAppMessage::ContextUpdated( _global_vars_context ) => {
                self.global_vars = global_vars.clone();
                return true;
            }

            MainAppMessage::OpenConfirmationDialog( dialog_props ) => {
                self.confirmation_dialog_open = true;
                self.confirmation_dialog_properties = dialog_props.clone();
                return true;
            }

            MainAppMessage::UpdateGlobalVars( new_value ) => {
                log!("MainAppMessage::UpdateGlobalVars called");
                self.global_vars_context.dispatch( new_value.to_owned() );
                self.global_vars = new_value.clone();

                return true;
            }

            MainAppMessage::UpdateCurrentUser( new_value ) => {
                // log!("UpdateCurrentUser", new_value.id, new_value.unread_notifications);
                let mut global_vars = self.global_vars.clone();

                global_vars.current_user = new_value.clone();
                global_vars.user_loading = false;

                self.global_vars_context.dispatch( global_vars.to_owned() );
                self.global_vars = global_vars.clone();

                return true;
            }

            MainAppMessage::LogOut( _new_value ) => {

                // log!("LogOut?");
                self.global_vars.current_user = User::default();
                self.show_mobile_menu = false;

                self.global_vars.login_token = "".to_owned();
                self.global_vars.user_loading = false;
                set_local_storage_string( "login_token", "".to_owned() );

                self.global_vars_context.dispatch( self.global_vars.to_owned() );
                // self.global_vars = global_vars.clone();

                return true;
            }

            MainAppMessage::SendWebSocket(
                send_message,
            ) => {
                let send_data_result = serde_json::to_string( &send_message );

                match send_data_result {
                    Ok( send_data ) => {
                        let msg_result = self.wss.tx.clone().try_send(send_data.to_owned() );
                        match msg_result {
                            Ok(_) => {
                                // do nothing, everything's GREAT!
                                return true;
                            }
                            Err( err ) => {
                                error!("MainWebAppMessages::SendWebSocket json send error", err.to_string(), send_data.to_owned() );
                                return false;
                            }
                        }

                    }
                    Err( err ) => {
                        error!( format!("MainWebAppMessages::SendWebSocket json to_str error {} {:?}", err.to_string(), &send_message) );
                        return false;
                    }
                }

            }

            MainAppMessage::ReceivedWebSocket( sent_data ) => {
                let msg_result: Result<WebSocketMessage, Error> = serde_json::from_str(&sent_data);
                match msg_result {
                    Ok( msg ) => {

                        handle_message(
                            msg,
                            self.global_vars.clone(),
                            ctx.link().callback(MainAppMessage::UpdateGlobalVars),
                        );
                        return true;
                    }
                    Err( err ) => {
                        error!("MainWebAppMessages::ReceivedWebSocket json from_str error", err.to_string(), &sent_data );
                        return false;
                    }
                }
            }


            MainAppMessage::SetSubmenu(
                new_value,
            ) => {

                if &self.current_sub_menu != &new_value.menu || &self.current_unread_notifications != &new_value.unread_notifications
                {
                    self.submenu = new_value.html.clone();
                    self.current_sub_menu = new_value.menu.to_owned();
                    self.current_unread_notifications = new_value.unread_notifications;
                    return true;
                } else {
                    return false;
                }

            }

        }
    }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {

        let show_mobile_menu = self.show_mobile_menu;
        // // log!("main_app view", self.global_vars.current_user.unread_notifications);
        let submenu = self.submenu.clone();
        let mobile_submenu = self.submenu.clone();



        let set_submenu = ctx.link().callback(MainAppMessage::SetSubmenu);
        let toggle_mobile_menu = ctx.link().callback(MainAppMessage::ToggleMobileMenu);
        let hide_popup_menus = ctx.link().callback(MainAppMessage::HidePopupMenus);
        let logout_action = ctx.link().callback(MainAppMessage::LogOut);
        let base_update_global_vars = ctx.link().callback(MainAppMessage::UpdateGlobalVars);
        let open_confirmation_dialog = ctx.link().callback(MainAppMessage::OpenConfirmationDialog);
        let close_confirmation_dialog = ctx.link().callback(MainAppMessage::CloseConfirmationDialog);

        let on_logout_action =  Callback::from( move | _e: MouseEvent | {
            logout_action.emit( "".to_owned() );

        });

        let on_click_toggle_mobile_menu = Callback::from( move | _e: MouseEvent | {
            toggle_mobile_menu.emit( true );

        });

        let on_click_hide_popup_menus = Callback::from( move | _e: MouseEvent | {
            hide_popup_menus.emit( true );

        });

        let mut active_class = "content-pane";

        if show_mobile_menu {
            active_class = "content-pane show-mobile-menu"
        }

        let global_vars1 = self.global_vars.clone();
        let global_vars2 = self.global_vars.clone();
        let global_vars3 = self.global_vars.clone();
        let global_vars4 = self.global_vars.clone();
        let global_vars5 = self.global_vars.clone();

        let login_token= self.global_vars.login_token.to_owned();
        // let callback_content =
        //     move |routes| {
        //         content_switch(
        //             routes,
        //             &set_submenu,
        //             global_vars3.clone(),
        //             &on_logout_action,
        //             &update_global_vars,
        //             &open_confirmation_dialog,
        //             show_mobile_menu,
        //         )
        //     }
        // ;

        html! {

            <>
                if self.confirmation_dialog_open {
                    <ConfirmationDialog
                        global_vars={global_vars4}
                        close_confirmation_dialog={close_confirmation_dialog}
                        definition={self.confirmation_dialog_properties.clone()}
                    />
                }

                <BrowserRouter>
                    <Switch<MainRoute> render={
                        move |routes|
                        top_menu_switch(
                            routes,
                            submenu.clone(),
                            &on_click_toggle_mobile_menu,
                            global_vars1.clone(),
                            show_mobile_menu,
                        )
                    } />

                <div class={"position-relative"}>
                <Switch<MainRoute> render={
                    move |routes|
                    mobile_menu_switch(
                        routes,
                        mobile_submenu.clone(),
                        &on_click_hide_popup_menus,
                        global_vars2.clone(),
                        show_mobile_menu,
                    )
                } />
                    <div class={active_class}>

                        <Switch<MainRoute> render={
                            move |routes| {
                                content_switch(
                                    routes,
                                    &set_submenu,
                                    global_vars3.clone(),
                                    &on_logout_action,
                                    &base_update_global_vars,
                                    &open_confirmation_dialog,
                                    show_mobile_menu,
                                )
                            }
                        } />
                        <footer class="text-center">{("Connecting to server ").to_owned() + &self.global_vars.server_root}</footer>
                    </div>
                </div>
                </BrowserRouter>
            </>
        }
    }
}

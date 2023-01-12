

use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
// use wasm_bindgen_futures::spawn_local;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::clear_local_storage;

use crate::local_storage::clear_all_local_data;
use crate::pages::info::InfoRoute;
use crate::pages::info::InfoRouter;

use crate::web_sockets::connect_to_websocket;
use gloo_console::error;
use gloo_console::log;
use crate::pages::main_home::MainHome;
use crate::pages::admin::AdminRouter;
use crate::pages::admin::home::AdminHome;


use crate::pages::main_playground::MainPlayground;
// use crate::local_storage::check_and_upgrade_index_db_stores;
use crate::components::ui_page::UIPage;

use crate::pages::user::login::UserLogin;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::register::Register;
use crate::web_sockets::WebsocketService;
use crate::web_sockets::handle_message::handle_message;
use serde_json::Error;
use crate::components::confirmation_dialog::ConfirmationDialog;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::libs::global_vars::GlobalVars;

use crate::pages::user::UserRoute;
use crate::pages::user::UserRouter;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;

use savaged_libs::user::User;

use gloo_timers::callback::Interval;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum MainRoute {
    #[at("/")]
    Home,

    #[at("/admin/*")]
    AdminRouter,
    #[at("/admin")]
    AdminHome,

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

    #[at("/info")]
    InfoRouterRedirect,
    #[at("/info/*")]
    InfoRouter,

    #[at("/playground")]
    Playground,
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

    // ToggleUserMenu( bool ),
    ToggleMobileMenu(bool),
    HidePopupMenus(bool),

    UpdateGlobalVars( GlobalVars ),
    ContextUpdated( GlobalVarsContext ),
    LogOut( String ),

    SendWebSocket( WebSocketMessage ),
    ReceivedWebSocket( String ),
    WebsocketOffline( bool ),

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
    global_vars: GlobalVars,
    on_logout_action: &Callback<MouseEvent>,
    base_update_global_vars: &Callback<GlobalVars>,
    open_confirmation_dialog: &Callback<ConfirmationDialogDefinition>,
    _show_mobile_menu: bool,
    on_click_hide_popup_menus: &Callback<MouseEvent>,
    toggle_mobile_menu_callback: &Callback<MouseEvent>,

) -> Html {

    let mut global_vars = global_vars.clone();
    global_vars.current_menu = format!("{}-{:?}", "main", routes ).to_lowercase();
    global_vars.current_sub_menu = "".to_string();
    global_vars.hide_popup_menus_callback = on_click_hide_popup_menus.clone();
    global_vars.toggle_mobile_menu_callback = toggle_mobile_menu_callback.clone();
    global_vars.logout_callback = on_logout_action.clone();
    match routes {
        MainRoute::Home => {

            html! {
                <MainHome
                    global_vars={global_vars}

                />
            }
        },
        MainRoute::InfoRouter => {

            html! {
                <InfoRouter
                    global_vars={global_vars}
                    // on_logout_action={on_logout_action}
                    update_global_vars={base_update_global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        MainRoute::UserRouterRedirect => {

            html! {
                <Redirect<UserRoute> to={UserRoute::SettingsPrivate} />
            }
        },

        MainRoute::InfoRouterRedirect => {

            html! {
                <Redirect<InfoRoute> to={InfoRoute::InfoAbout} />
            }
        },

        MainRoute::AdminRouter => {

            html! {
                <AdminRouter
                    global_vars={global_vars}
                    // on_logout_action={on_logout_action}
                    update_global_vars={base_update_global_vars}
                    open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },

        MainRoute::AdminHome => {

            html! {
                <AdminHome
                    global_vars={global_vars}
                    // on_logout_action={on_logout_action}
                    // update_global_vars={base_update_global_vars}
                    // open_confirmation_dialog={open_confirmation_dialog}
                />
            }
        },
        // MainRoute::Tech => {

        //     html! {
        //         <MainTech
        //             global_vars={global_vars}

        //         />
        //     }
        // },

        // MainRoute::ToDos => {

        //     html! {
        //         <MainTodos
        //             global_vars={global_vars}

        //         />
        //     }
        // },
        MainRoute::Playground => {

            html! {
                <MainPlayground
                    global_vars={global_vars}

                />
            }
        },
        MainRoute::UserRouter => {
            html! {
                <UserRouter
                    global_vars={global_vars}
                    // on_logout_action={on_logout_action}
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
            html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="Not Found 🥲"
                    submenu_tag={"".to_owned()}
                >
                    <h1>{ "MainRoute 404" }</h1>
                </UIPage>
            }
        }
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

        // let login_token = global_vars.login_token.to_owned();
        // let api_root = global_vars.api_root.to_owned();

        let send_websocket = ctx.link().callback(MainAppMessage::SendWebSocket);
        // let base_update_global_vars = ctx.link().callback(MainAppMessage::UpdateGlobalVars);
        // global_vars.update_global_vars = base_update_global_vars;
        global_vars.send_websocket = send_websocket;



        let login_token = global_vars.login_token.to_owned();

        let mut login_token_send: Option<String> = None;
        if !login_token.is_empty() {
            login_token_send = Some(login_token);
        }


        let received_message_callback = ctx.link().callback(MainAppMessage::ReceivedWebSocket);
        let websocket_offline_callback = ctx.link().callback(MainAppMessage::WebsocketOffline);

        let wss = connect_to_websocket(
            global_vars.server_root.to_owned(),
            &received_message_callback,
            &websocket_offline_callback,
            global_vars.login_token.to_owned(),
        );

        // let mut msg = WebSocketMessage::default();

        // msg.token = login_token_send;
        // msg.kind = WebsocketMessageType::Online;

        // global_vars.chargen_data = None;

        global_vars.chargen_data = None;
        // // let global_vars_future_callback = ctx.link().callback( MainAppMessage::UpdateGlobalVars );

        // global_vars.send_websocket.emit( msg );

        // spawn_local(
        //     async move {
        //         check_and_upgrade_index_db_stores().await;
        //     }
        // );

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

    // fn changed(
    //     &mut self,
    //     _ctx: &Context<Self>,
    //     _props: &MainAppProps,
    // ) -> bool {

    //     self.global_vars = (*self.global_vars_context).clone();

    //     log!("main_app changed called" );
    //     // self.reconnect_interval();

    //     true
    // }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: MainAppMessage,
    ) -> bool {



        // let global_vars = self.global_vars.clone();

        match msg {
            MainAppMessage::ToggleMobileMenu( _new_value ) => {
                log!("ToggleMobileMenu called");
                self.global_vars.show_mobile_menu = !self.global_vars.show_mobile_menu;
                return true;
            }

            MainAppMessage::HidePopupMenus( _new_value ) => {
                self.global_vars.show_mobile_menu = false;
                return true;
            }


            MainAppMessage::CloseConfirmationDialog( _event ) => {
                self.confirmation_dialog_open = false;
                self.confirmation_dialog_properties = ConfirmationDialogDefinition::default().clone();
                return true;
            }

            MainAppMessage::ContextUpdated( global_vars_context ) => {
                self.global_vars = (*global_vars_context).clone();
                return true;
            }

            MainAppMessage::OpenConfirmationDialog( dialog_props ) => {
                self.confirmation_dialog_open = true;
                self.confirmation_dialog_properties = dialog_props.clone();
                return true;
            }

            MainAppMessage::UpdateGlobalVars( new_value ) => {
                // log!( format!("MainAppMessage::UpdateGlobalVars called {:?}", &new_value) );

                self.global_vars = new_value.clone();
                self.global_vars.send_websocket = ctx.link().callback(MainAppMessage::SendWebSocket);
                self.global_vars_context.dispatch( new_value.to_owned() );

                return true;
            }

            MainAppMessage::LogOut( _new_value ) => {

                // log!("LogOut?");
                self.global_vars.current_user = User::default();
                self.show_mobile_menu = false;
                self.global_vars.saves = None;
                self.global_vars.chargen_data = None;


                self.global_vars.user_loading = false;
                clear_local_storage();

                let mut logout = WebSocketMessage::default();
                logout.kind = WebsocketMessageType::Logout;
                logout.token = Some(self.global_vars.login_token.clone());
                self.global_vars.send_websocket.emit( logout );

                self.global_vars.login_token = "".to_owned();

                let send_websocket = self.global_vars.send_websocket.clone();
                spawn_local(
                    async move {
                        clear_all_local_data().await;
                        let mut msg = WebSocketMessage::default();
                        msg.kind = WebsocketMessageType::ChargenData;
                        send_websocket.emit( msg );
                    }
                );
                self.global_vars_context.dispatch( self.global_vars.to_owned() );
                // self.global_vars = global_vars.clone();

                return true;
            }

            MainAppMessage::SendWebSocket(
                send_message,
            ) => {
                let send_data_result = serde_json::to_string( &send_message );

                // log!("MainWebAppMessages::SendWebSocket called");
                match send_data_result {
                    Ok( send_data ) => {
                        let msg_result = self.wss.tx.clone().try_send(send_data.to_owned() );
                        match msg_result {
                            Ok( _) => {
                                // do nothing, everything's GREAT!
                                // log!("MainWebAppMessages::SendWebSocket called (Ok)");
                                return false;
                            }
                            Err( _err ) => {
                                // error!("MainWebAppMessages::SendWebSocket json send error", err.to_string(), send_data.to_owned() );
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

            MainAppMessage::WebsocketOffline( offline ) => {
                let mut global_vars = self.global_vars.clone();

                if global_vars.offline != offline {
                    global_vars.offline = offline;

                    log!("WebsocketOffline called", offline);

                    ctx.link().callback(MainAppMessage::UpdateGlobalVars).emit( global_vars );

                }

                if offline {
                    let received_message_callback = ctx.link().callback(MainAppMessage::ReceivedWebSocket);
                    let websocket_offline_callback = ctx.link().callback(MainAppMessage::WebsocketOffline);

                    self.wss = connect_to_websocket(
                        self.global_vars.server_root.to_owned(),
                        &received_message_callback,
                        &websocket_offline_callback,
                        self.global_vars.login_token.to_owned(),
                    );
                }


                return false;
            }

            MainAppMessage::ReceivedWebSocket( sent_data ) => {

                // log!( format!("ReceivedWebSocket {}", &sent_data.len() ) );
                let msg_result: Result<WebSocketMessage, Error> = serde_json::from_str(&sent_data);
                let mut global_vars = self.global_vars.clone();
                // global_vars.update_global_vars = ctx.link().callback(MainAppMessage::UpdateGlobalVars);
                match msg_result {
                    Ok( msg ) => {
                        global_vars.offline = false;
                        // global_vars.user_loading = false;

                        // log!( format!("calling handle_message " ));
                        handle_message(
                            msg,
                            global_vars,
                            ctx.link().callback(MainAppMessage::UpdateGlobalVars),
                        );
                        return false;
                    }
                    Err( err ) => {
                        error!("MainWebAppMessages::ReceivedWebSocket json from_str error", err.to_string(), &sent_data );
                        return false;
                    }
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
        // let submenu = self.submenu.clone();
        // let mobile_submenu = self.submenu.clone();



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

        // let mut active_class = "content-pane";

        // if show_mobile_menu {
        //     active_class = "content-pane show-mobile-menu"
        // }

        // let global_vars1 = self.global_vars.clone();
        // let global_vars2 = self.global_vars.clone();
        let global_vars3 = self.global_vars.clone();
        let global_vars4 = self.global_vars.clone();


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
                            move |routes| {
                                content_switch(
                                    routes,
                                    global_vars3.clone(),
                                    &on_logout_action,
                                    &base_update_global_vars,
                                    &open_confirmation_dialog,
                                    show_mobile_menu,
                                    &on_click_hide_popup_menus,
                                    &on_click_toggle_mobile_menu,
                                )
                            }
                        } />

                    // </div>
                // </div>
                </BrowserRouter>
            </>
        }
    }
}

impl MainApp {
    fn _reconnect_interval(
        &mut self,
        // global_vars: GlobalVars,
    ) {

        // match self.wss.ws.state() {
        //     State::Closed => {
        //         log!("State Closed?!?")
        //     }
        //     _ => {

        //     }
        // }
        // if self.wss.ws.state() == State::Closed {
        //     log!("State Closed?!?")
        // }
        // if self.global_vars.offline {
        //     log!("reconnect_interval called");

        //     let login_token = self.global_vars.login_token.to_owned();

        //     let mut login_token_send: Option<String> = None;
        //     if !login_token.is_empty() {
        //         login_token_send = Some(login_token);
        //     }
        //     let msg = WebSocketMessage {
        //         token: login_token_send,
        //         kind: WebsocketMessageType::Online,
        //         user: None,
        //         payload: None,
        //     };

        //     log!(format!("reconnection l {:?}", msg));
        //     self.global_vars.send_websocket.emit( msg );
        // }
        // return;
        // log!("reconnect_interval called");
        // if !self.interval != 0 {
        //     web_sys::clear_interval_with_handle( self.interval );
        // }

        // if self.global_vars.offline {
        //     log!("We're disconnected, trying to reconnect...");

        //     // if self.interval.is_none() {
        //         // log!( format!("self.interval {:?}", self.interval));

        //         let global_vars = self.global_vars.clone();
        //         Some(Timeout::new(
        //             5_000,
        //             move || {

        //                 // Do something...
        //                 let login_token = global_vars.login_token.to_owned();

        //                 let mut login_token_send: Option<String> = None;
        //                 if !login_token.is_empty() {
        //                     login_token_send = Some(login_token);
        //                 }
        //                 let msg = WebSocketMessage {
        //                     token: login_token_send,
        //                     kind: WebsocketMessageType::Online,
        //                     user: None,
        //                     payload: None,
        //                 };

        //                 log!(format!("reconnection l {:?}", msg));
        //                 global_vars.send_websocket.emit( msg );
        //             }
        //         ));
        //     // }
        // // } else {
        //     // let iv_option = &mut self.interval;
        //     // match iv_option {
        //     //     Some( iv ) => {
        //     //         // iv.cancel(); // ownership error; trying more hacky way
        //     //         log!("Disconnecting Interval");
        //     //         let window = web_sys::window().unwrap();
        //     //         window.clear_interval_with_handle( 1 );
        //     //         window.clear_interval_with_handle( 2 );
        //     //         window.clear_interval_with_handle( 3 );
        //     //     }
        //     //     None => {

        //     //     }
        //     // }
        //     // self.interval = None;
        // };
    }

}

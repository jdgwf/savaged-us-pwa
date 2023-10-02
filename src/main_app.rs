pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
use std::ops::Deref;

use crate::components::alerts::AlertDefinition;
use crate::components::alerts::Alerts;
use crate::components::confirmation_dialog::ConfirmationDialog;
use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::libs::data_api::get_game_data;
use crate::libs::data_api::get_saves;
use crate::libs::data_api::get_current_user;
use crate::libs::data_api::logout_session;
use crate::libs::site_vars::SiteVars;
use crate::libs::global_vars::GlobalVars;
use crate::local_storage::clear_all_local_data;
use crate::local_storage::index_db_get_saves;
use crate::local_storage::index_db_get_game_data;
use crate::pages::admin::AdminRouter;
use crate::pages::admin::home::AdminHome;
use crate::pages::error404::Error404;
use crate::pages::help::HelpRouter;
use crate::pages::help::home::HelpHome;
use crate::pages::info::InfoRoute;
use crate::pages::info::InfoRouter;
use crate::pages::main_home::MainHome;
use crate::pages::user::UserRoute;
use crate::pages::user::UserRouter;
use crate::pages::user::forgot_password::ForgotPassword;
use crate::pages::user::login::UserLogin;
use crate::pages::user::register::Register;
use crate::web_sockets::WebsocketService;
use crate::web_sockets::connect_to_websocket;
use crate::web_sockets::handle_message::handle_message;
use gloo_console::error;
use gloo_console::log;
use gloo_timers::future::TimeoutFuture;
use savaged_libs::player_character::game_data_package::GameDataPackage;
use savaged_libs::save_db_row::SaveDBRow;
use savaged_libs::user::User;
use savaged_libs::websocket_message::{WebSocketMessage, WebsocketMessageType};
use serde_json::Error;
use standard_components::libs::local_storage_shortcuts::clear_local_storage;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
// use standard_components::libs::set_body_class::set_body_class;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable, Debug)]
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

    #[at("/help")]
    HelpHome,
    #[at("/help/*")]
    HelpRouter,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
pub struct MainAppProps {}

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

    UpdateSiteVars(SiteVars),
    UpdateGlobalVars(GlobalVars),
    ContextUpdated(GlobalVarsContext),
    LogOut(String),

    SendWebSocket(WebSocketMessage),
    ReceivedWebSocket(String),
    WebsocketOffline(bool),

    OpenConfirmationDialog(ConfirmationDialogDefinition),
    CloseConfirmationDialog(MouseEvent),
    AddAlert(AlertDefinition),
    RemoveAlert(Uuid),
    AlertFadeIn(Uuid),
    AlertDisplayNone(Uuid),
    AlertFadeOut(Uuid),

    UpdateGameData( Option<GameDataPackage> ),
    UpdateSaves( Vec<SaveDBRow> ),

    UpdateCurrentUser( User ),
}

pub struct MainApp {
    global_vars_context: GlobalVarsContext,
    global_vars: GlobalVars,
    alerts: Vec<AlertDefinition>,
    confirmation_dialog_open: bool,
    confirmation_dialog_properties: ConfirmationDialogDefinition,

    wss: WebsocketService,
}

fn content_switch(
    routes: MainRoute,
    mut global_vars: &GlobalVars,
) -> Html {

    // global_vars.site_vars.current_menu = format!("{}-{:?}", "main", routes).to_lowercase();
    let mut site_vars = global_vars.site_vars.clone();
    site_vars.current_menu = format!("{}-{:?}", "main", routes).to_lowercase();

    match routes {
        MainRoute::Home => {
            html! {
                <MainHome
                    site_vars={site_vars}

                />
            }
        }
        MainRoute::InfoRouter => {
            html! {
                <InfoRouter
                    site_vars={site_vars}
                    web_content={global_vars.web_content.clone()}
                />
            }
        }
        MainRoute::HelpRouter => {
            html! {
                <HelpRouter
                    site_vars={site_vars}
                    //web_content={global_vars.help_content}
                />
            }
        }
        MainRoute::HelpHome => {
            html! {
                <HelpHome
                    site_vars={site_vars}
                    //web_content={global_vars.help_content}
                />
            }
        }
        MainRoute::UserRouterRedirect => {
            html! {
                <Redirect<UserRoute> to={UserRoute::SettingsPrivate} />
            }
        }

        MainRoute::InfoRouterRedirect => {
            html! {
                <Redirect<InfoRoute> to={InfoRoute::InfoAbout} />
            }
        }

        MainRoute::AdminRouter => {
            html! {
                <AdminRouter
                    site_vars={site_vars}

                />
            }
        }

        MainRoute::AdminHome => {
            html! {
                <AdminHome
                    site_vars={site_vars}

                />
            }
        }

        MainRoute::UserRouter => {
            html! {
                <UserRouter
                    site_vars={site_vars}
                    game_data={global_vars.game_data.clone()}
                    saves={global_vars.saves.clone()}
                />
            }
        }

        MainRoute::UserLogin => {
            html! {
                <UserLogin
                    site_vars={site_vars}
                    game_data={global_vars.game_data.clone()}
                    saves={global_vars.saves.clone()}
                />
            }
        }
        MainRoute::ForgotPassword => {
            html! {
                <ForgotPassword
                    site_vars={site_vars}

                />
            }
        }
        MainRoute::Register => {
            html! {
                <Register
                    site_vars={site_vars}

                />
            }
        }
        MainRoute::NotFound => {
            html! {
                <Error404
                    site_vars={site_vars}
                />
            }
        }
    }
}

impl Component for MainApp {
    type Message = MainAppMessage;
    type Properties = MainAppProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (global_vars_context, _global_vars_context_handler) = ctx
            .link()
            .context::<GlobalVarsContext>(ctx.link().callback(MainAppMessage::ContextUpdated))
            .expect("global_vars context was not set");

        let mut global_vars = (*global_vars_context).clone();

        let send_websocket = ctx.link().callback(MainAppMessage::SendWebSocket);
        global_vars.site_vars.send_websocket = send_websocket;
        global_vars.site_vars.update_game_data = ctx.link().callback(MainAppMessage::UpdateGameData);
        global_vars.site_vars.update_saves = ctx.link().callback(MainAppMessage::UpdateSaves);
        global_vars.site_vars.update_site_vars = ctx.link().callback(MainAppMessage::UpdateSiteVars);
        global_vars.site_vars.update_global_vars = ctx.link().callback(MainAppMessage::UpdateGlobalVars);
        global_vars.site_vars.add_alert = ctx.link().callback(MainAppMessage::AddAlert);
        global_vars.site_vars.update_current_user = ctx.link().callback(MainAppMessage::UpdateCurrentUser);

        let received_message_callback = ctx.link().callback(MainAppMessage::ReceivedWebSocket);
        let websocket_offline_callback = ctx.link().callback(MainAppMessage::WebsocketOffline);
        global_vars.site_vars.open_confirmation_dialog =  ctx.link().callback(MainAppMessage::OpenConfirmationDialog);

        let wss = connect_to_websocket(
            global_vars.site_vars.server_root.to_owned(),
            &received_message_callback,
            &websocket_offline_callback,
            global_vars.site_vars.login_token.to_owned(),
        );

        global_vars.game_data = None;


        let site_vars = global_vars.site_vars.clone();

        let mut game_data_update = None;
        let game_data_last_updated_string = get_local_storage_string("game_data_last_updated", "".to_owned());
        if !game_data_last_updated_string.is_empty() {
            game_data_update = Some(game_data_last_updated_string.clone());
        }

        let mut saves_update = None;
        let saves_last_updated_string = get_local_storage_string("saves_last_updated", "".to_owned());
        if !saves_last_updated_string.is_empty() {
            saves_update = Some(saves_last_updated_string.clone());
        }
        let gv = global_vars.clone();

        log!( format!("game_data_update {:?}", &game_data_update));
        log!( format!("saves_update {:?}", &saves_update));
        spawn_local(async move {

            let game_data = index_db_get_game_data().await;
            site_vars.update_game_data.emit( game_data );

            let saves = index_db_get_saves().await;
            site_vars.update_saves.emit( saves.unwrap_or(Vec::new()) );

            // let global_vars = global_vars.clone();
            // get_current_user(
            //     site_vars.login_token.clone(),
            //     &site_vars,
            // ).await;


            // let site_vars = global_vars.site_vars.clone();
            // get_game_data(
            //     site_vars.login_token.clone(),
            //     &site_vars,
            //     game_data_update.clone(),
            // ).await;

            // let site_vars = global_vars.site_vars.clone();
            // get_saves(
            //     site_vars.login_token.clone(),
            //     &site_vars,
            //     saves_update.clone(),
            // ).await;

        });


        MainApp {
            global_vars_context: global_vars_context,
            global_vars: gv,
            alerts: Vec::new(),
            confirmation_dialog_open: false,
            confirmation_dialog_properties: ConfirmationDialogDefinition::default().clone(),
            wss: wss,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: MainAppMessage) -> bool {
        match msg {
            MainAppMessage::ToggleMobileMenu(_new_value) => {
                // log!("ToggleMobileMenu called", self.global_vars.site_vars.show_mobile_menu);
                self.global_vars.site_vars.show_mobile_menu = !self.global_vars.site_vars.show_mobile_menu;
                return true;
            }

            MainAppMessage::HidePopupMenus(_new_value) => {
                self.global_vars.site_vars.show_mobile_menu = false;
                return true;
            }

            MainAppMessage::CloseConfirmationDialog(_event) => {
                self.confirmation_dialog_open = false;
                self.confirmation_dialog_properties =
                    ConfirmationDialogDefinition::default().clone();
                return true;
            }

            MainAppMessage::ContextUpdated(global_vars_context) => {
                self.global_vars = (*global_vars_context).clone();
                return true;
            }

            MainAppMessage::OpenConfirmationDialog(dialog_props) => {
                self.confirmation_dialog_open = true;
                self.confirmation_dialog_properties = dialog_props.clone();
                return true;
            }

            MainAppMessage::AddAlert(alert_def) => {
                let alert_fade_in = ctx.link().callback(MainAppMessage::AlertFadeIn);
                let alert_fade_out = ctx.link().callback(MainAppMessage::AlertFadeOut);
                let alert_remove = ctx.link().callback(MainAppMessage::RemoveAlert);
                let alert_display_none = ctx.link().callback(MainAppMessage::AlertDisplayNone);

                let uuid1 = alert_def.uuid.to_owned();
                let uuid2 = alert_def.uuid.to_owned();
                let uuid3 = alert_def.uuid.to_owned();

                self.alerts.insert(0, alert_def.clone());

                // log!("Added Alert", &uuid1.to_string(), self.alerts.len());

                spawn_local(async move {
                    // log!("Spawn1");
                    let alert_fade_in = alert_fade_in.clone();
                    let alert_fade_out = alert_fade_out.clone();
                    let alert_display_none = alert_display_none.clone();
                    let alert_remove = alert_remove.clone();

                    // log!( format!("Spawn2 {}", uuid1.to_owned() ));
                    TimeoutFuture::new(50).await;
                    alert_fade_in.emit(uuid1);

                    // log!( format!("Spawn3 {}", uuid1.to_owned() ));
                    TimeoutFuture::new(3000).await;
                    alert_fade_out.emit(uuid2);

                    TimeoutFuture::new(350).await;
                    alert_display_none.emit(uuid2);

                    // log!( format!("Spawn4 {}", uuid1.to_owned() ));
                    TimeoutFuture::new(1000).await;
                    alert_remove.emit(uuid3);

                    // log!("SpawnEnd");
                });

                return true;
            }

            MainAppMessage::RemoveAlert(uuid) => {
                // log!("RemoveAlert", uuid.to_string(), self.alerts.len());

                let mut new_alerts: Vec<AlertDefinition> = Vec::new();
                for alert in self.alerts.clone().into_iter() {
                    if alert.uuid != uuid {
                        new_alerts.push(alert);
                    }
                }

                // log!("RemoveAlert CLONE", uuid.to_string(), self.alerts.len(), new_alerts.len());
                self.alerts = new_alerts.clone();
                return true;
            }

            MainAppMessage::AlertDisplayNone(uuid) => {
                // log!("AlertFadeIn", uuid.to_string(), self.alerts.len());

                let mut new_alerts: Vec<AlertDefinition> = Vec::new();
                for mut alert in self.alerts.clone().into_iter() {
                    if alert.uuid == uuid {
                        alert.active_class = "display-none".to_owned();
                    }
                    new_alerts.push(alert);
                }

                // log!("AlertFadeIn CLONE", uuid.to_string(), self.alerts.len(), new_alerts.len());
                self.alerts = new_alerts.clone();
                return true;
            }
            MainAppMessage::AlertFadeIn(uuid) => {
                // log!("AlertFadeIn", uuid.to_string(), self.alerts.len());

                let mut new_alerts: Vec<AlertDefinition> = Vec::new();
                for mut alert in self.alerts.clone().into_iter() {
                    if alert.uuid == uuid {
                        alert.active_class = "visible".to_owned();
                    }
                    new_alerts.push(alert);
                }

                // log!("AlertFadeIn CLONE", uuid.to_string(), self.alerts.len(), new_alerts.len());
                self.alerts = new_alerts.clone();
                return true;
            }

            MainAppMessage::AlertFadeOut(uuid) => {
                // log!("AlertFadeOut", uuid.to_string(), self.alerts.len());

                let mut new_alerts: Vec<AlertDefinition> = Vec::new();
                for mut alert in self.alerts.clone().into_iter() {
                    if alert.uuid == uuid {
                        alert.active_class = "".to_owned();
                    }
                    new_alerts.push(alert);
                }

                // log!("AlertFadeOut CLONE", uuid.to_string(), self.alerts.len(), new_alerts.len());
                self.alerts = new_alerts.clone();
                return true;
            }

            MainAppMessage::UpdateSiteVars(new_value) => {
                // log!( format!("MainAppMessage::UpdateSiteVars called {:?}", &new_value) );

                self.global_vars.site_vars = new_value.clone();
                self.global_vars.site_vars.send_websocket =
                    ctx.link().callback(MainAppMessage::SendWebSocket);

                self.global_vars_context.dispatch(self.global_vars.to_owned());

                return true;
            }

            MainAppMessage::UpdateSaves(new_value) => {
                // log!( format!("MainAppMessage::UpdateSaves called {:?}", &new_value) );

                self.global_vars.saves = Some(new_value.clone());
                self.global_vars.site_vars.send_websocket =
                    ctx.link().callback(MainAppMessage::SendWebSocket);
                self.global_vars.site_vars.offline = false;
                self.global_vars.site_vars.saves_loading = false;
                self.global_vars_context.dispatch(self.global_vars.to_owned());

                return true;
            }

            MainAppMessage::UpdateGameData(new_value) => {
                // log!( format!("MainAppMessage::UpdateGameData called {:?}", &new_value) );

                self.global_vars.game_data = new_value.clone();
                self.global_vars.site_vars.offline = false;
                self.global_vars.site_vars.game_data_loading = false;
                self.global_vars.site_vars.send_websocket =
                    ctx.link().callback(MainAppMessage::SendWebSocket);
                self.global_vars_context.dispatch(self.global_vars.to_owned());

                return true;
            }

            MainAppMessage::UpdateCurrentUser(new_value) => {
                // log!( format!("MainAppMessage::UpdateCurrentUser called {:?}", &new_value) );

                self.global_vars.site_vars.current_user= new_value.clone();
                self.global_vars.site_vars.send_websocket =
                    ctx.link().callback(MainAppMessage::SendWebSocket);

                self.global_vars.site_vars.user_loading = false;
                self.global_vars_context.dispatch(self.global_vars.to_owned());

                return true;
            }

            MainAppMessage::UpdateGlobalVars(new_value) => {
                // log!( format!("MainAppMessage::UpdateSiteVars called {:?}", &new_value) );

                self.global_vars= new_value.clone();
                self.global_vars.site_vars.send_websocket =
                    ctx.link().callback(MainAppMessage::SendWebSocket);
                self.global_vars_context.dispatch(self.global_vars.to_owned());

                return true;
            }

            MainAppMessage::LogOut(_new_value) => {
                log!("LogOut?");
                self.global_vars.site_vars.current_user = User::default();
                self.global_vars.site_vars.show_mobile_menu = false;
                self.global_vars.saves = None;
                self.global_vars.game_data = None;

                self.global_vars.site_vars.user_loading = false;
                clear_local_storage();

                let mut logout = WebSocketMessage::default();
                logout.kind = WebsocketMessageType::Logout;
                logout.token = Some(self.global_vars.site_vars.login_token.clone());

                self.global_vars.site_vars.send_websocket.emit(logout);
                self.global_vars.site_vars.login_token = "".to_owned();

                let send_websocket = self.global_vars.site_vars.send_websocket.clone();
                let site_vars = self.global_vars.site_vars.clone();
                spawn_local(async move {
                    clear_all_local_data().await;
                    logout_session( &site_vars ).await;
                    let mut msg = WebSocketMessage::default();
                    msg.kind = WebsocketMessageType::GameDataPackageUpdated;
                    send_websocket.emit(msg);
                });

                // set_local_storage_string( "UI_THEME", "_default_".to_string());

                self.global_vars_context
                    .dispatch(self.global_vars.to_owned());
                // self.global_vars = global_vars.clone();

                return true;
            }

            MainAppMessage::SendWebSocket(send_message) => {
                let send_data_result = serde_json::to_string(&send_message);

                // log!("MainWebAppMessages::SendWebSocket called");
                match send_data_result {
                    Ok(send_data) => {
                        let msg_result = self.wss.tx.clone().try_send(send_data.to_owned());
                        match msg_result {
                            Ok(_) => {
                                // do nothing, everything's GREAT!
                                // log!("MainWebAppMessages::SendWebSocket called (Ok)");
                                return false;
                            }
                            Err(_err) => {
                                // error!("MainWebAppMessages::SendWebSocket json send error", err.to_string(), send_data.to_owned() );
                                return false;
                            }
                        }
                    }
                    Err(err) => {
                        if !self.global_vars.site_vars.server_side_renderer {
                            error!(format!(
                                "MainWebAppMessages::SendWebSocket json to_str error {} {:?}",
                                err.to_string(),
                                &send_message
                            ));
                        }
                        return false;
                    }
                }
            }

            MainAppMessage::WebsocketOffline(offline) => {
                let mut global_vars = self.global_vars.clone();

                if global_vars.site_vars.offline != offline {
                    global_vars.site_vars.offline = offline;

                    // if !global_vars.site_vars.server_side_renderer {
                    //     log!("WebsocketOffline called", offline);
                    // }

                    ctx.link()
                        .callback(MainAppMessage::UpdateGlobalVars)
                        .emit(global_vars);
                }

                let window = web_sys::window().expect("Missing Window");
                let navigator = window.navigator();

                if offline && navigator.on_line() {
                    let received_message_callback = ctx.link().callback(MainAppMessage::ReceivedWebSocket);
                    let websocket_offline_callback = ctx.link().callback(MainAppMessage::WebsocketOffline);

                    self.wss = connect_to_websocket(
                        self.global_vars.site_vars.server_root.to_owned(),
                        &received_message_callback,
                        &websocket_offline_callback,
                        self.global_vars.site_vars.login_token.to_owned(),
                    );

                }

                return false;
            }

            MainAppMessage::ReceivedWebSocket(sent_data) => {
                // log!( format!("MainAppMessage::ReceivedWebSocket {}", &sent_data.len() ) );
                // log!( format!("ReceivedWebSocket {}", &sent_data ) );
                let msg_result: Result<WebSocketMessage, Error> = serde_json::from_str(&sent_data);
                let mut global_vars = self.global_vars.clone();
                // global_vars.update_site_vars = ctx.link().callback(MainAppMessage::UpdateSiteVars);
                match msg_result {
                    Ok(msg) => {
                        // global_vars.offline = false;
                        // global_vars.site_vars.user_loading = false;

                        // log!( format!("calling handle_message {:?}", msg ));
                        handle_message(
                            msg,
                            global_vars,
                            ctx.link().callback(MainAppMessage::UpdateSiteVars),
                            ctx.link().callback(MainAppMessage::UpdateGlobalVars),
                        );
                        return false;
                    }
                    Err(err) => {
                        if !self.global_vars.site_vars.server_side_renderer {
                            error!(
                                "MainWebAppMessages::ReceivedWebSocket json from_str error",
                                err.to_string(),
                                &sent_data
                            );
                        }
                        return false;
                    }
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let toggle_mobile_menu = ctx.link().callback(MainAppMessage::ToggleMobileMenu);
        let hide_popup_menus = ctx.link().callback(MainAppMessage::HidePopupMenus);
        let logout_action = ctx.link().callback(MainAppMessage::LogOut);

        let close_confirmation_dialog =
            ctx.link().callback(MainAppMessage::CloseConfirmationDialog);

        let on_logout_action = Callback::from(move |_e: MouseEvent| {
            logout_action.emit("".to_owned());
        });

        let on_click_toggle_mobile_menu = Callback::from(move |_e: MouseEvent| {
            toggle_mobile_menu.emit(true);
        });

        let on_click_hide_popup_menus = Callback::from(move |_e: MouseEvent| {
            hide_popup_menus.emit(true);
        });


        let mut global_vars = self.global_vars.clone();
        let mut body_class = get_local_storage_string( "UI_THEME", "".to_string());

        if self.global_vars.site_vars.current_user.id > 0 {
            body_class = self.global_vars.site_vars.current_user.theme_css.to_owned();
        }

        global_vars.site_vars.current_sub_menu = "".to_string();
        global_vars.site_vars.hide_popup_menus_callback = on_click_hide_popup_menus.to_owned();
        global_vars.site_vars.toggle_mobile_menu_callback = on_click_toggle_mobile_menu.to_owned();
        global_vars.site_vars.logout_callback = on_logout_action.to_owned();
        global_vars.site_vars.app_version = env!("CARGO_PKG_VERSION").to_owned();

        html! {

            <div id="main-container" class={"theme-".to_owned() + &body_class.replace("_default_", "default")}>
                if self.confirmation_dialog_open {
                    <ConfirmationDialog
                        close_confirmation_dialog={close_confirmation_dialog}
                        definition={self.confirmation_dialog_properties.clone()}
                    />
                }

                <Alerts
                    alerts={self.alerts.clone()}
                />

                <BrowserRouter>

                        <Switch<MainRoute> render={
                            move |routes| {
                                content_switch(
                                    routes,
                                    &global_vars,
                                )
                            }
                        } />
                </BrowserRouter>
            </div>
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
        // if global_varsoffline {
        //     log!("reconnect_interval called");

        //     let login_token = self.global_vars.site_vars.login_token.to_owned();

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
        //     self.global_vars.site_vars.send_websocket.emit( msg );
        // }
        // return;
        // log!("reconnect_interval called");
        // if !self.interval != 0 {
        //     web_sys::clear_interval_with_handle( self.interval );
        // }

        // if global_varsoffline {
        //     log!("We're disconnected, trying to reconnect...");

        //     // if self.interval.is_none() {
        //         // log!( format!("self.interval {:?}", self.interval));

        //         let global_vars = self.global_vars.clone();
        //         Some(Timeout::new(
        //             5_000,
        //             move || {

        //                 // Do something...
        //                 let login_token = global_vars.site_vars.login_token.to_owned();

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
        //                 global_vars.site_vars.send_websocket.emit( msg );
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

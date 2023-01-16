use std::rc::Rc;
use yew::prelude::*;
use savaged_libs::{user::User, websocket_message::WebSocketMessage, save_db_row::SaveDBRow, player_character::game_data_package::GameDataPackage};
use yew_router::history::{AnyHistory};

use gloo_net::websocket::{
    Message,
    futures::WebSocket,

};

use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalVars {
    pub api_root: String,
    pub current_menu: String,
    pub current_sub_menu: String,
    pub current_user: User,
    pub game_data: Option<GameDataPackage>,
    pub hide_popup_menus_callback: Callback<MouseEvent>,
    pub login_token: String,
    pub logout_callback: Callback<MouseEvent>,
    pub offline: bool,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
    pub saves: Option<Vec<SaveDBRow>>,
    pub send_websocket: Callback<WebSocketMessage>,
    pub server_root: String,
    pub server_side_renderer: bool,
    pub server_side_renderer_history: Option<AnyHistory>,
    pub show_mobile_menu: bool,
    pub site_title: String,
    pub toggle_mobile_menu_callback: Callback<MouseEvent>,
    pub update_global_vars: Callback<GlobalVars>,
    pub user_loading: bool,
}

impl Default for GlobalVars {
    fn default() -> Self {
        Self {
            api_root: "".to_owned(),
            current_menu: "".to_owned(),
            current_sub_menu: "".to_owned(),
            current_user: User::default(),
            game_data: None,
            hide_popup_menus_callback: Callback::noop(),
            login_token: "".to_owned(),
            logout_callback: Callback::noop(),
            offline: true,
            open_confirmation_dialog: Callback::noop(),
            saves: None,
            send_websocket: Callback::noop(),
            server_root: "".to_owned(),
            server_side_renderer: false,
            server_side_renderer_history: None,
            show_mobile_menu: false,
            site_title: "".to_owned(),
            toggle_mobile_menu_callback: Callback::noop(),
            update_global_vars: Callback::noop(),
            user_loading: true,
        }
    }
}

impl Reducible for GlobalVars {
    type Action = GlobalVars;

    fn reduce(
        self: Rc<Self>,
        _action: Self::Action,
    ) -> Rc<Self> {
        // action
        self
    }
}

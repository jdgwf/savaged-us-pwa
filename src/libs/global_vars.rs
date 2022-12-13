use serde::{Serialize, Deserialize};
use std::rc::Rc;
use yew::prelude::*;
use savaged_libs::{user::User, websocket_message::WebSocketMessage, save_db_row::SaveDBRow, player_character::chargen_data::ChargenData};
use yew::prelude::*;

use gloo_net::websocket::{
    Message,
    futures::WebSocket,

};

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalVars {
    pub login_token: String,
    pub current_user: User,
    pub user_loading: bool,
    pub api_root: String,
    pub server_root: String,
    pub site_title: String,

    pub no_calls: bool,
    pub offline: bool,

    // pub update_global_vars: Callback<GlobalVars>,
    pub send_websocket: Callback<WebSocketMessage>,
    pub chargen_data: Option<ChargenData>,
    pub saves: Option<Vec<SaveDBRow>>,

    pub current_menu: String,
    pub current_sub_menu: String,
    pub hide_popup_menus_callback: Callback<MouseEvent>,
    pub toggle_mobile_menu_callback: Callback<MouseEvent>,
    pub logout_callback: Callback<MouseEvent>,

    pub show_mobile_menu: bool,
}

impl Default for GlobalVars {
    fn default() -> Self {
        Self {
            current_user: User::default(),
            login_token: "".to_owned(),
            user_loading: true,
            offline: true,
            api_root: "".to_owned(),
            server_root: "".to_owned(),
            site_title: "".to_owned(),
            show_mobile_menu: false,
            no_calls: false,
            // update_global_vars: Callback::noop(),
            send_websocket: Callback::noop(),
            hide_popup_menus_callback: Callback::noop(),
            toggle_mobile_menu_callback: Callback::noop(),
            logout_callback: Callback::noop(),
            chargen_data: None,
            saves: None,

            current_menu: "".to_owned(),
            current_sub_menu: "".to_owned(),

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

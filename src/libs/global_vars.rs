use serde::{Serialize, Deserialize};
use std::rc::Rc;
use yew::prelude::*;
use savaged_libs::{user::User, websocket_message::WebSocketMessage};
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

    pub update_global_vars: Callback<GlobalVars>,
    pub send_websocket: Callback<WebSocketMessage>,
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
            no_calls: false,
            update_global_vars: Callback::noop(),
            send_websocket: Callback::noop(),
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

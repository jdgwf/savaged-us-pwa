use yew::prelude::*;
use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
use crate::libs::global_vars::GlobalVars;
use gloo_console::error;
use gloo_console::log;

pub fn handle_message(
    msg: WebSocketMessage,
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
) {
    match msg.kind {
        WebsocketMessageType::Online => {
            log!( format!("handle_message Online {:?}", msg) );
            let mut global_vars = global_vars.clone();
            global_vars.offline = false;
            update_global_vars.emit( global_vars );
        }

        WebsocketMessageType::Offline => {
            log!( format!("handle_message Offline {:?}", msg) );
            let mut global_vars = global_vars.clone();
            global_vars.offline = true;
            update_global_vars.emit( global_vars );
        }

        _ => {
            error!( format!("Unhandled Message Type! {:?}", msg ) );
            let mut global_vars = global_vars.clone();
            global_vars.offline = false;
            update_global_vars.emit( global_vars );
        }
    }
}
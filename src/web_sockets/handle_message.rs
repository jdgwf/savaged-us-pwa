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
    // update_global_vars: Callback<GlobalVars>,
) {
    match msg.kind {
        WebsocketMessageType::Online => {
            log!( format!("handle_message Online {:?}", msg) );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = false;
            new_global_vars.user_loading = false;
            match msg.user {
                Some( user ) =>  {
                    new_global_vars.current_user = user.clone();
                }
                None => {

                }
            }

            log!( format!("handle_message new_global_vars {:?}", &new_global_vars ) );
            global_vars.update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::Offline => {
            log!( format!("handle_message Offline {:?}", msg) );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = true;
            new_global_vars.user_loading = false;
            global_vars.update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::ChargenData => {
            log!( format!("handle_message ChargenData {:?}", msg) );
            // let mut new_global_vars = global_vars.clone();
            // new_global_vars.offline = true;
            // new_global_vars.user_loading = false;
            // global_vars.update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::Saves => {
            log!( format!("handle_message Saves {:?}", msg) );
            // let mut new_global_vars = global_vars.clone();
            // new_global_vars.offline = true;
            // new_global_vars.user_loading = false;
            // global_vars.update_global_vars.emit( new_global_vars );
        }

        _ => {
            error!( format!("Unhandled Message Type! {:?}", msg ) );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = false;
            // global_vars.update_global_vars.emit( new_global_vars );
        }
    }
}
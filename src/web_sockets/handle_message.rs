use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
use crate::libs::global_vars::GlobalVars;
use crate::local_storage::get_chargen_data_from_index_db;
use crate::local_storage::index_db_save_chargen_data;
use gloo_console::error;
use gloo_console::log;

pub fn handle_message(
    msg: WebSocketMessage,
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
) {
    match msg.kind {
        WebsocketMessageType::Online => {
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


            let mut global_vars_future = new_global_vars.clone();
            spawn_local(async move {

                global_vars_future.chargen_data = get_chargen_data_from_index_db().await;
                update_global_vars.emit(global_vars_future);
            });

            // log!( format!("handle_message new_global_vars {:?}", &new_global_vars ) );
            // update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::Offline => {
            log!( format!("handle_message Offline {:?}", msg) );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = true;
            new_global_vars.user_loading = false;
            update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::ChargenData => {
            // log!( format!("handle_message ChargenData {:?}", msg) );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.chargen_data = msg.chargen_data.clone();
            // new_global_vars.user_loading = false;

            match  msg.chargen_data {
                Some( chargen_data ) => {
                    spawn_local(async move {
                        let results = index_db_save_chargen_data(chargen_data).await;
                        log!( format!(" results, {:?}", results ) );
                    });
                }
                None => {}
            }



            update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::Saves => {
            // log!( format!("handle_message Saves {:?}", msg.saves) );

            let mut new_global_vars = global_vars.clone();
            new_global_vars.saves = msg.saves;
            // new_global_vars.offline = true;
            // new_global_vars.user_loading = false;
            update_global_vars.emit( new_global_vars );
        }

        _ => {
            error!( format!("Unhandled Message Type! {:?}", msg ) );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = false;
            // global_vars.update_global_vars.emit( new_global_vars );
        }
    }
}
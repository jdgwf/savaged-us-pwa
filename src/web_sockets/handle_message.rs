use savaged_libs::player_character::chargen_data::ChargenData;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
use crate::libs::global_vars::GlobalVars;
use crate::local_storage::clear_all_local_data;
use crate::local_storage::clear_chargen_local_data;
use crate::local_storage::get_chargen_data_from_index_db;
use crate::local_storage::get_saves_from_index_db;
use crate::local_storage::index_db_save_chargen_data;
use crate::local_storage::index_db_save_saves;
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
                    // log!( format!("user {} {}", user.id, user.unread_notifications));
                    new_global_vars.current_user = user.clone();
                }
                None => {

                }
            }


            if new_global_vars.current_user.id > 0 {
                let mut global_vars_future = new_global_vars.clone();
                spawn_local(async move {

                    global_vars_future.chargen_data = get_chargen_data_from_index_db().await;
                    global_vars_future.saves = get_saves_from_index_db().await;

                    update_global_vars.emit(global_vars_future);
                });
            } else {
                let mut global_vars_future = new_global_vars.clone();
                spawn_local(async move {

                    global_vars_future.chargen_data = get_chargen_data_from_index_db().await;
                    global_vars_future.saves = get_saves_from_index_db().await;

                    global_vars_future.offline = false;
                    global_vars_future.user_loading = false;
                    // update_global_vars.emit(global_vars_future);
                    // global_vars_future.saves = global_vars_future.saves;
                    // global_vars_future.chargen_data = global_vars_future.chargen_data;

                    let saves = global_vars_future.clone().saves.unwrap_or( Vec::new() );
                    let chargen_data = &global_vars_future.clone().chargen_data.unwrap_or( ChargenData::default() );

                    // log!("new_global_vars.saves", saves.len());
                    // log!("chargen_data.books", chargen_data.books.len());
                    // log!("chargen_data.edges", chargen_data.edges.len());
                    // log!("chargen_data.hindrances", chargen_data.hindrances.len());

                    if saves.len() > 0 || chargen_data.books.len() != 2 {
                        clear_all_local_data().await;

                        global_vars_future.chargen_data = None;
                        global_vars_future.saves = None;

                    }

                    update_global_vars.emit(global_vars_future);

                    let mut msg = WebSocketMessage::default();

                    msg.token = None;
                    msg.kind = WebsocketMessageType::ChargenData;
                    new_global_vars.send_websocket.emit( msg );
                });
            }

            // spawn_local(async move {


            //     update_global_vars.emit(global_vars_future);
            // });
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
            log!( format!("handle_message ChargenData") );
            let mut new_global_vars = global_vars.clone();
            new_global_vars.chargen_data = msg.chargen_data.clone();


            let chargen_data_level = get_local_storage_string("chargen_data_level", "".to_owned() );
            let chargen_data_last_updated = get_local_storage_string("chargen_data_last_updated", "".to_owned() );

            log!( format!("handle_message ChargenData chargen_data_level {}", chargen_data_level) );
            log!( format!("handle_message ChargenData chargen_data_last_updated {}", chargen_data_last_updated) );
            // new_global_vars.user_loading = false;

            match  msg.chargen_data {
                Some( chargen_data ) => {
                    spawn_local(async move {
                        let _results  = clear_chargen_local_data().await;
                        let _results = index_db_save_chargen_data(chargen_data).await;
                        // log!( format!(" results, {:?}", results ) );
                    });
                }
                None => {}
            }



            update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::Saves => {


            let mut new_global_vars = global_vars.clone();
            new_global_vars.saves = msg.saves.clone();

            match msg.saves {
                Some( saves ) => {
                    // log!( format!("handle_message Saves {:?}", &saves) );
                    // for item in &saves {
                    //     if (&item.name).to_owned() == "Chi Master".to_owned() {
                    //         log!( format!("saves item {:?}", item) );
                    //     }
                    // }
                    spawn_local(async move {
                        let _results = index_db_save_saves(saves).await;
                        // log!( format!("÷ results, {:?}", &_results ) );
                    });
                }
                None => {}
            }
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
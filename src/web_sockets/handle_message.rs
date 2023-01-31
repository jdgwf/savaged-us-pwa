use crate::libs::global_vars::GlobalVars;
use crate::local_storage::clear_all_local_data;
use crate::local_storage::clear_game_data_local_data;
use crate::local_storage::get_game_data_from_index_db;
use crate::local_storage::get_saves_from_index_db;
use crate::local_storage::index_db_save_game_data;
use crate::local_storage::index_db_save_saves;
use gloo_console::error;
use gloo_console::log;
use savaged_libs::player_character::game_data_package::GameDataPackage;
use savaged_libs::websocket_message::{WebSocketMessage, WebsocketMessageType};
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn handle_message(
    msg: WebSocketMessage,
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
) {
    if global_vars.server_side_renderer {
        return;
    }
    // log!(format!("handle_message called"));
    match msg.kind {
        WebsocketMessageType::Online => {
            // log!(format!("handle_message Online"));
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = false;
            new_global_vars.user_loading = false;

            new_global_vars.web_content = msg.web_content;
            match msg.user {
                Some(user) => {
                    // log!( format!("user {} {}", user.id, user.unread_notifications));
                    new_global_vars.current_user = user.clone();

                }
                None => {}
            }

            if new_global_vars.current_user.id > 0 {
                let mut global_vars_future = new_global_vars.clone();
                spawn_local(async move {
                    global_vars_future.game_data = get_game_data_from_index_db().await;
                    global_vars_future.saves = get_saves_from_index_db().await;

                    update_global_vars.emit(global_vars_future);
                });
                set_local_storage_string( "UI_THEME", new_global_vars.current_user.theme_css.to_owned());
            } else {

                set_local_storage_string( "UI_THEME", "_default_".to_string());

                let mut global_vars_future = new_global_vars.clone();
                spawn_local(async move {
                    global_vars_future.game_data = get_game_data_from_index_db().await;
                    global_vars_future.saves = get_saves_from_index_db().await;

                    global_vars_future.offline = false;
                    global_vars_future.user_loading = false;
                    // update_global_vars.emit(global_vars_future);
                    // global_vars_future.saves = global_vars_future.saves;
                    // global_vars_future.game_data = global_vars_future.game_data;

                    let saves = global_vars_future.clone().saves.unwrap_or(Vec::new());
                    let game_data = &global_vars_future
                        .clone()
                        .game_data
                        .unwrap_or(GameDataPackage::default());

                    // log!("new_global_vars.saves", saves.len());
                    // log!("game_data.books", game_data.books.len());
                    // log!("game_data.edges", game_data.edges.len());
                    // log!("game_data.hindrances", game_data.hindrances.len());

                    if saves.len() > 0 || game_data.books.len() != 2 {
                        clear_all_local_data().await;

                        global_vars_future.game_data = None;
                        global_vars_future.saves = None;
                    }

                    update_global_vars.emit(global_vars_future);

                    let mut msg = WebSocketMessage::default();

                    msg.token = None;
                    msg.kind = WebsocketMessageType::GameDataPackage;
                    new_global_vars.send_websocket.emit(msg);
                });
            }

            // spawn_local(async move {

            //     update_global_vars.emit(global_vars_future);
            // });
            // log!( format!("handle_message new_global_vars {:?}", &new_global_vars ) );
            // update_global_vars.emit( new_global_vars );
        }

        WebsocketMessageType::Offline => {
            // log!(format!("handle_message Offline"));

            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = true;
            new_global_vars.user_loading = false;
            update_global_vars.emit(new_global_vars);
        }

        WebsocketMessageType::GameDataPackage => {
            // log!(format!("handle_message GameDataPackage"));
            let mut new_global_vars = global_vars.clone();
            new_global_vars.game_data = msg.game_data.clone();

            let game_data_user_level = get_local_storage_string("game_data_user_level", "".to_owned());
            let game_data_last_updated = get_local_storage_string("game_data_last_updated", "".to_owned());

            match msg.game_data {
                Some(game_data) => {
                    spawn_local(async move {
                        let _results = clear_game_data_local_data().await;
                        let _results = index_db_save_game_data(game_data).await;
                        // log!( format!(" results, {:?}", results ) );
                    });
                }
                None => {}
            }

            update_global_vars.emit(new_global_vars);
        }

        WebsocketMessageType::Saves => {
            // log!(format!("handle_message Saves"));
            let new_global_vars = global_vars.clone();
            let server_root = global_vars.server_root.to_owned();

            match msg.saves {
                Some(saves) => {
                    // log!( format!("handle_message Saves {:?}", &saves) );
                    // for item in &saves {
                    //     if (&item.name).to_owned() == "Chi Master".to_owned() {
                    //         log!( format!("saves item {:?}", item) );
                    //     }
                    // }
                    spawn_local(async move {
                        let _results = index_db_save_saves(server_root, saves).await;
                        // log!( format!("÷ results, {:?}", &_results ) );
                        let mut global_vars_future = new_global_vars.clone();
                        global_vars_future.saves = get_saves_from_index_db().await;
                        update_global_vars.emit(global_vars_future);
                    });
                }
                None => {}
            }

            // new_global_vars.offline = true;
            // new_global_vars.user_loading = false;
            // update_global_vars.emit( new_global_vars );
        }

        _ => {
            if !global_vars.server_side_renderer {
                error!(format!("Unhandled Message Type! {:?}", msg));
            }
            let mut new_global_vars = global_vars.clone();
            new_global_vars.offline = false;
            // global_vars.update_global_vars.emit( new_global_vars );
        }
    }
}

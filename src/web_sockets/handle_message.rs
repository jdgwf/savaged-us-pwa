use crate::libs::data_api::get_game_data;
use crate::libs::data_api::get_saves;
use crate::libs::global_vars::GlobalVars;
use crate::libs::site_vars::SiteVars;
use crate::local_storage::clear_all_local_data;
use crate::local_storage::clear_game_data_local_data;
use crate::local_storage::index_db_get_game_data;
use crate::local_storage::index_db_get_saves;
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
    update_site_vars: Callback<SiteVars>,
    update_global_vars: Callback<GlobalVars>,
) {
    if global_vars.site_vars.server_side_renderer {
        return;
    }
    log!(format!("handle_message called {:?}", msg.kind));
    match msg.kind {
        WebsocketMessageType::Online => {
            log!(format!("handle_message Online {:?}", msg));
            let mut new_global_vars = global_vars.clone();
            // new_global_vars.site_vars.offline = false;
            // new_global_vars.site_vars.user_loading = false;

            // new_global_vars.web_content = msg.web_content;
            match msg.user {
                Some(user) => {
                    log!( format!("user {} {}", user.id, user.unread_notifications));
                    new_global_vars.site_vars.current_user = user.clone();

                }
                None => {}
            }

            let mut global_vars_future = new_global_vars.clone();
            if new_global_vars.site_vars.current_user.id > 0 {

            // set_local_storage_string( "UI_THEME", new_global_vars.site_vars.current_user.theme_css.to_owned());

                spawn_local(async move {
                    global_vars_future.game_data = index_db_get_game_data().await;
                    global_vars_future.saves = index_db_get_saves().await;

                    // let saves = global_vars_future.clone().saves.unwrap_or(Vec::new());
                    // let game_data = &global_vars_future
                    //     .clone()
                    //     .game_data
                    //     .unwrap_or(GameDataPackage::default());


                    // get_game_data(
                    //     global_vars_future.site_vars.login_token.clone(),
                    //     &global_vars_future.site_vars,
                    //     None,
                    // ).await;
                    // get_saves(
                    //     global_vars_future.site_vars.login_token.clone(),
                    //     &global_vars_future.site_vars,
                    //     None,
                    // ).await;

                    // log!("new_global_vars.saves", saves.len());
                    // log!("game_data.books", game_data.books.len());
                    // log!("game_data.edges", game_data.edges.len());
                    // log!("game_data.hindrances", game_data.hindrances.len());

                    update_global_vars.emit(global_vars_future);
                });

                let mut send_msg = WebSocketMessage::default();
                send_msg.kind = WebsocketMessageType::GameDataPackageUpdated;
                new_global_vars.site_vars.send_websocket.emit(send_msg.clone());

                let mut send_saves_msg = WebSocketMessage::default();
                send_saves_msg.kind = WebsocketMessageType::SavesUpdated;
                new_global_vars.site_vars.send_websocket.emit(send_saves_msg);
            } else {

                // set_local_storage_string( "UI_THEME", "_default_".to_string());

                spawn_local(async move {
                    global_vars_future.game_data = index_db_get_game_data().await;
                    // global_vars_future.saves = index_db_get_saves().await;

                    global_vars_future.site_vars.offline = false;
                    global_vars_future.site_vars.user_loading = false;
                    // update_global_vars.emit(global_vars_future);
                    // global_vars_future.saves = global_vars_future.saves;
                    // global_vars_future.game_data = global_vars_future.game_data;

                    // let saves = global_vars_future.clone().saves.unwrap_or(Vec::new());
                    // let game_data = &global_vars_future
                    //     .clone()
                    //     .game_data
                    //     .unwrap_or(GameDataPackage::default());

                    // log!("new_global_vars.saves", saves.len());
                    // log!("game_data.books", game_data.books.len());
                    // log!("game_data.edges", game_data.edges.len());
                    // log!("game_data.hindrances", game_data.hindrances.len());

                    global_vars_future.saves = None;

                    match &global_vars_future.game_data {
                        Some( game_data ) => {

                            if game_data.books.len() != 2 {
                                get_game_data(
                                    global_vars_future.site_vars.login_token.clone(),
                                    &global_vars_future.site_vars,
                                    None,
                                ).await;
                            }
                        }
                        None => {
                            get_game_data(
                                global_vars_future.site_vars.login_token.clone(),
                                &global_vars_future.site_vars,
                                None,
                            ).await;
                        }
                    }

                    update_global_vars.emit(global_vars_future);

                    let mut msg = WebSocketMessage::default();

                    msg.token = None;
                    msg.kind = WebsocketMessageType::GameDataPackageUpdated;
                    new_global_vars.site_vars.send_websocket.emit(msg);
                });
            }

        }

        WebsocketMessageType::Offline => {
            // log!(format!("handle_message Offline"));

            let mut new_site_vars = global_vars.site_vars.clone();
            new_site_vars.offline = true;
            new_site_vars.user_loading = false;
            update_site_vars.emit(new_site_vars);
        }

        WebsocketMessageType::GameDataPackageUpdated => {
            log!(format!("handle_message GameDataPackage"));
            let new_global_vars = global_vars.clone();
            // let server_root = site_vars.server_root.to_owned();

            let game_data_user_level = get_local_storage_string("game_data_user_level", "".to_owned());
            let game_data_last_updated = get_local_storage_string("game_data_last_updated", "".to_owned());

            match msg.game_data {
                Some(game_data) => {
                    spawn_local(async move {
                        let _results = clear_game_data_local_data().await;
                        let _results = index_db_save_game_data(game_data).await;
                        log!( format!("index_db_save_game_data results, {:?}", _results ) );
                        // let mut global_vars_future = new_global_vars.clone();
                        // let saves = index_db_get_saves().await;
                        let game_data_option = index_db_get_game_data().await;
                        // log!( format!("index_db_get_saves complete" ));

                        new_global_vars.site_vars.update_game_data.emit(game_data_option);
                        // match game_data_option {
                        //     Some(game_data) => {

                        //     }
                        //     None => {

                        //     }
                        // }

                    });
                }
                None => {}
            }

            // update_global_vars.emit(new_global_vars);
        }

        WebsocketMessageType::SavesUpdated => {
            log!(format!("handle_message Saves"));
            let new_global_vars = global_vars.clone();
            let server_root = new_global_vars.site_vars.server_root.to_owned();

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

                        // let mut global_vars_future = new_global_vars.clone();
                        let saves_option = index_db_get_saves().await;
                        // log!( format!("index_db_get_saves complete" ));

                        match saves_option {
                            Some( saves ) => {
                                new_global_vars.site_vars.update_saves.emit(saves);
                            }
                            None => {
                                new_global_vars.site_vars.update_saves.emit(Vec::new());
                            }
                        }

                    });
                }
                None => {}
            }

            // new_site_vars.offline = true;
            // new_site_vars.user_loading = false;
            // update_site_vars.emit( new_site_vars );
        }

        _ => {
            if !global_vars.site_vars.server_side_renderer {
                error!(format!("Unhandled Message Type! {:?}", msg));
            }
            let mut new_site_vars = global_vars.site_vars.clone();
            new_site_vars.offline = false;
            // site_vars.update_site_vars.emit( new_site_vars );
        }
    }
}

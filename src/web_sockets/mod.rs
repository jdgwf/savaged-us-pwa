pub mod handle_message;

use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_console::error;
use gloo_console::log;
use gloo_net::websocket::{futures::WebSocket, Message};
use gloo_timers::future::sleep;
use savaged_libs::websocket_message::WebSocketMessage;
use savaged_libs::websocket_message::WebsocketMessageType;
use serde_json;
use std::time::Duration;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub struct WebsocketService {
    pub tx: Sender<String>,
}

impl WebsocketService {
    pub fn new(
        server_root: String,
        received_message_callback: &Callback<String>,
        websocket_offline_callback: &Callback<bool>,
        login_token: String,
    ) -> Self {
        let wss_url = server_root
            .replace("http://", "ws://")
            .replace("https://", "wss://")
            + &"/_ws".to_owned();

        log!("server_root", &server_root);
        log!("wss_url", &wss_url);
        let ws: WebSocket = WebSocket::open(&wss_url).unwrap();

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        let websocket_offline_callback_send = websocket_offline_callback.clone();
        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                // log!("got event from channel! {}", &s);
                write.send(Message::Text(s)).await.unwrap();
                websocket_offline_callback_send.clone().emit(false);
            }
        });

        // websocket_offline_callback.emit( false );

        // spawn_local(async move {
        //     send_ping( &mut &ws );
        // });

        let received_message_callback = received_message_callback.clone();
        let websocket_offline_callback = websocket_offline_callback.clone();
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(val)) => {
                        // log!("Message::Text", &val);
                        received_message_callback.emit(val.to_string());
                        websocket_offline_callback.emit(false);
                    }

                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            received_message_callback.emit(val.to_string());

                            websocket_offline_callback.emit(false);
                        }
                    }

                    Err(_e) => {
                        //error!( format!)"ws: {:?}", e.to_string()) );
                        error!("WebSocket connection failure - will try to reconnect periodically.");

                        websocket_offline_callback.emit(true);
                    }
                }
            }
            log!("WebSocket Closed...sleeping");
            let _ = sleep(Duration::from_secs(15)).await;
            log!("I'm awake, trying to reconnect...");
            websocket_offline_callback.emit(true);
        });

        let mut msg = WebSocketMessage::default();

        msg.token = Some(login_token);
        msg.kind = WebsocketMessageType::Online;

        // let global_vars_future_callback = ctx.link().callback( MainAppMessage::UpdateGlobalVars );
        let send_data_result = serde_json::to_string(&msg);

        // log!("MainWebAppMessages::SendWebSocket called");
        match send_data_result {
            Ok(send_data) => {
                // write.send( send_data );
                let _ = in_tx.clone().try_send(send_data.to_owned());
            }
            Err(_err) => {}
        }

        Self { tx: in_tx }
    }
}

pub fn connect_to_websocket(
    server_root: String,
    received_message_callback: &Callback<String>,
    websocket_offline_callback: &Callback<bool>,
    login_token: String,
) -> WebsocketService {
    // log!("connect_to_websocket() called");
    return WebsocketService::new(
        server_root.to_owned(),
        received_message_callback,
        websocket_offline_callback,
        login_token,
    );
}

// fn send_ping( ws: &mut WebSocket ) {
//     log!("send ping");
//     ws.send(Message::Text("__ping__".to_string()));

//     let timeout = Timeout::new(
//         5_000,
//         move || {

//             // send_ping( ws );
//         }
//     );

//     timeout.forget();
// }

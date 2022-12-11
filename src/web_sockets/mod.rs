pub mod handle_message;
use std::thread::sleep;

use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_net::websocket::{
    Message,
    futures::WebSocket,
    State,
};
use gloo_console::log;
use gloo_console::error;
use savaged_libs::websocket_message::WebSocketMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use crate::libs::global_vars::GlobalVars;

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

        let mut ws = WebSocket::open( &wss_url ).unwrap();

        match ws.state() {
            State::Closed => {
                ws = WebSocket::open( &wss_url ).unwrap();
            }

            _ => {

            }
        }

        log!("Attempting connection via web socket...");

        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        let websocket_offline_callback_send = websocket_offline_callback.clone();
        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                // log!("got event from channel! {}", &s);
                write.send(Message::Text(s)).await.unwrap();
                websocket_offline_callback_send.clone().emit( false );
            }
        });
        websocket_offline_callback.emit( false );

        let received_message_callback = received_message_callback.clone();
        let websocket_offline_callback = websocket_offline_callback.clone();
        spawn_local(async move {

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(val)) => {

                        received_message_callback.emit( val.to_string()  );
                        websocket_offline_callback.emit( false );
                    }

                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {

                            received_message_callback.emit( val.to_string()  );


                            websocket_offline_callback.emit( false );
                        }
                    }
                    Err(e) => {
                        error!("ws: {:?}", e.to_string());
                        websocket_offline_callback.emit( true );

                    }
                }
            }
            log!("WebSocket Closed");
            websocket_offline_callback.emit( true );

        });

        Self {
            tx: in_tx,
        }
    }

}


pub fn connect_to_websocket(
    server_root: String,
    received_message_callback: &Callback<String>,
    websocket_offline_callback: &Callback<bool>,
    login_token: String,
) -> WebsocketService {

    return WebsocketService::new(
        server_root.to_owned(),
        received_message_callback,
        websocket_offline_callback,
        login_token,
    );
}
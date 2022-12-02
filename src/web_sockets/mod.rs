pub mod handle_message;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use gloo_net::websocket::{
    Message,
    futures::WebSocket,

};
use gloo_console::log;
use gloo_console::error;
use savaged_libs::websocket_message::WebSocketMessage;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use crate::libs::global_vars::GlobalVars;

pub struct WebsocketService {
    pub tx: Sender<String>,
    // pub global_vars: GlobalVars
}

impl WebsocketService {
    pub fn new(
        server_root: String,
        received_message_callback: Callback<String>,
    ) -> Self {

        let wss_url = server_root
        .replace("http://", "ws://")
        .replace("https://", "wss://")
        + &"/_ws".to_owned();
        let ws = WebSocket::open(wss_url.as_ref()).unwrap();
        log!("WebSocket Connected to ", &wss_url);
        let (mut write, mut read) = ws.split();

        let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

        spawn_local(async move {
            while let Some(s) = in_rx.next().await {
                // log!("got event from channel! {}", &s);
                write.send(Message::Text(s)).await.unwrap();
            }
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(val)) => {
                        received_message_callback.emit( val.to_string()  );
                        // log!("from websocket: {}", &data);
                    }
                    Ok(Message::Bytes(b)) => {
                        let decoded = std::str::from_utf8(&b);
                        if let Ok(val) = decoded {
                            received_message_callback.emit( val.to_string()  );
                            // log!("from websocket: {}", val);
                        }
                    }
                    Err(e) => {
                        error!("ws: {:?}", e.to_string())
                    }
                }
            }
            log!("WebSocket Closed");
        });

        Self { tx: in_tx }
    }
}

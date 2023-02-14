use savaged_libs::websocket_message::WebSocketMessage;
use savaged_libs::websocket_message::WebsocketMessageType;
use yew::Callback;
use web_sys;

pub mod admin_api;
pub mod fetch_api;
pub mod global_vars;
pub mod site_vars;

pub fn websocket_set_location(
    send_websocket: Callback<WebSocketMessage>,
    location: String,
) {
    let msg = WebSocketMessage {
        kind: WebsocketMessageType::SetLocation,
        payload: Some( location ),
        ..Default::default()
    };
    send_websocket.emit( msg );
}

pub fn websocket_set_room(
    send_websocket: Callback<WebSocketMessage>,
    room_id: String,
) {
    let msg = WebSocketMessage {
        kind: WebsocketMessageType::SetRoom,
        payload: Some( room_id ),
        ..Default::default()
    };
    send_websocket.emit( msg );
}


use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod pages;
mod main_app;
mod libs;
mod components;

// use gloo_console::log;
use main_app::MainApp;
use crate::libs::global_vars::GlobalVars;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use savaged_libs::user::User;
use gloo_console::log;
use gloo_net::websocket::{
    Message,
    futures::WebSocket,

};
use futures::{SinkExt, StreamExt};
// use futures::{FutureExt, StreamExt};

#[function_component]
fn App() -> Html {

    let login_token = get_local_storage_string( "login_token", "".to_owned() );
    let mut user_loading = true;

    if login_token.is_empty() {
        user_loading = false;
    }

    let server_root = "https://v4.savaged.us".to_owned();
    // let server_root = "http://localhost:5001".to_owned();
    // let server_root = "https://savaged.us".to_owned();
    // let server_root = "https://staging.savaged.us".to_owned();



    let mut offline = false;

    let wss_url = server_root
    .replace("http://", "ws://")
    .replace("https://", "wss://")
    + &"/_ws".to_owned();

    let mut ws = WebSocket::open(
        wss_url.as_ref()
    ).unwrap();

    // if ws_result.is_err() {
    //     log!("WebSocket Failed! {}", err.to_string() );
    //     offline = true;
    // }
    // match ws_result.as_mut() {
    //     Ok( ws ) => {
    //         log!("WebSocket Connected!");
    //         offline = false;
    //     }

    //     Err( err ) => {
    //         log!("WebSocket Failed! {}", err.to_string() );
    //         offline = true;
    //     }

    // }

    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        write.send(Message::Text(String::from("test"))).await.unwrap();
        write.send(Message::Text(String::from("test 2"))).await.unwrap();
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            log!(format!("1. {:?}", msg))
        }
        log!("WebSocket Closed")
    });

    let global_vars_state = use_reducer(
        || GlobalVars {
            login_token:  login_token,
            current_user: User::default(),
            user_loading: user_loading,
            server_root: server_root.to_owned(),
            api_root: server_root + &"/_api",
            site_title: "v4.savaged.us".to_owned(),
            no_calls: false,
            offline: offline,
            update_global_vars: Callback::noop(),
        }
    );

    html! {
        <ContextProvider<GlobalVarsContext>
            context={global_vars_state}
        >
            <MainApp />
        </ContextProvider<GlobalVarsContext>>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

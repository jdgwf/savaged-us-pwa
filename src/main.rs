use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

mod pages;
mod main_app;
mod libs;
mod components;
mod web_sockets;

// use gloo_console::log;
use main_app::MainApp;
use crate::libs::global_vars::GlobalVars;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use savaged_libs::user::User;
use gloo_console::log;

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

    // let mut offline = false;


    let global_vars_state = use_reducer(
        || GlobalVars {
            login_token:  login_token,
            current_user: User::default(),
            user_loading: user_loading,
            server_root: server_root.to_owned(),
            api_root: server_root + &"/_api",
            site_title: "v4.savaged.us".to_owned(),
            no_calls: false,
            offline: false,
            update_global_vars: Callback::noop(),
            send_websocket: Callback::noop(),
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

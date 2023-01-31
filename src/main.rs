mod components;
mod libs;
mod local_storage;
mod main_app;
mod menu_items;
mod pages;
mod web_sockets;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
use crate::libs::global_vars::GlobalVars;
use main_app::MainApp;
use savaged_libs::user::User;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let login_token = get_local_storage_string("login_token", "".to_owned());
    let mut user_loading = true;

    if login_token.is_empty() {
        user_loading = false;
    }

    let server_root = "https://v4.savaged.us".to_owned();
    // let server_root = "http://localhost:5001".to_owned();
    // let server_root = "https://savaged.us".to_owned();
    // let server_root = "https://staging.savaged.us".to_owned();

    let global_vars_state = use_reducer(|| GlobalVars {
        api_root: server_root.to_owned() + &"/_api",
        current_menu: "".to_owned(),
        current_sub_menu: "".to_owned(),
        current_user: User::default(),
        game_data: None,
        hide_popup_menus_callback: Callback::noop(),
        add_alert: Callback::noop(),
        login_token: login_token,
        logout_callback: Callback::noop(),
        offline: false,
        open_confirmation_dialog: Callback::noop(),
        saves: None,
        send_websocket: Callback::noop(),
        server_root: server_root.to_owned(),
        server_side_renderer: false,
        server_side_renderer_history: None,
        show_mobile_menu: false,
        site_title: "v4.savaged.us".to_owned(),
        toggle_mobile_menu_callback: Callback::noop(),
        update_global_vars: Callback::noop(),
        user_loading: user_loading,
        web_content: None,
    });

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

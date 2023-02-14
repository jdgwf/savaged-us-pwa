mod components;
mod libs;
mod local_storage;
mod main_app;
mod menu_items;
mod pages;
mod web_sockets;
pub type GlobalVarsContext = UseReducerHandle<GlobalVars>;
pub type WebContentContext = UseReducerHandle<WebContent>;
use crate::libs::global_vars::GlobalVars;
use crate::libs::site_vars::SiteVars;
use main_app::MainApp;
use savaged_libs::{user::User, web_content::WebContent};
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
    // let server_roxot = "http://localhost:5001".to_owned();
    // let server_root = "https://savaged.us".to_owned();
    // let server_root = "https://staging.savaged.us".to_owned();

    let global_vars_state = use_reducer(|| GlobalVars {
        game_data: None,
        saves: None,
        site_vars: SiteVars::default(),
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

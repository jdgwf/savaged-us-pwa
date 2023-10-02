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
use savaged_libs::web_content::WebContent;
use yew::prelude::*;

#[function_component]
fn App() -> Html {

    let mut site_vars = SiteVars::default();

    site_vars.server_root = "https://v4-rust.savaged.us".to_owned();
    // site_vars.server_root = "http://localhost:5001".to_owned();
    // site_vars.server_root = "https://savaged.us".to_owned();
    // site_vars.server_root = "https://staging.savaged.us".to_owned();

    site_vars.api_root = site_vars.server_root.to_owned() + "/" + &site_vars.api_root;

    let global_vars_state = use_reducer(|| GlobalVars {
        game_data: None,
        saves: None,
        site_vars: site_vars,
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

use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
use yew::prelude::*;
use standard_components::ui::nbsp::Nbsp;

use standard_components::libs::set_document_title::set_document_title;

// use crate::lib::fetch_api::fetch_api;
// use crate::lib::fetch_api::savaged_login;

// use web_sys::console;
// use wasm_bindgen_futures::spawn_local;
// use gloo_utils::format::JsValueSerdeExt;
use crate::libs::global_vars::GlobalVars;

// use savaged_libs::user::User;
// use savaged_libs::user::LoginTokenResult;

#[derive(Properties, PartialEq)]
pub struct MainHomeProps {
    pub global_vars: GlobalVars,
}

pub enum MainHomeMessage {

}

pub struct MainHome {

}

impl Component for MainHome {
    type Message = MainHomeMessage;
    type Properties = MainHomeProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Home".to_owned(), global_vars.no_calls,);
        MainHome {

        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = ctx.props().global_vars.clone();

        html! {
            <div class={"main-content"}>
                <h2><i class="fa fa-house" /><Nbsp />{"Home Page"}</h2>
                <hr />
                {"This is an RPG Awesome Icon:"}<Nbsp /><i class="ra  ra-dinosaur " />
                <hr />
                <button
                    class="btn"
                    onclick={ move |_e| {
                        let login_token = global_vars.login_token.to_owned();
                        let mut login_token_send: Option<String> = None;
                        if !login_token.is_empty() {
                            login_token_send = Some(login_token);
                        }
                        let msg = WebSocketMessage {
                            token: login_token_send,
                            kind: WebsocketMessageType::Online,
                            user: None,
                            payload: None,
                        };
                        global_vars.send_websocket.emit( msg );
                    }}
                >
                    {"Clicky"}
                </button>
            </div>

        }

    }
}

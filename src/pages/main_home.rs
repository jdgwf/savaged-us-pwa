use savaged_libs::player_character::chargen_data::ChargenData;
use savaged_libs::save_db_row::SaveDBRow;
use savaged_libs::websocket_message::{
    WebSocketMessage,
    WebsocketMessageType,
};
use yew::prelude::*;
use standard_components::ui::nbsp::Nbsp;

use standard_components::libs::set_document_title::set_document_title;

// use crate::lib::fetch_api::fetch_api;
// use crate::lib::fetch_api::savaged_login;

use gloo_console::log;
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
    chargen_data: Option<ChargenData>,
    saves: Option<Vec<SaveDBRow>>,
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
            chargen_data: None,
            saves: None,
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = ctx.props().global_vars.clone();
        let global_vars2 = ctx.props().global_vars.clone();

        let mut saves_html = html!{<></>};
        let mut chargen_html = html!{<></>};


        match &global_vars.chargen_data {
            Some( chargen_data ) => {
                chargen_html = html!{
                    <>
                    {"Books: "}{chargen_data.books.len()}<br />
                    {"Edges: "}{chargen_data.edges.len()}<br />
                    {"Hindrances: "}{chargen_data.hindrances.len()}<br />
                    </>
                };
            }
            None => {}
        }
        match &global_vars.saves {
            Some( saves ) => {
                saves_html = html!{
                    <>
                        {"saves: "}{saves.len()}<br />
                    </>
                };
            }
            None => {}
        }

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
                            kind: WebsocketMessageType::ChargenData,
                            user: None,
                            payload: None,
                            updated_on: None,
                            saves: None,
                            chargen_data: None,
                        };
                        global_vars.send_websocket.emit( msg );
                    }}
                >
                    {"Request Chargen Data"}
                </button>

                if global_vars2.current_user.id > 0 {
                <button
                    class="btn"
                    onclick={ move |_e| {
                        let login_token = global_vars2.login_token.to_owned();
                        let mut login_token_send: Option<String> = None;
                        if !login_token.is_empty() {
                            login_token_send = Some(login_token);
                        }
                        let msg = WebSocketMessage {
                            token: login_token_send,
                            kind: WebsocketMessageType::Saves,
                            user: None,
                            payload: None,
                            updated_on: None,
                            saves: None,
                            chargen_data: None,
                        };
                        global_vars2.send_websocket.emit( msg );
                    }}
                >
                    {"Request Saves"}
                </button>
                }

                <div class="row">
                    <div class="col-6">
                        {chargen_html}
                    </div>
                    <div class="col-6">
                        {saves_html}
                    </div>
                </div>
            </div>

        }

    }
}

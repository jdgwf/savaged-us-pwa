use yew::prelude::*;
use crate::components::alerts::AlertDefinition;
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use savaged_libs::alert_level::AlertLevel;
use standard_components::ui::nbsp::Nbsp;

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
        _ctx: &Context<Self>
    ) -> Self {

        // let global_vars = ctx.props().global_vars.clone();

        MainHome {
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = ctx.props().global_vars.clone();
        // let global_vars2 = ctx.props().global_vars.clone();

        let mut saves_html = html!{<></>};
        let mut game_data_html = html!{<></>};

        match &global_vars.game_data {
            Some( game_data ) => {
                game_data_html = html!{
                    <>
                    {"Books: "}{game_data.books.len()}<br />
                    {"Edges: "}{game_data.edges.len()}<br />
                    {"Hindrances: "}{game_data.hindrances.len()}<br />
                    <br />
                    {"Gear: "}{game_data.gear.len()}<br />
                    {"Armor: "}{game_data.armor.len()}<br />
                    {"Weapons: "}{game_data.weapons.len()}<br />
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

        let global_vars_copy = ctx.props().global_vars.clone();
        html! {
            <UIPage
                global_vars={ctx.props().global_vars.clone()}
                page_title="Home"
            >

                <h2><i class="fa fa-house" /><Nbsp />{"Home Page"}</h2>
                <hr />
                <div class="alert alert-info">
                    {"Note, this is using LIVE data from"}<Nbsp /><a href="https://savaged.us">{"https://savaged.us"}</a>{". However it's not using the live database. Every morning at 03:00CST the entirety of the live database is copied here. Which means your changes, settings, and login tokens at this URL will all be overwritten every day."}<br />
                    <br />
                    {"This is good, because this is a ground-up rewrite and there certainly will be breaking changes to our data - especially this early in development."}
                </div>
                // {"This is an RPG Awesome Icon:"}<Nbsp /><i class="ra  ra-dinosaur " />
                <hr />
                <button
                    class="btn"
                        onclick={ move |_e| {
                        global_vars_copy.add_alert.emit(
                            AlertDefinition {
                                level: AlertLevel::Info,
                                text: Some("Hello!".to_owned()),
                                ..Default::default()
                            }
                        );
                    }}
                >
                    {"Add Alert"}
                </button>
                <hr />
                // <h3>{"Data Retrieval Event Buttons"}</h3>
                // <button
                //     class="btn"
                //     onclick={ move |_e| {
                //         let login_token = global_vars.login_token.to_owned();
                //         let mut login_token_send: Option<String> = None;
                //         if !login_token.is_empty() {
                //             login_token_send = Some(login_token);
                //         }

                //         let mut msg = WebSocketMessage::default();

                //         msg.token = login_token_send;
                //         msg.kind = WebsocketMessageType::GameDataPackage;

                //         global_vars.send_websocket.emit( msg );
                //     }}
                // >
                //     {"Request GameDataRow Data"}
                // </button>

                // if global_vars2.current_user.id > 0 {
                // <button
                //     class="btn"
                //     onclick={ move |_e| {
                //         let login_token = global_vars2.login_token.to_owned();
                //         let mut login_token_send: Option<String> = None;
                //         if !login_token.is_empty() {
                //             login_token_send = Some(login_token);
                //         }

                //         let mut msg = WebSocketMessage::default();

                //         msg.token = login_token_send;
                //         msg.kind = WebsocketMessageType::Saves;

                //         global_vars2.send_websocket.emit( msg );
                //     }}
                // >
                //     {"Request Saves"}
                // </button>
                // }

                <div class="row">
                    <div class="col-6">
                        <h4>{"GameDataRow Data Counts"}</h4>
                        {game_data_html}
                    </div>
                    <div class="col-6">
                        <h4>{"Saves Count"}</h4>
                        {saves_html}
                    </div>
                </div>
            </UIPage>

        }

    }
}

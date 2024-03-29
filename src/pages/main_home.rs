// use crate::components::alerts::AlertDefinition;
use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use standard_components::ui::content_box::ContentBox;
// use gloo_console::log;
// use savaged_libs::alert_level::AlertLevel;
use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MainHomeProps {
    pub site_vars: SiteVars,
}

pub enum MainHomeMessage {}

pub struct MainHome {}

impl Component for MainHome {
    type Message = MainHomeMessage;
    type Properties = MainHomeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        // let site_vars = ctx.props().site_vars.clone();

        MainHome {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let site_vars = ctx.props().site_vars.clone();
        // let site_vars2 = ctx.props().site_vars.clone();

        // let site_vars_copy = ctx.props().site_vars.clone();
        html! {
            <UIPage
                site_vars={site_vars}
                page_title="Home"
            >

            <div class="row home-points">
            <div class="col-md-6"></div>
            <div class="col-md-6">
            <h2 class="text-center home-h2">{"Create ANY Savage Worlds Character In Minutes!"}</h2>
            <ContentBox>

                <div class="row home-points">
                    <div class="col-md-6">
                        <p><strong>{"FREE"}</strong><Nbsp />{"Basic character creation"}</p>
                        <p>{"Export to popular Virtual Tabletops"}
                            <ul>
                                <li>{"Roll 20"}</li>
                                <li>{"Fantasy Grounds"}</li>
                                <li>{"Foundry"}</li>
                            </ul>
                        </p>
                        <p>{"Many ready-to-go built-in settings, or create your own!"}</p>
                    </div>
                    <div class="col-md-6">
                        <p>{"Share your campaigns and characters with your players"}</p>

                        <p>{"Create custom vehicles from the Science Fiction Companion"}</p>

                        <p>{"Create your own custom Edges, Hindrances, Gear, and More"}</p>
                    </div>
                </div>
            </ContentBox>
            </div>
            </div>
                // <h2><i class="fa fa-house" /><Nbsp />{"Home Page"}</h2>
                // <hr />
                // <div class="alert alert-info">
                //     {"Note, this is using LIVE data from"}<Nbsp /><a href="https://savaged.us">{"https://savaged.us"}</a>{". However it's not using the live database. Every morning at 03:00CST the entirety of the live database is copied here. Which means your changes, settings, and login tokens at this URL will all be overwritten every day."}<br />
                //     <br />
                //     {"This is good, because this is a ground-up rewrite and there certainly will be breaking changes to our data - especially this early in development."}
                // </div>
                // // {"This is an RPG Awesome Icon:"}<Nbsp /><i class="ra  ra-dinosaur " />
                // <hr />
                // <button
                //     class="btn"
                //         onclick={ move |_e| {
                //         site_vars_copy.add_alert.emit(
                //             AlertDefinition {
                //                 level: AlertLevel::Info,
                //                 text: Some("Hello!".to_owned()),
                //                 ..Default::default()
                //             }
                //         );
                //     }}
                // >
                //     {"Add Alert"}
                // </button>
                // <hr />
                // <h3>{"Data Retrieval Event Buttons"}</h3>
                // <button
                //     class="btn"
                //     onclick={ move |_e| {
                //         let login_token = site_vars.login_token.to_owned();
                //         let mut login_token_send: Option<String> = None;
                //         if !login_token.is_empty() {
                //             login_token_send = Some(login_token);
                //         }

                //         let mut msg = WebSocketMessage::default();

                //         msg.token = login_token_send;
                //         msg.kind = WebsocketMessageType::GameDataPackageUpdated;

                //         site_vars.send_websocket.emit( msg );
                //     }}
                // >
                //     {"Request GameDataRow Data"}
                // </button>

                // if site_vars2.current_user.id > 0 {
                // <button
                //     class="btn"
                //     onclick={ move |_e| {
                //         let login_token = site_vars2.login_token.to_owned();
                //         let mut login_token_send: Option<String> = None;
                //         if !login_token.is_empty() {
                //             login_token_send = Some(login_token);
                //         }

                //         let mut msg = WebSocketMessage::default();

                //         msg.token = login_token_send;
                //         msg.kind = WebsocketMessageType::SavesUpdated;

                //         site_vars2.send_websocket.emit( msg );
                //     }}
                // >
                //     {"Request Saves"}
                // </button>
                // }

                // <div class="row">
                //     <div class="col-6">
                //         <h4>{"GameDataRow Data Counts"}</h4>
                //         {game_data_html}
                //     </div>
                //     <div class="col-6">
                //         <h4>{"Saves Count"}</h4>
                //         {saves_html}
                //     </div>
                // </div>
            </UIPage>

        }
    }
}

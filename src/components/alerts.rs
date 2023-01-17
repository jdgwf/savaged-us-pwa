use yew::prelude::*;
use crate::libs::{global_vars::GlobalVars};
use savaged_libs::alert_level::AlertLevel;
use uuid::{Uuid};
use gloo_timers::callback::Timeout;


#[derive(Clone, Debug, PartialEq)]
pub struct AlertDefinition {
    pub level: AlertLevel,
    pub html: Option<Html>,
    pub text: Option<String>,
    pub uuid: Uuid,
    pub active_class: String,
}

impl Default for AlertDefinition {
    fn default() -> Self {
        AlertDefinition {
            level: AlertLevel::Info,
            html: None,
            text: None,
            uuid: Uuid::new_v4(),
            active_class: "".to_owned(),

        }
    }

}

#[derive(Properties, PartialEq)]
pub struct AlertsProps {
    pub global_vars: GlobalVars,
    pub alerts: Vec<AlertDefinition>,
}

pub enum AlertsMessage {

}

#[function_component(Alerts)]
pub fn alerts_component(
    props: &AlertsProps,
) -> Html {



    html! {
        <div class={"alerts-container"}>

        {props.alerts.clone().into_iter().map( | alert | {
            let mut alert_class = "alert".to_owned();

            match alert.level {
                AlertLevel::Info => {
                    alert_class = alert_class + &" alert-info";
                }
                AlertLevel::Warning => {
                    alert_class = alert_class + &" alert-warning";
                }
                AlertLevel::Danger => {
                    alert_class = alert_class + &" alert-danger";
                }
                AlertLevel::Secondary => {
                    alert_class = alert_class + &" alert-secondary";
                }
                AlertLevel::Success => {
                    alert_class = alert_class + &" alert-success";
                }

            }

            let mut definition_html = html!{<></>};
            let mut definition_text = "".to_string();

            match alert.html {
                Some( html ) => {
                    definition_html = html.clone();
                }
                None => {}
            }
            match alert.text {
                Some( html ) => {
                    definition_text = html.clone();
                }
                None => {}
            }

            alert_class = alert_class + &" " + &alert.active_class;

            html!{
                <div id={alert.uuid.to_string()} class={alert_class}>
                    {definition_text}
                    {definition_html}
                    <hr />
                    {alert.uuid.to_string()}
                </div>
            }
        }).collect::<Html>()}

        </div>

    }


}

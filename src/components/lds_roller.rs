use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct LDSRollerProps {}

#[function_component(LDSRoller)]
pub fn lds_roller(_props: &LDSRollerProps) -> Html {
    html! {

        <div class="lds-roller"><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div></div>
    }
}

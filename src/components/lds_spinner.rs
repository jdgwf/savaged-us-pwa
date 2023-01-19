use yew::{function_component, Properties, Html, html};

#[derive(Properties, PartialEq)]
pub struct LDSSpinnerProps {
}

#[function_component(LDSSpinner)]
pub fn lds_spinner(
    _props: &LDSSpinnerProps,
) -> Html {

    html! {

        <div class="lds-spinner"><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div></div>
    }
}
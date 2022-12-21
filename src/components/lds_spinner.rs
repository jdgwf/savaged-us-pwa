use yew::{function_component, Properties, Html, html};

#[derive(Properties, PartialEq)]
pub struct LDSSpinnerProps {
    // pub global_vars: GlobalVars,
}

#[function_component(LDSSpinner)]
pub fn lds_spinner(
    _props: &LDSSpinnerProps,
) -> Html {

    html! {

        <div class="lds-spinner"><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div></div>
    }
}
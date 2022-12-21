use yew::{function_component, Properties, Html, html};

#[derive(Properties, PartialEq)]
pub struct LDSRollerProps {
    // pub global_vars: GlobalVars,
}

#[function_component(LDSRoller)]
pub fn lds_roller(
    _props: &LDSRollerProps,
) -> Html {

    html! {

        <div class="lds-roller"><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div></div>
    }
}
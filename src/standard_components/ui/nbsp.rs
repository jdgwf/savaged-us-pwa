use yew::{function_component, html};

#[function_component(Nbsp)]
pub fn nbsp_html() -> Html {
    html! { "\u{00a0}" }
}


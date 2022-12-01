use yew::{function_component, Html, Properties};
use web_sys::{Element, Node};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub inner_html: String,
}

#[function_component(RawHtml)]
pub fn raw_html(props: &Props) -> Html {
    let div: Element = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.inner_html.clone());
    let node: Node = div.into();
    Html::VRef(node)
}
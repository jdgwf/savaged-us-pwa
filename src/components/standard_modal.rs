use yew::{function_component, Properties, Html, html, Children};

#[derive(Properties, PartialEq)]
pub struct StandardModalProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub xl: bool,

}

#[function_component(StandardModal)]
pub fn standard_modal(
    props: &StandardModalProps,
) -> Html {

    let mut class="modal-container".to_owned();
    if props.xl {
        class = "modal-container modal-xl".to_owned();
    }
    html! {

        <div class={class}>
            <div class="modal-dialog">
                <div class="modal-body">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
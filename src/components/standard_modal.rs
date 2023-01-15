use yew::{function_component, Properties, Html, html, Children};

#[derive(Properties, PartialEq)]
pub struct StandardModalProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub xl: bool,

    #[prop_or_default]
    pub title: Option<String>,

}

#[function_component(StandardModal)]
pub fn standard_modal(
    props: &StandardModalProps,
) -> Html {

    let mut class="modal-container".to_owned();
    if props.xl {
        class = "modal-container modal-xl".to_owned();
    }
    let mut modal_header = html!{<></>};
    match &props.title {
        Some( title ) => {
            modal_header = html!{
                <div class="modal-head">
                    <h3 class="text-center">{title}</h3>
                </div>
            };
                // "modal-container modal-xl".to_owned();
        }
        None => {}

    }
    html! {

        <div class={class}>
            <div class="modal-dialog">
                {modal_header}
                <div class="modal-body">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
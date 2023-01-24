use standard_components::ui::standard_form_save_buttons::StandardFormSaveButtons;
use yew::{function_component, html, AttrValue, Callback, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct StandardModalProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub xl: bool,

    #[prop_or_default]
    pub title: Option<AttrValue>,

    #[prop_or_default]
    pub close_cancel_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub add_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub save_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub save_as_new_callback: Option<Callback<bool>>,

    #[prop_or_default]
    pub add_label: Option<AttrValue>,

    #[prop_or_default]
    pub save_label: Option<AttrValue>,

    #[prop_or_default]
    pub save_as_new_label: Option<AttrValue>,

    #[prop_or_default]
    pub save_and_leave_open_callback: Option<Callback<bool>>,
}

#[function_component(StandardModal)]
pub fn standard_modal(props: &StandardModalProps) -> Html {
    let mut class = "modal-container".to_owned();
    if props.xl {
        class = "modal-container modal-xl".to_owned();
    }
    let mut modal_header = html! {<></>};
    match &props.title {
        Some(title) => {
            modal_header = html! {
                <div class="modal-head">
                    <h3 class="text-center">{title}</h3>
                </div>
            };
            // "modal-container modal-xl".to_owned();
        }
        None => {}
    }

    let mut modal_footer = html!(<></>);

    match &props.close_cancel_callback {
        Some(_close_cancel_callback) => {
            modal_footer = html! {
                <div class="modal-foot">
                    <StandardFormSaveButtons
                        close_cancel_callback={_close_cancel_callback.clone()}
                        save_callback={props.save_callback.clone()}
                        add_callback={props.add_callback.clone()}
                        save_as_new_callback={props.save_as_new_callback.clone()}
                        save_and_leave_open_callback={props.save_and_leave_open_callback.clone()}
                    />
                </div>
            }
        }
        None => {}
    }
    html! {

        <div class={class}>
            <div class="modal-dialog">
            <form>
                {modal_header}
                <div class="modal-body">
                    { for props.children.iter() }
                </div>
                {modal_footer}
            </form>
            </div>
        </div>
    }
}

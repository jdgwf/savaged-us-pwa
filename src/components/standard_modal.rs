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

    let mut close_cancel_label = "Close".to_owned();
    let mut save_label = "Save".to_owned();
    let mut add_label = "Add".to_owned();

    match &props.add_label {
        Some(ov) => add_label = ov.to_string(),
        None => {}
    }

    match &props.save_label {
        Some(ov) => save_label = ov.to_string(),
        None => {}
    }


    let mut add_button = html! {<></>};
    let mut save_button = html! {<></>};
    let mut close_cancel_callback = Callback::noop();

    match &props.close_cancel_callback {
        Some( cb ) => {
            close_cancel_callback = cb.clone();
        }
        None => {}
    }

    match &props.add_callback {
        Some(cb) => {
            let the_callback = cb.clone();
            close_cancel_label = "Cancel".to_owned();
            add_button = html! {
                <button
                    class="btn btn-success"
                    type="button"
                    title={add_label}
                    onclick={move |_e | {
                        the_callback.emit(true);
                    }}
                >
                    <i class="fa fa-plus" />
                </button>
            };
        }
        None => {}
    }


    match &props.save_callback {
        Some(cb) => {
            let the_callback = cb.clone();
            close_cancel_label = "Cancel".to_owned();
            save_button = html! {
                <button
                    class="btn btn-success"
                    type="submit"
                    title={save_label}
                    onclick={move |_e | {
                        the_callback.emit(false);
                    }}
                >
                    <i class="fa fa-floppy-disk" />
                </button>
            };
        }
        None => {}
    }


    let close_button = html! {
        <button
            class="btn btn-secondary"
            type="button"
            title={close_cancel_label}
            onclick={move |_e | {
                close_cancel_callback.emit(false);
            }}
        >
            <i class="fa fa-cancel" />
        </button>
    };

    let mut modal_header = html! {<></>};
    match &props.title {
        Some(title) => {
            modal_header = html! {
                <div class="modal-head">
                    <div class="pull-right">
                        {close_button}
                        {save_button}
                        {add_button}
                    </div>
                    <h3 class="text-center">{title}</h3>
                </div>
            };
            // "modal-container modal-xl".to_owned();
        }
        None => {
            modal_header = html! {
                <div class="modal-head">
                    <div class="text-right">
                        {close_button}
                        {save_button}
                        {add_button}
                    </div>
                </div>
            };
            // "modal-container modal-xl".to_owned();
        }
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

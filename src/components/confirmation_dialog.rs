use standard_components::ui::nbsp::Nbsp;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct ConfirmationDialogDefinition {
    pub title: Option<String>,
    pub callback: Callback<bool>,
    pub html: Option<Html>,
    pub text: Option<String>,
    pub label_yes: Option<String>,
    pub label_no: Option<String>,
}
impl Default for ConfirmationDialogDefinition {
    fn default() -> Self {
        ConfirmationDialogDefinition {
            title: Some("Confirmation Required".to_owned()),
            callback: Callback::noop(),
            html: None,
            text: None,
            label_yes: None,
            label_no: None,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ConfirmationDialogProps {
    pub close_confirmation_dialog: Callback<MouseEvent>,
    pub definition: ConfirmationDialogDefinition,
}

pub enum ConfirmationDialogMessage {}

pub struct ConfirmationDialog {}

impl Component for ConfirmationDialog {
    type Message = ConfirmationDialogMessage;
    type Properties = ConfirmationDialogProps;

    fn create(ctx: &Context<Self>) -> Self {

        ConfirmationDialog {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cancel_action = ctx.props().close_confirmation_dialog.clone();
        let yes_cancel_action = ctx.props().close_confirmation_dialog.clone();
        let yes_callback = ctx.props().definition.callback.clone();

        let mut definition_title = "".to_owned();
        let mut definition_html = html! {<></>};
        let mut definition_text = "Are you sure you want to do this?".to_owned();
        match &ctx.props().definition.title {
            Some(title) => {
                definition_title = title.clone();
            }
            None => {}
        }
        match &ctx.props().definition.html {
            Some(html) => {
                definition_html = html.clone();
            }
            None => {}
        }
        match &ctx.props().definition.text {
            Some(html) => {
                definition_text = html.clone();
            }
            None => {}
        }

        let definition_label_yes = ctx
            .props()
            .definition
            .label_yes
            .clone()
            .unwrap_or("Yes".to_owned());
        let definition_label_no = ctx
            .props()
            .definition
            .label_no
            .clone()
            .unwrap_or("No, thank you".to_owned());

        html! {
            <div class={"modal-container"}>

                <div class={"modal-dialog"}>
                    if !definition_title.is_empty() {
                        <div class={"modal-head"}>
                            <h3 class="text-center">{definition_title}</h3>
                        </div>
                    }
                    <div class={"modal-body"}>
                    {definition_html}
                    if !definition_text.is_empty() {
                        <p>{definition_text}</p>
                    }
                    </div>
                    <div class={"modal-foot"}>
                        <button
                            class="btn btn-secondary"
                            type="button"
                            onclick={cancel_action}
                        >
                            {definition_label_no}
                        </button>
                        <Nbsp />
                        <button
                            class="btn btn-primary"
                            type="submit"
                            onclick={move |e| {

                                yes_callback.emit( true );
                                yes_cancel_action.emit( e );
                            }}
                        >
                            {definition_label_yes}
                        </button>
                    </div>
                </div>

            </div>

        }
    }
}

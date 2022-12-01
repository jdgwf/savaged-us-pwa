use yew::prelude::*;
use standard_components::ui::nbsp::Nbsp;
use crate::libs::global_vars::GlobalVars;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfirmationDialogDefinition {
    pub title: String,
    pub callback: Callback<bool>,
    pub html: Html,
    pub text: String,
    pub label_yes: String,
    pub label_no: String,
}
impl Default for ConfirmationDialogDefinition {
    fn default() -> Self {
        ConfirmationDialogDefinition {
            title: "Confirmation Required".to_owned(),
            callback: Callback::noop(),
            html: html! {<></>},
            text: "Are you sure you want to do this?".to_owned(),
            label_yes: "Yes".to_owned(),
            label_no: "No".to_owned(),
        }
    }

}

#[derive(Properties, PartialEq)]
pub struct ConfirmationDialogProps {
    pub global_vars: GlobalVars,
    pub close_confirmation_dialog: Callback<MouseEvent>,
    pub definition: ConfirmationDialogDefinition,
}

pub enum ConfirmationDialogMessage {

}

pub struct ConfirmationDialog {

}

impl Component for ConfirmationDialog {
    type Message = ConfirmationDialogMessage;
    type Properties = ConfirmationDialogProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let _global_vars = ctx.props().global_vars.clone();

        ConfirmationDialog {

        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let cancel_action = ctx.props().close_confirmation_dialog.clone();
        let yes_cancel_action = ctx.props().close_confirmation_dialog.clone();
        let yes_callback = ctx.props().definition.callback.clone();

        html! {
            <div class={"modal-container"}>

                <div class={"modal-dialog"}>
                    <div class={"modal-header"}>
                        {&ctx.props().definition.title}
                    </div>
                    <div class={"modal-body"}>
                    if ctx.props().definition.text.is_empty() {
                        {ctx.props().definition.html.clone()}
                    } else {
                        {ctx.props().definition.text.clone()}
                    }
                    </div>
                    <div class={"modal-footer"}>
                        <button
                            class="btn btn-secondary"
                            onclick={cancel_action}
                        >
                            {&ctx.props().definition.label_no}
                        </button>
                        <Nbsp />
                        <button
                            class="btn btn-primary"
                            onclick={move |e| {

                                yes_callback.emit( true );
                                yes_cancel_action.emit( e );
                            }}
                        >
                            {&ctx.props().definition.label_yes}
                        </button>
                    </div>
                </div>

            </div>

        }

    }
}

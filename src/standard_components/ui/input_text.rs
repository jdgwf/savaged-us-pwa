use yew::prelude::*;
use super::super::internal::ui::input_label::InputLabel;
use web_sys::{HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct InputTextProps {

    #[prop_or_default]
    pub onchange: Callback<String>,

    #[prop_or_default]
    pub input_type: String,

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub description: String,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub r#type: String,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub inline: bool,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub label_class: String,

    #[prop_or_default]
    pub input_class: String,

    #[prop_or_default]
    pub children: Children,

}

pub enum InputTextMessage {
    OnChange(String),
}

pub struct InputText;

impl Component for InputText {
    type Message = InputTextMessage;
    type Properties = InputTextProps;

     fn create(_ctx: &Context<Self>) -> Self {
        InputText {

        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: InputTextMessage,
    ) -> bool {
        match msg {
            InputTextMessage::OnChange( new_value ) => {
                // self.value += 1;
                ctx.props().onchange.emit( new_value );
                false
            }
        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let onchange = ctx.link().callback(
            |event: InputEvent| {
                let input: HtmlInputElement = event.target_unchecked_into();
                InputTextMessage::OnChange(input.value())
            }
        );

        let mut description = html!(<></>);
        if ctx.props().description.to_owned() != "" {
            description = html!(
                <div class={"small-text"}>
                { ctx.props().description.to_owned() }
                </div>
            );
        }

        let mut input_type = ctx.props().input_type.to_owned();
        if input_type.is_empty() {
            input_type = "text".to_owned();
        }

        html! {
            <label
                class={ctx.props().label_class.to_owned()}
                title={ctx.props().title.to_owned()}
            >
                <InputLabel
                    label={ctx.props().label.to_owned()}
                    inline={ctx.props().inline}
                />

                {description}

                <input
                    class={ctx.props().input_class.to_owned()}
                    placeholder={ctx.props().placeholder.to_owned()}
                    type={input_type}
                    readonly={ctx.props().readonly}
                    value={ctx.props().value.to_owned()}
                    oninput={onchange}
                />

                { for ctx.props().children.iter() }
            </label>
        }
    }
}

use yew::prelude::*;
use web_sys::{HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct InputCheckboxProps {

    #[prop_or_default]
    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub image_version: bool,

    #[prop_or_default]
    pub bigger_image: bool,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub description: String,

    #[prop_or_default]
    pub inline: bool,

    #[prop_or_default]
    pub label_class: String,

    #[prop_or_default]
    pub input_class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub image_path: String,
}

pub enum InputCheckboxMessage {
    OnChange(bool),
}

pub struct InputCheckbox;

impl Component for InputCheckbox {
    type Message = InputCheckboxMessage;
    type Properties = InputCheckboxProps;

    fn create(_ctx: &Context<Self>) -> Self {
        InputCheckbox
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: InputCheckboxMessage,
    ) -> bool {
        match msg {
            InputCheckboxMessage::OnChange( new_value ) => {
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
            |event: Event| {
                let input: HtmlInputElement = event.target_unchecked_into();
                InputCheckboxMessage::OnChange(input.checked())
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

        let mut class_variable = "cursor-pointer ".to_owned() + &ctx.props().label_class.to_owned();

        if ctx.props().image_version {
            class_variable = "checkbox-image ".to_owned() + &class_variable;
            if ctx.props().bigger_image {
                class_variable = "checkbox-image bigger-image ".to_owned() + &class_variable;
            }
        }

        html! {
            <label
                class={class_variable}
                title={ctx.props().title.to_owned()}
            >
                if ctx.props().checked {
                    <img class={"check-image"} src={ctx.props().image_path.to_owned() + "/check-yes.png"} />
                } else {
                    <img class={"check-image"} src={ctx.props().image_path.to_owned() + "/check-no.png"} />
                }
                <input
                    class={ctx.props().input_class.to_owned()}
                    type={"checkbox"}
                    checked={ctx.props().checked}
                    onchange={onchange}
                />
                {ctx.props().label.to_owned()}

                {description}
                { for ctx.props().children.iter() }
            </label>
        }
    }
}

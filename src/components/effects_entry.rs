use standard_components::ui::input_label::InputLabel;
use standard_components::ui::textarea::TextArea;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EffectsEntryProps {
    #[prop_or_default]
    pub onchange: Callback<Vec<String>>,

    #[prop_or_default]
    pub title: String,

    #[prop_or_default]
    pub value: Vec<String>,

    #[prop_or_default]
    pub description: String,

    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub r#type: String,

    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub label_class: String,

    #[prop_or_default]
    pub input_class: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub starting_height: u32,
}

pub enum EffectsEntryMessage {
    OnChange(String),
}

pub struct EffectsEntry;

impl Component for EffectsEntry {
    type Message = EffectsEntryMessage;
    type Properties = EffectsEntryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        EffectsEntry {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: EffectsEntryMessage) -> bool {
        match msg {
            EffectsEntryMessage::OnChange(new_value) => {
                let mut nv: Vec<String> = Vec::new();

                for val in new_value.as_str().split("\n") {
                    nv.push( val.to_owned() );
                }
                ctx.props().onchange.emit( nv );
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut style = "height: 100px".to_owned();

        if ctx.props().starting_height > 100 {
            style = "height: ".to_owned() + &ctx.props().starting_height.to_string() + &"px";
        }

        let onchange = ctx.link().callback(|event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            EffectsEntryMessage::OnChange(input.value())
        });

        let mut description = html!(<></>);
        if ctx.props().description.to_owned() != "" {
            description = html!(
                <div class={"small-text"}>
                { ctx.props().description.to_owned() }
                </div>
            );
        }

        html! {
            <label
                class={ctx.props().label_class.to_owned()}
                title={ctx.props().title.to_owned()}
            >
                <div class="pull-right small-text">
                    {"TODO: Modlines Check"}
                </div>
                <InputLabel
                    label={ctx.props().label.to_owned()}
                    inline={false}
                />

                {description}

                <textarea
                    class={ctx.props().input_class.to_owned()}
                    placeholder={ctx.props().placeholder.to_owned()}
                    readonly={ctx.props().readonly}
                    style={style}
                    oninput={onchange}
                    value={ctx.props().value.join("\n")}
                />
                { for ctx.props().children.iter() }
            </label>
        }
    }
}

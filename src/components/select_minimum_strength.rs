use web_sys::HtmlSelectElement;
use yew::prelude::*;
use standard_components::ui::input_label::InputLabel;

#[derive(Properties, PartialEq)]
pub struct SelectMinimumStrengthProps {
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

#[function_component(SelectMinimumStrength)]
pub fn select_minimum_strength( props: &SelectMinimumStrengthProps) -> Html {

    let onchange = props.onchange.clone();

    // let onchange = ctx.link().callback(|event: Event| {
    //     let input: HtmlSelectElement = event.target_unchecked_into();
    //     // InputTextMessage::OnChange(input.value())
    // });
    let callback_set_value = Callback::from(move |e: Event| {
        e.prevent_default();

        let input: HtmlSelectElement = e.target_unchecked_into();

        onchange.emit(input.value())
    });

    let mut description = html!(<></>);
    if props.description.to_owned() != "" {
        description = html!(
            <div class={"small-text"}>
            { props.description.to_owned() }
            </div>
        );
    }
    let mut label_class = props.label_class.to_owned();

    if props.inline {
        label_class = label_class + &" inline";
    }
    html! {
        <label
            class={label_class.to_owned()}
            title={props.title.to_owned()}
        >
            <InputLabel
                label={props.label.to_owned()}
                inline={props.inline}
            />

            {description}

            <select
                class={props.input_class.to_owned()}
                readonly={props.readonly}
               onchange={callback_set_value}
            >
                <option selected={props.value == ""} value="">{"None"}</option>
                <option selected={props.value == "d4"} value="d4">{"d4"}</option>
                <option selected={props.value == "d6"} value="d6">{"d6"}</option>
                <option selected={props.value == "d8"} value="d8">{"d8"}</option>
                <option selected={props.value == "d10"} value="d10">{"d10"}</option>
                <option selected={props.value == "d12"} value="d12">{"d12"}</option>

                <option selected={props.value == "d12+1"} value="d12+1">{"d12+1"}</option>
                <option selected={props.value == "d12+2"} value="d12+2">{"d12+2"}</option>
                <option selected={props.value == "d12+3"} value="d12+3">{"d12+3"}</option>
                <option selected={props.value == "d12+4"} value="d12+4">{"d12+4"}</option>
                <option selected={props.value == "d12+5"} value="d12+5">{"d12+5"}</option>
                <option selected={props.value == "d12+6"} value="d12+6">{"d12+6"}</option>
                <option selected={props.value == "d12+7"} value="d12+7">{"d12+7"}</option>
                <option selected={props.value == "d12+8"} value="d12+8">{"d12+8"}</option>
                <option selected={props.value == "d12+9"} value="d12+9">{"d12+9"}</option>
                <option selected={props.value == "d12+10"} value="d12+10">{"d12+10"}</option>
                <option selected={props.value == "d12+11"} value="d12+11">{"d12+11"}</option>
                <option selected={props.value == "d12+12"} value="d12+12">{"d12+12"}</option>


            </select>

            { for props.children.iter() }
        </label>
    }

}

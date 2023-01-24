use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFieldTextProps {
    #[prop_or_default]
    pub td_class: String,
    pub value: String,
}

#[function_component(AdminTableFieldText)]
pub fn admin_table_field_text(props: &AdminTableFieldTextProps) -> Html {
    html! {
        <td class={&props.td_class}>{&props.value}</td>
    }
}

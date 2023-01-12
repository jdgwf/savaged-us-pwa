use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFieldBoolProps {

    #[prop_or_default]
    pub td_class: String,

    pub value: bool,
}

#[function_component(AdminTableFieldBool)]
pub fn admin_table_field_bool(
    props: &AdminTableFieldBoolProps,

) -> Html {

    let mut td_class = "vert-middle".to_owned();
    if !props.td_class.is_empty() {
        td_class = td_class + &" " + &props.td_class;
    }
    html!{
        <td class={td_class}>
            if props.value {

                <i class="color-green fa-solid fa-circle-check" />
            } else {
                <i class="color-red fa-solid fa-circle-xmark" />
            }
        </td>
    }
}
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminTableFieldActiveProps {
    #[prop_or_default]
    pub td_class: String,

    pub active: bool,
    pub no_select: bool,

    #[prop_or_default]
    pub rowspan: u8,

    #[prop_or_default]
    pub colspan: u8,
}

#[function_component(AdminTableFieldActive)]
pub fn admin_table_field_bool(props: &AdminTableFieldActiveProps) -> Html {
    let mut td_class = "vert-middle".to_owned();
    if !props.td_class.is_empty() {
        td_class = td_class + &" " + &props.td_class;
    }
    let mut row_span = props.rowspan;
    if row_span < 1 {
        row_span = 1;
    }

    let mut col_span = props.colspan;
    if col_span < 1 {
        col_span = 1;
    }
    html! {
        <td colspan={col_span.to_string()} rowspan={row_span.to_string()} class={td_class}>
            if props.no_select {
                <i title="Active, but not selectable by users. Can be called via a modline or a legacy selected item" class="color-orange fa-solid fa-eye-slash" />
            } else {
                if props.active {
                    <i title="Active and selected" class="color-green fa-solid fa-circle-check" />
                } else {
                    <i title="Inactive and unusable" class="color-red fa-solid fa-circle-xmark" />
                }
            }

        </td>
    }
}

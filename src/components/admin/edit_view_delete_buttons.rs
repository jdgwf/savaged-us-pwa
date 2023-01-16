use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditViewDeleteButtonsProps {

    pub id: u32,

    pub name: String,

    #[prop_or_default]
    pub view_callback: Option<Callback<u32>>,

    #[prop_or_default]
    pub edit_callback: Option<Callback<u32>>,

    #[prop_or_default]
    pub delete_callback: Option<Callback<u32>>,

    #[prop_or_default]
    pub duplicate_callback: Option<Callback<u32>>,

}

#[function_component(EditViewDeleteButtons)]
pub fn edit_view_delete_buttons(
    props: &EditViewDeleteButtonsProps,
) -> Html {

    let view_callback: Option<Callback<u32>> = props.view_callback.clone();
    let edit_callback: Option<Callback<u32>> = props.edit_callback.clone();
    let delete_callback: Option<Callback<u32>> = props.delete_callback.clone();
    let duplicate_callback: Option<Callback<u32>> = props.duplicate_callback.clone();

    let id = props.id;

    if view_callback == None
        &&
        edit_callback == None
        &&
        delete_callback == None
        &&
        duplicate_callback == None
    {
        return html!{
            <div class="text-center small-text">{"No Access!"}</div>
        };
    }

    let delete_title = "Click here to delete '".to_owned() + &props.name + "'. You will be asked to verify this action.";
    let edit_title = "Click here to delete '".to_owned() + &props.name + "'.";
    let duplicate_title = "Click here to duplicate '".to_owned() + &props.name + "'. You will be asked to verify this action.";
    let view_title = "Click here to view '".to_owned() + &props.name + "'.";

    return html!{
        <td class="text-center no-wrap">
            if view_callback != None {
                <button
                    type="button"
                    class="btn btn-sm btn-info"
                    title={view_title}
                    onclick={move |e: MouseEvent| {
                        let view_callback = view_callback.clone();
                        e.prevent_default();
                        view_callback.unwrap().emit(id);
                    }}
                >
                    <i class="fa fa-eye" />
                </button>
            }

            if edit_callback != None {
                <button
                    type="button"
                    title={edit_title}
                    class="btn btn-sm btn-success"
                    onclick={move |e: MouseEvent| {
                        let edit_callback = edit_callback.clone();
                        e.prevent_default();
                        edit_callback.unwrap().emit(id);
                    }}
                >
                    <i class="fa fa-edit" />
                </button>
            }

            if duplicate_callback != None {
                <button
                    type="button"
                    class="btn btn-sm btn-warning"
                    title={duplicate_title}
                    onclick={move |e: MouseEvent| {
                        let duplicate_callback = duplicate_callback.clone();
                        e.prevent_default();
                        duplicate_callback.unwrap().emit(id);
                    }}
                >
                    <i class="fa fa-copy" />
                </button>
            }

            if delete_callback != None {
                <button
                    type="button"
                    title={delete_title}
                    class="btn btn-sm btn-danger"
                    onclick={move |e: MouseEvent| {
                        let delete_callback = delete_callback.clone();
                        e.prevent_default();
                        delete_callback.unwrap().emit(id);
                    }}
                >
                    <i class="fa fa-trash" />
                </button>
            }

        </td>
    };
}
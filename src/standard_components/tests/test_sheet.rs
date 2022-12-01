use yew::prelude::*;
use super::super::ui::input_text::InputText;
use super::super::ui::input_checkbox::InputCheckbox;
use super::super::tests::test_sheet_global_vars::PlaceholderToDo;
use super::super::ui::nbsp::Nbsp;
use gloo_console::error;
use super::super::libs::local_storage_shortcuts::set_local_storage_bool;
use super::super::libs::local_storage_shortcuts::set_local_storage_string;
use super::super::libs::set_document_title::set_document_title;
use super::super::libs::fetch_json::fetch_json;
use wasm_bindgen_futures::spawn_local;
use gloo_utils::format::JsValueSerdeExt;
use super::super::tests::test_sheet_global_vars::TestSheetGlobalVars;

#[derive(Properties, PartialEq)]
pub struct TestSheetProps {
    pub update_global_vars: Callback<TestSheetGlobalVars>,
    pub global_vars: TestSheetGlobalVars,
}

pub enum TestSheetMessage {
    UpdateTest2(String),
    UpdateCheck1(bool),
    UpdateCheck2(bool),
    UpdateCheck3(bool),
    SetToDos( Vec<PlaceholderToDo> ),
}

pub struct TestSheet {
    global_vars: TestSheetGlobalVars,
}

impl Component for TestSheet {
    type Message = TestSheetMessage;
    type Properties = TestSheetProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        set_document_title("Test Sheet".to_owned(), "Worksheet".to_owned(), true);
        let global_vars = ctx.props().global_vars.clone();
        TestSheet {
            global_vars: global_vars,
        }
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
    ) -> bool {
        self.global_vars = ctx.props().global_vars.clone();
        true
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: TestSheetMessage,
    ) -> bool {

        let mut global_vars = self.global_vars.clone();
        match msg {

            TestSheetMessage::UpdateTest2( new_value ) => {

                global_vars.test2 = new_value.to_owned();
                set_local_storage_string( "test2", new_value );

                ctx.props().update_global_vars.emit( global_vars.to_owned() );
                self.global_vars = global_vars.clone();
                return false;
            }

            TestSheetMessage::UpdateCheck1( new_value ) => {

                global_vars.check1 = new_value.to_owned();
                set_local_storage_bool( "check1", new_value );

                ctx.props().update_global_vars.emit( global_vars.to_owned() );
                self.global_vars = global_vars.clone();
                return true;
            }

            TestSheetMessage::UpdateCheck2( new_value ) => {
                global_vars.check2 = new_value.to_owned();
                set_local_storage_bool( "check2", new_value );

                ctx.props().update_global_vars.emit( global_vars.to_owned() );
                self.global_vars = global_vars.clone();
                return true;
            }

            TestSheetMessage::UpdateCheck3( new_value ) => {
                global_vars.check3 = new_value.to_owned();
                set_local_storage_bool( "check3", new_value );

                ctx.props().update_global_vars.emit( global_vars.to_owned() );
                self.global_vars = global_vars.clone();
                return true;
            }

            TestSheetMessage::SetToDos( todo_vec ) => {

                global_vars.to_dos = todo_vec.clone();

                ctx.props().update_global_vars.emit( global_vars.to_owned() );
                self.global_vars = global_vars.clone();
                return true;
            }
        }

    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = &self.global_vars;

        let update_test2 = ctx.link().callback(TestSheetMessage::UpdateTest2);
        let update_check1 = ctx.link().callback(TestSheetMessage::UpdateCheck1);
        let update_check2 = ctx.link().callback(TestSheetMessage::UpdateCheck2);
        let update_check3 = ctx.link().callback(TestSheetMessage::UpdateCheck3);
        let update_to_dos = ctx.link().callback(TestSheetMessage::SetToDos);

        let do_it = Callback::from( move | _e: MouseEvent | {
            let update_to_dos = update_to_dos.clone();
            spawn_local (
                async move {
                    let result = fetch_json( "https://jsonplaceholder.typicode.com/todos/".to_owned() ).await;

                    match result {
                        Ok( value ) => {

                            let vec_val_result = value.into_serde::<Vec<PlaceholderToDo>>();
                            match vec_val_result {
                                Ok( vec_val ) => {
                                    update_to_dos.emit( vec_val.clone() );
                                }
                                Err( _ ) => {
                                    error!("get_data_via_fetch Serde Err()" );
                                }

                            }
                        }
                        Err(_) => {
                            error!("get_data_via_fetch Err()" );
                        }
                    }
                }
            );

        });

        html! {
            <>
                <h1>{ "Test Sheet" }</h1>
                <p class={"text-center"}>{"The data here is stored on your browser's localStorage"}</p>

                <fieldset class={"fieldset"}>
                    <legend>{"Fetch API Calls"}</legend>
                    <p>{"The following makes a call to"}<Nbsp /><a href={"https://jsonplaceholder.typicode.com/to_dos/"}>{"https://jsonplaceholder.typicode.com/todos/"}</a><Nbsp />{"then displays the number of records retrieved. It should read '200' once the action button is clicked"}</p>
                    <button
                        class={"btn btn-primary"}
                        onclick={do_it}
                    >
                        {"Perform Remote API Call"}
                    </button>
                    <Nbsp />{"API Call Count:"}<Nbsp />{global_vars.to_dos.len()}
                </fieldset>

                <fieldset class={"fieldset"}>
                    <legend>{"Text Inputs"}</legend>

                    <InputText
                        label={"Test Input 2"}
                        inline={false}
                        input_type={"text"}
                        placeholder={"Just testing"}
                        value={global_vars.test2.clone()}
                        title={"This is just another input"}
                        onchange={update_test2}
                        description={"A Description For Text Input"}
                    >
                        <p>{"This is a child element!"}</p>
                    </InputText>
                </fieldset>

                <fieldset class={"fieldset"}>
                    <legend>{"Checkbox Inputs"}</legend>
                    <InputCheckbox
                        onchange={update_check1}
                        label={"Checkbox 1"}
                        checked={global_vars.check1}
                        title={"Checkbox One"}
                        description={"Description For This Checkbox"}
                    >
                        <>
                            <p>{"This is a child element!"}</p>
                            <p>{"This is a child element paragraph 2!"}</p>
                        </>
                    </InputCheckbox>

                    <InputCheckbox
                        onchange={update_check2}
                        label={"Checkbox 2"}
                        image_version={true}
                        checked={global_vars.check2}
                        title={"Checkbox 2"}
                        image_path={"../images"}
                        description={"This checkbox uses an image to display checked/unchecked. This is still selectable via tabstop and can toggle via space or mouse click on component."}
                    >
                        <p>{"This is the same component as \"Checkbox 1\" above, but with the property"}
                        <Nbsp /> <pre class={"inline"}>{"image_version"}</pre><Nbsp />
                        {" set to true"}</p>
                    </InputCheckbox>

                    <InputCheckbox
                        onchange={update_check3}
                        label={"Checkbox 3"}
                        image_version={true}
                        bigger_image={true}
                        checked={global_vars.check3}
                        title={"Checkbox 3"}
                        image_path={"../images"}
                        description={"This checkbox uses an image to display checked/unchecked. This is still selectable via tabstop and can toggle via space or mouse click on component."}
                    >
                        <p>{"This is the same component as \"Checkbox 2\" above, but with the property"}
                        <Nbsp /> <pre class={"inline"}>{"bigger_image"}</pre><Nbsp />
                        {" set to true"}</p>
                    </InputCheckbox>
                </fieldset>

            </>
        }
    }
}

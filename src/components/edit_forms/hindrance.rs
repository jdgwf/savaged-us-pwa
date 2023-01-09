use std::ops::Deref;

use savaged_libs::player_character::hindrance::Hindrance;
use savaged_libs::save_db_row::SaveDBRow;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use standard_components::ui::markdown_editor::MarkdownEditor;
use yew::prelude::*;
use web_sys::File;
use standard_components::ui::input_text::InputText;
use chrono::Utc;
use wasm_bindgen_futures::spawn_local;
use crate::components::tertiary_menu::{TertiaryMenuItem, TertiaryMenu};
use crate::libs::global_vars::GlobalVars;

#[derive(Properties, PartialEq)]
pub struct EditHindranceProps {
    pub global_vars: GlobalVars,
    pub edit_item: Hindrance,
    pub edit_save: SaveDBRow,
    pub on_changed_callback: Callback< Hindrance >,
}


pub enum EditHindranceMessage {
    ChangePage(String),

    UpdateName(String),
    UpdateSummary(String),
    UpdateDescription(String),

}

pub struct EditHindrance {
    edit_item: Hindrance,
    global_vars: GlobalVars,
    edit_save: SaveDBRow,
    local_storage_page_name: String,
}

impl Component for EditHindrance {
    type Message = EditHindranceMessage;
    type Properties = EditHindranceProps;

    fn create(
        ctx: &Context<Self>,
    ) -> Self {

        EditHindrance {
            edit_item: ctx.props().edit_item.clone(),
            global_vars: ctx.props().global_vars.clone(),
            edit_save: ctx.props().edit_save.clone(),
            local_storage_page_name: "hindrance_edit_form_page".to_owned(),
        }
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &EditHindranceProps,
    ) -> bool {
        self.global_vars = ctx.props().global_vars.clone();
        self.edit_save = ctx.props().edit_save.clone();
        // self.image_name = ctx.props().image_name.clone();
        // self.upload_url = ctx.props().upload_url.clone();

        self.edit_item = ctx.props().edit_item.clone();
        true
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: EditHindranceMessage,
    ) -> bool {
        match msg {
            EditHindranceMessage::ChangePage( new_value ) => {
                set_local_storage_string( &self.local_storage_page_name, new_value);
                true
            }
            EditHindranceMessage::UpdateName( new_value ) => {
                self.edit_item.name = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone() );
                true
            }
            EditHindranceMessage::UpdateSummary( new_value ) => {
                self.edit_item.summary = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone() );
                true
            }
            EditHindranceMessage::UpdateDescription( new_value ) => {
                self.edit_item.description = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone());
                true
            }
            _ => {
                false
            }
        }

    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let current_page = get_local_storage_string( &self.local_storage_page_name, "general".to_owned());


        let on_changed_callback = ctx.props().on_changed_callback.clone();

        let mut sub_menu_items: Vec<TertiaryMenuItem> = vec![
            TertiaryMenuItem {
                tag: "general".to_owned(),
                label: "General".to_owned(),
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "selection".to_owned(),
                label: "Selection".to_owned(),
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "effects".to_owned(),
                label: "Effects".to_owned(),
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
        ];

        if ctx.props().global_vars.current_user.has_admin_access() {
            sub_menu_items.push(
                TertiaryMenuItem {
                    tag: "admin".to_owned(),
                    label: "Admin".to_owned(),
                    class: None,
                    callback: None,
                    title: None,
                    icon_class: None,
                    separate: false,
                },
            );
        }
        let change_page_callback_form = ctx.link().callback(EditHindranceMessage::ChangePage);
        let header = html!{
            <>
                <TertiaryMenu
                    global_vars={ctx.props().global_vars.clone()}
                    menu_items={sub_menu_items}
                    menu_changed_callback={change_page_callback_form}
                    local_storage_variable={self.local_storage_page_name.to_owned()}
                />



            </>
        };
        match current_page.as_str() {


            "admin" => {
                html!{
                    <div>
                    {header}
                    <fieldset class={"fieldset"}>
                    <legend>{"Admin"}</legend>
                    {"Admin Page"}
                    </fieldset>
                    </div>
                }
            }

            "selection" => {
                html!{
                    <div>
                    {header}
                    <fieldset class={"fieldset"}>
                    <legend>{"Selection"}</legend>
                    {"Selection Page"}
                    </fieldset>
                    </div>
                }
            }


            "effects" => {
                html!{
                    <div>
                    {header}
                    <fieldset class={"fieldset"}>
                    <legend>{"Effects"}</legend>
                    {"Effects Page"}
                    </fieldset>
                    </div>
                }
            }

            // default to general
            _ => {
                set_local_storage_string( &self.local_storage_page_name, "general".to_owned());
                html! {
                    <div>
                        {header}
                        <fieldset class={"fieldset"}>
                                <legend>{"General"}</legend>
                                <div class="row full-width">
                                    <div class="col-md-6">
                                        <InputText
                                            label={"Name"}
                                            value={(self.edit_item.name).to_owned()}
                                            onchange={ ctx.link().callback( EditHindranceMessage::UpdateName) }
                                        />

                                        <InputText
                                            label={"Summary"}
                                            value={(self.edit_item.summary).to_owned()}
                                            onchange={ ctx.link().callback( EditHindranceMessage::UpdateSummary) }
                                        />
                                    </div>
                                    <div class="col-md-6">
                                        <MarkdownEditor
                                            label={"Description"}
                                            value={(self.edit_item.description).to_owned()}
                                            onchange={ ctx.link().callback( EditHindranceMessage::UpdateDescription) }
                                        />
                                    </div>
                                </div>
                            </fieldset>
                    </div>
                }
            }
        }

    }
}

use crate::components::admin::book_select::BookSelect;
use crate::components::tertiary_menu::{TertiaryMenuItem, TertiaryMenu};
use crate::libs::global_vars::GlobalVars;
use savaged_libs::book::Book;
use savaged_libs::player_character::edge::Edge;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::{get_local_storage_string, get_local_storage_bool, set_local_storage_bool};
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::input_text::InputText;
use standard_components::ui::markdown_editor::MarkdownEditor;
// use standard_components::ui::textarea::TextArea;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditEdgeProps {
    pub global_vars: GlobalVars,
    pub edit_item: Edge,

    #[prop_or_default]
    pub book_list: Vec<Book>,

    pub on_changed_callback: Callback< Edge >,

    #[prop_or_default]
    pub form_title: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub for_admin: bool,
}

pub enum EditEdgeMessage {
    ChangePage(String),
    ToggleNoPages( String ),

    UpdateName(String),

    UpdateSummary(String),
    UpdateDescription(String),

    // SetMinorOrMajorEdge(bool),
    // SetMajorEdge(bool),

    // UpdateConflicts( String ),
    // UpdateEffects( String ),

    // UpdateMinorEffects( String ),
    // UpdateSummaryMinor( String ),

    UpdateBookID( u32 ),
    UpdatePage(String),
    UpdateActive(bool),
}

pub struct EditEdge {
    edit_item: Edge,
    local_storage_page_name: String,
}

impl Component for EditEdge {
    type Message = EditEdgeMessage;
    type Properties = EditEdgeProps;

    fn create(
        ctx: &Context<Self>,
    ) -> Self {

        EditEdge {
            edit_item: ctx.props().edit_item.clone(),
            local_storage_page_name: "edge_edit_form_page".to_owned(),
        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: EditEdgeMessage,
    ) -> bool {
        match msg {
            EditEdgeMessage::ChangePage( new_value ) => {
                if new_value != "__all__".to_owned() {
                    set_local_storage_string( &self.local_storage_page_name, new_value);

                }
                return true;

            }

            EditEdgeMessage::ToggleNoPages( _new_value ) => {
                let new_value = get_local_storage_bool("edit_forms_one_page", false);
                set_local_storage_bool( "edit_forms_one_page", !new_value);
                return true;
            }

            EditEdgeMessage::UpdateName( new_value ) => {
                self.edit_item.name = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone() );
                return true;
            }
            EditEdgeMessage::UpdatePage( new_value ) => {
                self.edit_item.page = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone() );
                return true;
            }

            EditEdgeMessage::UpdateSummary( new_value ) => {
                self.edit_item.summary = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone() );
                return true;
            }

            EditEdgeMessage::UpdateDescription( new_value ) => {
                self.edit_item.description = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone());
                return true;
            }

            // EditEdgeMessage::SetMinorOrMajorEdge( new_value ) => {
            //     self.edit_item.minor_or_major = new_value.to_owned();
            //     ctx.props().on_changed_callback.emit( self.edit_item.clone());
            //     return true;
            // }

            // EditEdgeMessage::SetMajorEdge( new_value ) => {
            //     self.edit_item.major = new_value.to_owned();
            //     ctx.props().on_changed_callback.emit( self.edit_item.clone());
            //     return true;
            // }

            EditEdgeMessage::UpdateActive( new_value ) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit( self.edit_item.clone());
                return true;
            }

            // EditEdgeMessage::UpdateConflicts( new_value ) => {

            //     let mut nv: Vec<String> = Vec::new();

            //     for val in new_value.as_str().split("\n") {
            //         nv.push( val.to_owned() );
            //     }

            //     self.edit_item.conflicts = nv;
            //     ctx.props().on_changed_callback.emit( self.edit_item.clone());
            //     return true;
            // }

            // EditEdgeMessage::UpdateEffects( new_value ) => {
            //     let mut nv: Vec<String> = Vec::new();

            //     for val in new_value.as_str().split("\n") {
            //         nv.push( val.to_owned() );
            //     }

            //     self.edit_item.effects = nv;
            //     ctx.props().on_changed_callback.emit( self.edit_item.clone());
            //     return true;
            // }

            // EditEdgeMessage::UpdateMinorEffects( new_value ) => {

            //     let mut nv: Vec<String> = Vec::new();

            //     for val in new_value.as_str().split("\n") {
            //         nv.push( val.to_owned() );
            //     }

            //     self.edit_item.effects_minor = nv;
            //     ctx.props().on_changed_callback.emit( self.edit_item.clone());
            //     return true;
            // }

            // EditEdgeMessage::UpdateSummaryMinor( new_value ) => {
            //     self.edit_item.summary_minor = new_value.to_owned();
            //     ctx.props().on_changed_callback.emit( self.edit_item.clone() );
            //     return true;
            // }

            EditEdgeMessage::UpdateBookID( new_value ) => {
                self.edit_item.book_id = new_value;
                ctx.props().on_changed_callback.emit( self.edit_item.clone() );
                return true;
            }

        }

    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
         _old_props: &Self::Properties
    ) -> bool {
        self.edit_item = ctx.props().edit_item.clone();
        true
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let all = get_local_storage_bool("edit_forms_one_page", false);

        let mut current_page = get_local_storage_string( &self.local_storage_page_name, "general".to_owned());

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

        if ctx.props().global_vars.current_user.has_admin_access() && ctx.props().for_admin {
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

        let toggle_no_pages = ctx.link().callback( EditEdgeMessage::ToggleNoPages);

        if all {
            sub_menu_items = vec![
                TertiaryMenuItem {
                    tag: "__all__".to_owned(),
                    label: "Back to Paged".to_owned(),
                    class: Some("all-pages".to_owned()),
                    callback: Some( toggle_no_pages ),
                    title: None,
                    icon_class: None,
                    separate: true,
                },
            ];

        } else {
            sub_menu_items.push(
                TertiaryMenuItem {
                    tag: "__all__".to_owned(),
                    label: "No Pages".to_owned(),
                    class: Some("all-pages".to_owned()),
                    callback: Some( toggle_no_pages ),
                    title: None,
                    icon_class: None,
                    separate: true,
                },
            );
        }

        let change_page_callback_form = ctx.link().callback(EditEdgeMessage::ChangePage);
        let mut title = html!{<></>};
        match &ctx.props().form_title {
            Some( form_title ) => {
                title = html!{<h3 class="text-center no-margins">{form_title.to_owned()}</h3>};
            }
            None => {}
        }
        let header = html!{
            <>
                <TertiaryMenu
                    server_side_renderer={ctx.props().global_vars.server_side_renderer}
                    menu_items={sub_menu_items}
                    menu_changed_callback={change_page_callback_form}
                    local_storage_variable={self.local_storage_page_name.to_owned()}
                />

                {title.to_owned()}

            </>
        };

        if all {
            current_page = "__all__".to_owned();

        } else {
            current_page = get_local_storage_string( &self.local_storage_page_name, "general".to_owned());
        }
        let book_list = ctx.props().book_list.clone();

        html!{
            <div class="edit-form">
            {header}
            <div class="form-flex">
            if (current_page.as_str() == "admin" || current_page.as_str() == "__all__" ) && ctx.props().global_vars.current_user.has_admin_access() && ctx.props().for_admin {

                <fieldset class={"fieldset"}>
                    <legend>{"Admin"}</legend>

                    <InputCheckbox
                        label="Active"
                        readonly={ctx.props().readonly}
                        checked={self.edit_item.active}
                        onchange={ctx.link().callback( EditEdgeMessage::UpdateActive )}
                    />

                    <BookSelect
                        readonly={ctx.props().readonly}
                        current_user={ctx.props().global_vars.current_user.clone()}
                        book_list={book_list}
                        label={"Book"}
                        value={self.edit_item.book_id}
                        onchange={ ctx.link().callback( EditEdgeMessage::UpdateBookID) }
                    />

                    <InputText
                        readonly={ctx.props().readonly}
                        label={"Page Number"}
                        inline={true}
                        value={(self.edit_item.page).to_owned()}
                        onchange={ ctx.link().callback( EditEdgeMessage::UpdatePage) }
                    />
                </fieldset>

            }

            if current_page.as_str() == "general" || current_page.as_str() == "__all__" {
                <fieldset class={"fieldset"}>
                    <legend>{"General"}</legend>
                    <div class="row full-width">
                        <div class="col-md-6">
                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Name"}
                                value={(self.edit_item.name).to_owned()}
                                onchange={ ctx.link().callback( EditEdgeMessage::UpdateName) }
                            />

                            // <InputCheckbox
                            //     label="Major Edge"
                            //     readonly={ctx.props().readonly}
                            //     checked={self.edit_item.major}
                            //     onchange={ctx.link().callback( EditEdgeMessage::SetMajorEdge )}
                            // />
                            // <InputCheckbox
                            //     label="Minor or Major Edge"
                            //     readonly={ctx.props().readonly}
                            //     checked={self.edit_item.minor_or_major}
                            //     onchange={ctx.link().callback( EditEdgeMessage::SetMinorOrMajorEdge )}
                            // />

                            // if self.edit_item.minor_or_major {

                            //     <InputText
                            //         readonly={ctx.props().readonly}
                            //         label={"Major Summary"}
                            //         value={(self.edit_item.summary).to_owned()}
                            //         onchange={ ctx.link().callback( EditEdgeMessage::UpdateSummary) }
                            //     />
                            //     <InputText
                            //         readonly={ctx.props().readonly}
                            //         label={"Minor Summary"}
                            //         value={(self.edit_item.summary_minor).to_owned()}
                            //         onchange={ ctx.link().callback( EditEdgeMessage::UpdateSummaryMinor) }
                            //     />
                            // } else {
                            //     <InputText
                            //         readonly={ctx.props().readonly}
                            //         label={"Summary"}

                            //         value={(self.edit_item.summary).to_owned()}
                            //         onchange={ ctx.link().callback( EditEdgeMessage::UpdateSummary) }
                            //     />
                            // }

                            <InputText
                                label={"UUID"}
                                readonly={true}
                                value={(self.edit_item.uuid.to_string()).to_owned()}
                            />
                        </div>
                        <div class="col-md-6">
                            <MarkdownEditor
                                readonly={ctx.props().readonly}
                                label={"Description"}
                                starting_height={175}
                                value={(self.edit_item.description).to_owned()}
                                onchange={ ctx.link().callback( EditEdgeMessage::UpdateDescription) }
                            />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "effects" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Effects"}</legend>

                    // if self.edit_item.minor_or_major {
                    //     <div class="row full-width">
                    //         <div class="col-md-6">
                    //             <TextArea
                    //                 readonly={ctx.props().readonly}
                    //                 label={"Major Effects"}
                    //                 value={self.edit_item.effects.join("\n")}
                    //                 onchange={ ctx.link().callback( EditEdgeMessage::UpdateEffects) }
                    //             />
                    //         </div>
                    //         <div class="col-md-6">
                    //             <TextArea
                    //                 readonly={ctx.props().readonly}
                    //                 label={"Minor Effects"}
                    //                 value={self.edit_item.effects_minor.join("\n")}
                    //                 onchange={ ctx.link().callback( EditEdgeMessage::UpdateMinorEffects ) }
                    //             />
                    //         </div>
                    //     </div>
                    // } else {
                    //     <TextArea
                    //         readonly={ctx.props().readonly}
                    //         label={"Effects"}
                    //         value={self.edit_item.effects.join("\n")}
                    //         onchange={ ctx.link().callback( EditEdgeMessage::UpdateEffects) }
                    //     />
                    // }
                </fieldset>
            }

            if current_page.as_str() == "selection" || current_page.as_str() == "__all__" {
                <fieldset class={"fieldset"}>
                    <legend>{"Selection"}</legend>
                    // <TextArea
                    //     readonly={ctx.props().readonly}
                    //     label={"Conflicts"}
                    //     value={self.edit_item.conflicts.join("\n")}
                    //     onchange={ ctx.link().callback( EditEdgeMessage::UpdateConflicts) }
                    // />
                </fieldset>
            }
                </div>
            </div>
        }

    }

}

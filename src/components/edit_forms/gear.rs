use crate::components::abilities_entry::AbilitiesEntry;
use crate::components::admin::book_select::BookSelect;
use crate::components::effects_entry::EffectsEntry;
use crate::components::tertiary_menu::{TertiaryMenu, TertiaryMenuItem};
use crate::libs::global_vars::GlobalVars;
use gloo_console::error;
use savaged_libs::book::Book;
use savaged_libs::player_character::gear::Gear;
use standard_components::libs::local_storage_shortcuts::get_local_storage_bool;
use standard_components::libs::local_storage_shortcuts::get_local_storage_string;
use standard_components::libs::local_storage_shortcuts::set_local_storage_bool;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::input_label::InputLabel;
use standard_components::ui::input_number::InputNumber;
use standard_components::ui::input_text::InputText;
use standard_components::ui::markdown_editor::MarkdownEditor;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditGearProps {
    pub global_vars: GlobalVars,
    pub edit_item: Gear,

    #[prop_or_default]
    pub book_list: Vec<Book>,

    pub on_changed_callback: Callback<Gear>,

    #[prop_or_default]
    pub form_title: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub for_admin: bool,
}

pub enum EditGearMessage {
    ChangePage(String),
    ToggleNoPages(String),

    UpdateName(String),
    UpdateGearType(String),

    UpdateSummary(String),
    UpdateAbilities(Vec<String>),
    UpdateDescription(String),

    UpdateRippersReason( f32 ),
    UpdateNumberPer( f32 ),

    UpdateCost( f32 ),
    UpdateWeight( f32 ),

    UpdateEffects( Vec<String> ),

    UpdateBookID(u32),
    UpdatePage(String),
    UpdateActive(bool),

    UpdateIsContainer(bool),
    UpdateContainerNoWeight(bool),
    UpdateFractionalWeight(Event),
}

pub struct EditGear {
    edit_item: Gear,
    local_storage_page_name: String,
}

impl Component for EditGear {
    type Message = EditGearMessage;
    type Properties = EditGearProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditGear {
            edit_item: ctx.props().edit_item.clone(),
            local_storage_page_name: "gear_edit_form_page".to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: EditGearMessage) -> bool {
        match msg {
            EditGearMessage::ChangePage(new_value) => {
                if new_value != "__all__".to_owned() {
                    set_local_storage_string(&self.local_storage_page_name, new_value);
                }
                return true;
            }

            EditGearMessage::ToggleNoPages(_new_value) => {
                let new_value = get_local_storage_bool("edit_forms_one_page", false);
                set_local_storage_bool("edit_forms_one_page", !new_value);
                return true;
            }

            EditGearMessage::UpdateName(new_value) => {
                self.edit_item.name = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateGearType(new_value) => {
                self.edit_item.gear_type = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdatePage(new_value) => {
                self.edit_item.page = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateSummary(new_value) => {
                self.edit_item.summary = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateAbilities(new_value) => {
                self.edit_item.abilities = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateRippersReason( new_value ) => {
                self.edit_item.rippers_reason_cost = new_value.round() as i32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateCost( new_value ) => {
                self.edit_item.cost = new_value;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditGearMessage::UpdateWeight( new_value ) => {
                self.edit_item.weight = new_value;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateNumberPer( new_value ) => {
                self.edit_item.number_per = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateDescription(new_value) => {
                self.edit_item.description = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateActive(new_value) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateIsContainer(new_value) => {
                self.edit_item.container = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditGearMessage::UpdateContainerNoWeight(new_value) => {
                self.edit_item.container_no_weight = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditGearMessage::UpdateFractionalWeight(event) => {
                event.prevent_default();

                let input: HtmlSelectElement = event.target_unchecked_into();
                let nv_res = input.value().parse::<f32>();
                match nv_res {
                    Ok( nv ) => {
                        self.edit_item.container_fractional_weight = nv;
                    }
                    Err (err ) => {
                        error!( format!("Cannot format f32! '{}' - {}", input.value(), err) );
                    }
                }

                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateEffects( new_value ) => {

                self.edit_item.effects = new_value.clone();
                ctx.props().on_changed_callback.emit( self.edit_item.clone());
                return true;
            }

            EditGearMessage::UpdateBookID(new_value) => {
                self.edit_item.book_id = new_value;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.edit_item = ctx.props().edit_item.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let all = get_local_storage_bool("edit_forms_one_page", false);

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
                tag: "details".to_owned(),
                label: "Details".to_owned(),
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
            sub_menu_items.push(TertiaryMenuItem {
                tag: "admin".to_owned(),
                label: "Admin".to_owned(),
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            });
        }

        let toggle_no_pages = ctx.link().callback(EditGearMessage::ToggleNoPages);

        if all {
            sub_menu_items = vec![TertiaryMenuItem {
                tag: "__all__".to_owned(),
                label: "Back to Paged".to_owned(),
                class: Some("all-pages".to_owned()),
                callback: Some(toggle_no_pages),
                title: None,
                icon_class: None,
                separate: true,
            }];
        } else {
            sub_menu_items.push(TertiaryMenuItem {
                tag: "__all__".to_owned(),
                label: "No Pages".to_owned(),
                class: Some("all-pages".to_owned()),
                callback: Some(toggle_no_pages),
                title: None,
                icon_class: None,
                separate: true,
            });
        }

        let change_page_callback_form = ctx.link().callback(EditGearMessage::ChangePage);
        let mut title = html! {<></>};
        match &ctx.props().form_title {
            Some(form_title) => {
                title = html! {<h3 class="text-center no-margins">{form_title.to_owned()}</h3>};
            }
            None => {}
        }
        let mut header = html! {
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

        let mut current_page =
            get_local_storage_string(&self.local_storage_page_name, "general".to_owned());

        let valid_pages = vec!["general", "admin", "effects", "details"];
        if (current_page.as_str() == "admin"
            && !ctx.props().global_vars.current_user.has_admin_access())
            || !valid_pages.contains(&current_page.as_str())
        {
            current_page = "general".to_owned();
        }

        if all {
            current_page = "__all__".to_owned();
        }

        let book_list = ctx.props().book_list.clone();

        html! {
            <div class="edit-form">
            {header}
            <div class="form-flex">
            if (current_page.as_str() == "admin" || current_page.as_str() == "__all__" ) && ctx.props().global_vars.current_user.has_admin_access() && ctx.props().for_admin {

                <fieldset class={"fieldset"}>
                    <legend>{"Admin"}</legend>

                    <div class="row">
                        <div class="col-md-4">
                            <InputCheckbox
                                label="Active"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.active}
                                onchange={ctx.link().callback( EditGearMessage::UpdateActive )}
                            />
                        </div>
                        <div class="col-md-4">
                            <BookSelect
                                readonly={ctx.props().readonly}
                                current_user={ctx.props().global_vars.current_user.clone()}
                                book_list={book_list}
                                label={"Book"}
                                value={self.edit_item.book_id}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateBookID) }
                            />
                        </div>
                        <div class="col-md-4">
                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Page Number"}
                                inline={true}
                                value={(self.edit_item.page).to_owned()}
                                onchange={ ctx.link().callback( EditGearMessage::UpdatePage) }
                            />
                        </div>
                    </div>
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
                                onchange={ ctx.link().callback( EditGearMessage::UpdateName) }
                            />

                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Type"}
                                value={(self.edit_item.gear_type).to_owned()}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateGearType) }
                            />

                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Summary"}
                                value={(self.edit_item.summary).to_owned()}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateSummary) }
                            />

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
                                onchange={ ctx.link().callback( EditGearMessage::UpdateDescription) }
                            />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "effects" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Effects"}</legend>
                    <div class="row full-width">
                        <div class="col-md-6">

                            <EffectsEntry
                                readonly={ctx.props().readonly}
                                description="These effects will apply when this item is equipped"
                                label={"Effects"}
                                value={self.edit_item.effects.clone()}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateEffects) }
                            />

                            <InputNumber
                                readonly={ctx.props().readonly}
                                label={"Rippers Reason Cost"}
                                step={"1"}
                                min={"-5"}
                                max={"0"}
                                description={"Just put in the reason cost here (-1, -2, -3, etc) to reduce your character's reason due to RipperTech"}
                                value={self.edit_item.rippers_reason_cost as f32}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateRippersReason) }
                            />
                        </div>
                        <div class="col-md-6">

                            <AbilitiesEntry
                                readonly={ctx.props().readonly}
                                description="These abilities will be added to the summary when this item is equipped"
                                label={"Abilities"}
                                value={self.edit_item.abilities.clone()}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateEffects) }
                            />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "details" || current_page.as_str() == "__all__" {
                <fieldset class={"fieldset"}>
                    <legend>{"Details"}</legend>
                    <div class="row full-width">
                        <div class="col-md-6">

                            <InputNumber
                                readonly={ctx.props().readonly}
                                label={"Cost"}
                                step={".01"}
                                value={self.edit_item.cost }
                                onchange={ ctx.link().callback( EditGearMessage::UpdateCost) }
                            />
                            <InputNumber
                                readonly={ctx.props().readonly}
                                label={"Weight"}
                                step={".01"}
                                value={self.edit_item.weight}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateWeight) }
                            />
                            <InputNumber
                                readonly={ctx.props().readonly}
                                label={"Number per cost/weight"}
                                step={"1"}
                                min={"1"}
                                max={"500"}
                                value={self.edit_item.number_per as f32}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateNumberPer) }
                            />
                        </div>
                        <div class="col-md-6">

                        <InputCheckbox
                            label="Is a Container"
                            checked={self.edit_item.container}
                            onchange={ ctx.link().callback( EditGearMessage::UpdateIsContainer) }
                        />

                        if self.edit_item.container {

                            <InputCheckbox
                                label="Container doesn't contribute to weight (a pack animal, or a bag of holding)"
                                checked={self.edit_item.container_no_weight}
                                onchange={ ctx.link().callback( EditGearMessage::UpdateContainerNoWeight) }
                            />

                            if !self.edit_item.container_no_weight {
                                <label
                                >
                                    <InputLabel
                                        label={"Fractional weight"}
                                        inline={false}
                                    />

                                    <div class="small-text">
                                    {"Item contents weights are multiplied by the value below if stored in this container."}
                                    </div>

                                    <select
                                        readonly={ctx.props().readonly}
                                        value={self.edit_item.container_fractional_weight.to_string()}
                                        onchange={ctx.link().callback( EditGearMessage::UpdateFractionalWeight )}
                                    >
                                        <option value=".25" selected={self.edit_item.container_fractional_weight == 0.25}>{"¼ Total Weight"}</option>
                                        <option value=".5" selected={self.edit_item.container_fractional_weight == 0.5}>{"½ Total Weight"}</option>
                                        <option value="1" selected={self.edit_item.container_fractional_weight == 1.0}>{"Normal Weight (no reduction)"}</option>

                                    </select>

                                </label>
                            }
                        }

                        </div>
                    </div>

                    // <TextArea
                    //     readonly={ctx.props().readonly}
                    //     label={"Conflicts"}
                    //     value={self.edit_item.conflicts.join("\n")}
                    //     onchange={ ctx.link().callback( EditGearMessage::UpdateConflicts) }
                    // />
                </fieldset>
            }
                </div>
            </div>
        }
    }
}

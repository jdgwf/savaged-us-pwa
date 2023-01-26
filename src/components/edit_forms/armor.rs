use crate::components::abilities_entry::AbilitiesEntry;
use crate::components::admin::book_select::BookSelect;
use crate::components::effects_entry::EffectsEntry;
use crate::components::select_minimum_strength::SelectMinimumStrength;
use crate::components::tertiary_menu::{TertiaryMenu, TertiaryMenuItem};
use crate::libs::global_vars::GlobalVars;
use savaged_libs::book::Book;
use savaged_libs::player_character::armor::Armor;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::{
    get_local_storage_bool, get_local_storage_string, set_local_storage_bool,
};
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::input_number::InputNumber;
use standard_components::ui::input_text::InputText;
use standard_components::ui::markdown_editor::MarkdownEditor;
use standard_components::ui::textarea::TextArea;
// use standard_components::ui::textarea::TextArea;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditArmorProps {
    pub global_vars: GlobalVars,
    pub edit_item: Armor,

    #[prop_or_default]
    pub book_list: Vec<Book>,

    pub on_changed_callback: Callback<Armor>,

    #[prop_or_default]
    pub form_title: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub for_admin: bool,

}

pub enum EditArmorMessage {
    ChangePage(String),
    ToggleNoPages(String),

    UpdateName(String),

    UpdateSummary(String),
    UpdateDescription(String),
    UpdateMinimumStrength( String ),

    UpdateEffects( Vec<String> ),

    UpdateBookID(u32),
    UpdatePage(String),
    UpdateActive(bool),

    UpdateCost( f32 ),
    UpdateWeight( f32 ),
    UpdateSize( f32 ),

    UpdateSecondaryArmor( f32 ),
    UpdateArmor( f32 ),
    UpdateToughness( f32 ),


    UpdateAbilities(Vec<String>),

    UpdateCoversHead(bool),
    UpdateCoversFace(bool),
    UpdateCoversTorso(bool),
    UpdateCoversArms(bool),
    UpdateCoversLegs(bool),

    UpdateIsShield( bool ),
    UpdateIsEnergyScreen( bool ),

    UpdateArmorStacks( bool ),
    UpdateHeavyArmor( bool ),

    UpdateRequiresTwoHands( bool ),
    UpdateParry( f32 ),
    UpdateHardness( f32 ),
    UpdateCoverVsRanged( f32 ),
}

pub struct EditArmor {
    edit_item: Armor,
    local_storage_page_name: String,
}

impl Component for EditArmor {
    type Message = EditArmorMessage;
    type Properties = EditArmorProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditArmor {
            edit_item: ctx.props().edit_item.clone(),
            local_storage_page_name: "armor_edit_form_page".to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: EditArmorMessage) -> bool {
        match msg {
            EditArmorMessage::ChangePage(new_value) => {
                if new_value != "__all__".to_owned() {
                    set_local_storage_string(&self.local_storage_page_name, new_value);
                }
                return true;
            }

            EditArmorMessage::ToggleNoPages(_new_value) => {
                let new_value = get_local_storage_bool("edit_forms_one_page", false);
                set_local_storage_bool("edit_forms_one_page", !new_value);
                return true;
            }

            EditArmorMessage::UpdateName(new_value) => {
                self.edit_item.name = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdatePage(new_value) => {
                self.edit_item.page = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateSummary(new_value) => {
                self.edit_item.summary = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateMinimumStrength(new_value) => {
                self.edit_item.minimum_strength = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateDescription(new_value) => {
                self.edit_item.description = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateActive(new_value) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateCoversHead(new_value) => {
                self.edit_item.covers_head = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateCoversFace(new_value) => {
                self.edit_item.covers_face = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateCoversTorso(new_value) => {
                self.edit_item.covers_torso = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateCost( new_value ) => {
                self.edit_item.cost = new_value;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateWeight( new_value ) => {
                self.edit_item.weight = new_value;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateSize( new_value ) => {
                self.edit_item.size = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateSecondaryArmor( new_value ) => {
                self.edit_item.secondary_armor_value = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateArmor( new_value ) => {
                self.edit_item.armor_value = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateParry( new_value ) => {
                self.edit_item.shield_parry_bonus = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateHardness( new_value ) => {
                self.edit_item.hardness = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateCoverVsRanged( new_value ) => {
                self.edit_item.shield_cover_vs_ranged = new_value.round() as i32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateToughness( new_value ) => {
                self.edit_item.toughness = new_value.round() as u32;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateRequiresTwoHands(new_value) => {
                self.edit_item.requires_2_hands = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateCoversArms(new_value) => {
                self.edit_item.covers_arms = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateCoversLegs(new_value) => {
                self.edit_item.covers_legs = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateIsShield(new_value) => {
                self.edit_item.is_shield = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateIsEnergyScreen(new_value) => {
                self.edit_item.is_energy_screen = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateArmorStacks(new_value) => {
                self.edit_item.stacks_with_other_armor = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateHeavyArmor(new_value) => {
                self.edit_item.heavy = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateEffects( new_value ) => {

                self.edit_item.effects = new_value.clone();
                ctx.props().on_changed_callback.emit( self.edit_item.clone());
                return true;
            }

            EditArmorMessage::UpdateBookID(new_value) => {
                self.edit_item.book_id = new_value;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditArmorMessage::UpdateAbilities(new_value) => {
                self.edit_item.abilities = new_value.to_owned();
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
                tag: "protection".to_owned(),
                label: "Protection".to_owned(),
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
            TertiaryMenuItem {
                tag: "integrated_weapons".to_owned(),
                label: "Integrated Weapons".to_owned(),
                class: None,
                callback: None,
                title: None,
                icon_class: None,
                separate: false,
            },
            TertiaryMenuItem {
                tag: "alternate_modes".to_owned(),
                label: "Alternate Modes".to_owned(),
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

        let toggle_no_pages = ctx.link().callback(EditArmorMessage::ToggleNoPages);

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

        let change_page_callback_form = ctx.link().callback(EditArmorMessage::ChangePage);
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

        let valid_pages = vec!["general", "admin", "effects", "protection", "details", "integrated_weapons", "alternate_modes"];
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
                                onchange={ctx.link().callback( EditArmorMessage::UpdateActive )}
                            />
                        </div>
                        <div class="col-md-4">
                            <BookSelect
                                readonly={ctx.props().readonly}
                                current_user={ctx.props().global_vars.current_user.clone()}
                                book_list={book_list}
                                label={"Book"}
                                value={self.edit_item.book_id}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateBookID) }
                            />
                        </div>
                        <div class="col-md-4">
                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Page Number"}
                                inline={true}
                                value={(self.edit_item.page).to_owned()}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdatePage) }
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
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateName) }
                            />

                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Name"}
                                value={(self.edit_item.summary).to_owned()}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateSummary) }
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
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateDescription) }
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
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateCost) }
                            />
                            <InputNumber
                                readonly={ctx.props().readonly}
                                label={"Weight"}
                                step={".01"}
                                value={self.edit_item.weight}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateWeight) }
                            />

                        </div>
                        <div class="col-md-6">

                            <SelectMinimumStrength
                                label={"Minimum Strength"}
                                readonly={ctx.props().readonly}
                                value={(self.edit_item.minimum_strength.to_string()).to_owned()}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateMinimumStrength) }
                            />

                            <InputCheckbox
                                label="Is a Shield"
                                description="This will modify the available Protection parameters below"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.is_shield}
                                onchange={ctx.link().callback( EditArmorMessage::UpdateIsShield )}
                            />

                            <InputNumber
                                readonly={ctx.props().readonly}
                                label={"Size"}
                                step={"1"}
                                value={self.edit_item.size as f32}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateSize) }
                            />


                        </div>
                    </div>

                    // <TextArea
                    //     readonly={ctx.props().readonly}
                    //     label={"Conflicts"}
                    //     value={self.edit_item.conflicts.join("\n")}
                    //     onchange={ ctx.link().callback( EditArmorMessage::UpdateConflicts) }
                    // />
                </fieldset>
            }


            if current_page.as_str() == "protection" || current_page.as_str() == "__all__" {

                if self.edit_item.is_shield {
                    <fieldset class={"fieldset"}>
                        <legend>{"Shield Protection"}</legend>
                        <div class="row full-width">
                            <div class="col-md-4">
                                <InputNumber
                                    readonly={ctx.props().readonly}
                                    label={"Parry Bonus"}
                                    step={"1"}
                                    value={self.edit_item.shield_parry_bonus as f32}
                                    onchange={ ctx.link().callback( EditArmorMessage::UpdateParry) }
                                />

                                <InputNumber
                                    readonly={ctx.props().readonly}
                                    label={"Cover vs Ranged"}
                                    description={"This will be a negative number if anything"}
                                    step={"1"}
                                    min={"-5"}
                                    max={"0"}
                                    value={self.edit_item.shield_cover_vs_ranged as f32}
                                    onchange={ ctx.link().callback( EditArmorMessage::UpdateCoverVsRanged) }
                                />

                                <InputNumber
                                    readonly={ctx.props().readonly}
                                    label={"Hardness"}
                                    step={"1"}
                                    value={self.edit_item.hardness as f32}
                                    onchange={ ctx.link().callback( EditArmorMessage::UpdateHardness) }
                                />

                                <InputCheckbox
                                    label="Requires Two Hands"

                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.requires_2_hands}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateRequiresTwoHands )}
                                />


                            </div>
                        </div>
                    </fieldset>
                } else {
                    <fieldset class={"fieldset"}>
                        <legend>{"Armor Protection"}</legend>
                        <div class="row full-width">
                            <div class="col-md-4">
                                <InputNumber
                                    readonly={ctx.props().readonly}
                                    label={"Armor Value"}
                                    step={"1"}
                                    value={self.edit_item.armor_value as f32}
                                    onchange={ ctx.link().callback( EditArmorMessage::UpdateArmor) }
                                />
                                <InputNumber
                                    readonly={ctx.props().readonly}
                                    label={"Secondary Armor Value"}
                                    step={"1"}
                                    value={self.edit_item.secondary_armor_value as f32}
                                    onchange={ ctx.link().callback( EditArmorMessage::UpdateSecondaryArmor) }
                                />
                                <InputNumber
                                    readonly={ctx.props().readonly}
                                    label={"Toughness Bonus"}
                                    step={"1"}
                                    value={self.edit_item.toughness as f32}
                                    onchange={ ctx.link().callback( EditArmorMessage::UpdateToughness) }
                                />


                            </div>
                            <div class="col-md-4">
                                <h4>{"Locations"}</h4>
                                <InputCheckbox
                                    label="Head"
                                    label_class="no-margins"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.covers_head}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateCoversHead )}
                                />
                                <InputCheckbox
                                    label="Face"
                                    label_class="no-margins"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.covers_face}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateCoversFace )}
                                />
                                <InputCheckbox
                                    label="Torso"
                                    label_class="no-margins"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.covers_torso}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateCoversTorso )}
                                />
                                <InputCheckbox
                                    label="Arms"
                                    label_class="no-margins"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.covers_arms}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateCoversArms )}
                                />
                                <InputCheckbox
                                    label="Legs"
                                    label_class="no-margins"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.covers_legs}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateCoversLegs )}
                                />

                            </div>
                            <div class="col-md-4">
                                <InputCheckbox
                                    label="Is an Energy Screen"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.is_shield}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateIsEnergyScreen )}
                                />

                                <InputCheckbox
                                    label="Heavy Armor"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.heavy}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateHeavyArmor )}
                                />

                                <InputCheckbox
                                    label="Stacks fully with other armor"
                                    readonly={ctx.props().readonly}
                                    checked={self.edit_item.stacks_with_other_armor}
                                    onchange={ctx.link().callback( EditArmorMessage::UpdateArmorStacks )}
                                />




                            </div>
                        </div>
                    </fieldset>
                }
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
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateEffects) }
                            />

                        </div>
                        <div class="col-md-6">

                            <AbilitiesEntry
                                readonly={ctx.props().readonly}
                                description="These abilities will be added to the summary when this item is equipped"
                                label={"Abilities"}
                                value={self.edit_item.abilities.clone()}
                                onchange={ ctx.link().callback( EditArmorMessage::UpdateAbilities) }
                            />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "integrated_weapons" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Integrated Weapons"}</legend>

                    <div class="row full-width">
                        <div class="col-md-6">

                            // <EffectsEntry
                            //     readonly={ctx.props().readonly}
                            //     description="These effects will apply when this item is equipped"
                            //     label={"Effects"}
                            //     value={self.edit_item.effects.clone()}
                            //     onchange={ ctx.link().callback( EditArmorMessage::UpdateEffects) }
                            // />

                        </div>
                        <div class="col-md-6">

                            // <AbilitiesEntry
                            //     readonly={ctx.props().readonly}
                            //     description="These abilities will be added to the summary when this item is equipped"
                            //     label={"Abilities"}
                            //     value={self.edit_item.abilities.clone()}
                            //     onchange={ ctx.link().callback( EditArmorMessage::UpdateAbilities) }
                            // />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "alternate_modes" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Alternate Modes"}</legend>

                    <div class="row full-width">
                        <div class="col-md-6">

                            // <EffectsEntry
                            //     readonly={ctx.props().readonly}
                            //     description="These effects will apply when this item is equipped"
                            //     label={"Effects"}
                            //     value={self.edit_item.effects.clone()}
                            //     onchange={ ctx.link().callback( EditArmorMessage::UpdateEffects) }
                            // />

                        </div>
                        <div class="col-md-6">

                            // <AbilitiesEntry
                            //     readonly={ctx.props().readonly}
                            //     description="These abilities will be added to the summary when this item is equipped"
                            //     label={"Abilities"}
                            //     value={self.edit_item.abilities.clone()}
                            //     onchange={ ctx.link().callback( EditArmorMessage::UpdateAbilities) }
                            // />
                        </div>
                    </div>
                </fieldset>
            }
                </div>
            </div>
        }
    }
}

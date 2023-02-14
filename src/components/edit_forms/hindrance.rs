use crate::components::admin::book_select::BookSelect;
use crate::components::conflicts_entry::ConflictsEntry;
use crate::components::effects_entry::EffectsEntry;
use crate::components::tertiary_menu::{TertiaryMenu, TertiaryMenuItem};
use crate::libs::site_vars::SiteVars;
use savaged_libs::book::Book;
use savaged_libs::player_character::hindrance::Hindrance;
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::{
    get_local_storage_bool, get_local_storage_string, set_local_storage_bool,
};
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::input_text::InputText;
use standard_components::ui::markdown_editor::MarkdownEditor;
use standard_components::ui::textarea::TextArea;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditHindranceProps {
    pub site_vars: SiteVars,
    pub edit_item: Hindrance,

    #[prop_or_default]
    pub book_list: Vec<Book>,

    pub on_changed_callback: Callback<Hindrance>,

    #[prop_or_default]
    pub form_title: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub for_admin: bool,
}

pub enum EditHindranceMessage {
    ChangePage(String),
    ToggleNoPages(String),

    UpdateName(String),

    UpdateSummary(String),
    UpdateDescription(String),

    SetMinorOrMajorHindrance(bool),
    SetMajorHindrance(bool),

    UpdateConflicts(Vec<String>),
    UpdateEffects(Vec<String>),

    UpdateMinorEffects(Vec<String>),
    UpdateSummaryMinor(String),

    UpdateBookID(u32),
    UpdatePage(String),
    UpdateActive(bool),
    UpdateNoSelect(bool),

    UpdateHiddenOnCharacterSheet(bool),
    UpdateNeedsToSpecify(bool),
    UpdateCanBeTakenMoreThanOnce(bool),
    UpdateCannotBeSelected(bool),
    UpdateAlwaysShowLongName(bool),

    UpdateCountsAsOther( String ),
}

pub struct EditHindrance {
    edit_item: Hindrance,
    local_storage_page_name: String,
}

impl Component for EditHindrance {
    type Message = EditHindranceMessage;
    type Properties = EditHindranceProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditHindrance {
            edit_item: ctx.props().edit_item.clone(),
            local_storage_page_name: "hindrance_edit_form_page".to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: EditHindranceMessage) -> bool {
        match msg {
            EditHindranceMessage::ChangePage(new_value) => {
                if new_value != "__all__".to_owned() {
                    set_local_storage_string(&self.local_storage_page_name, new_value);
                }
                return true;
            }

            EditHindranceMessage::ToggleNoPages(_new_value) => {
                let new_value = get_local_storage_bool("edit_forms_one_page", false);
                set_local_storage_bool("edit_forms_one_page", !new_value);
                return true;
            }

            EditHindranceMessage::UpdateName(new_value) => {
                self.edit_item.name = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditHindranceMessage::UpdatePage(new_value) => {
                self.edit_item.page = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateSummary(new_value) => {
                self.edit_item.summary = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateDescription(new_value) => {
                self.edit_item.description = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::SetMinorOrMajorHindrance(new_value) => {
                self.edit_item.minor_or_major = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::SetMajorHindrance(new_value) => {
                self.edit_item.major = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateActive(new_value) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditHindranceMessage::UpdateNoSelect(new_value) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditHindranceMessage::UpdateHiddenOnCharacterSheet(new_value) => {
                self.edit_item.hidden_on_character_sheet = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditHindranceMessage::UpdateNeedsToSpecify(new_value) => {
                self.edit_item.needs_to_specify = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditHindranceMessage::UpdateCanBeTakenMoreThanOnce(new_value) => {
                self.edit_item.can_be_taken_more_than_once = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateCannotBeSelected(new_value) => {
                self.edit_item.cannot_be_selected = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateAlwaysShowLongName(new_value) => {
                self.edit_item.always_show_long_name = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateCountsAsOther(new_value) => {
                let mut nv: Vec<String> = Vec::new();

                for val in new_value.as_str().split("\n") {
                    nv.push( val.to_owned() );
                }

                self.edit_item.counts_as_other = nv;
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateConflicts(new_value) => {

                self.edit_item.conflicts = new_value.clone();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateEffects(new_value) => {

                self.edit_item.effects = new_value.clone();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateMinorEffects(new_value) => {

                self.edit_item.effects_minor = new_value.clone();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateSummaryMinor(new_value) => {
                self.edit_item.summary_minor = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditHindranceMessage::UpdateBookID(new_value) => {
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

        if ctx.props().site_vars.current_user.has_admin_access() && ctx.props().for_admin {
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

        let toggle_no_pages = ctx.link().callback(EditHindranceMessage::ToggleNoPages);

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

        let change_page_callback_form = ctx.link().callback(EditHindranceMessage::ChangePage);
        let mut title = html! {<></>};
        match &ctx.props().form_title {
            Some(form_title) => {
                title = html! {<h3 class="text-center no-margins">{form_title.to_owned()}</h3>};
            }
            None => {}
        }
        let header = html! {
            <>
                <TertiaryMenu
                    server_side_renderer={ctx.props().site_vars.server_side_renderer}
                    menu_items={sub_menu_items}
                    menu_changed_callback={change_page_callback_form}
                    local_storage_variable={self.local_storage_page_name.to_owned()}
                />

                {title.to_owned()}

            </>
        };

        let mut current_page =
            get_local_storage_string(&self.local_storage_page_name, "general".to_owned());

        let valid_pages = vec!["general", "admin", "effects", "selection"];
        if (current_page.as_str() == "admin"
            && !ctx.props().site_vars.current_user.has_admin_access())
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
            if (current_page.as_str() == "admin" || current_page.as_str() == "__all__" ) && ctx.props().site_vars.current_user.has_admin_access() && ctx.props().for_admin {

                <fieldset class={"fieldset"}>
                    <legend>{"Admin"}</legend>

                    <div class="row">
                        <div class="col-md-6">
                            <InputCheckbox
                                label="Active"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.active}
                                onchange={ctx.link().callback( EditHindranceMessage::UpdateActive )}
                            />
                            <InputCheckbox
                                label="No Select"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.no_select}
                                onchange={ctx.link().callback( EditHindranceMessage::UpdateNoSelect )}
                            />
                        </div>
                        <div class="col-md-6">
                            <BookSelect
                                readonly={ctx.props().readonly}
                                current_user={ctx.props().site_vars.current_user.clone()}
                                book_list={book_list}
                                label={"Book"}
                                value={self.edit_item.book_id}
                                onchange={ ctx.link().callback( EditHindranceMessage::UpdateBookID) }
                            />
                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Page Number"}
                                inline={true}
                                value={(self.edit_item.page).to_owned()}
                                onchange={ ctx.link().callback( EditHindranceMessage::UpdatePage) }
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
                                onchange={ ctx.link().callback( EditHindranceMessage::UpdateName) }
                            />

                            <InputCheckbox
                                label="Major Hindrance"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.major}
                                onchange={ctx.link().callback( EditHindranceMessage::SetMajorHindrance )}
                            />
                            <InputCheckbox
                                label="Minor or Major Hindrance"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.minor_or_major}
                                onchange={ctx.link().callback( EditHindranceMessage::SetMinorOrMajorHindrance )}
                            />

                            if self.edit_item.minor_or_major {

                                <InputText
                                    readonly={ctx.props().readonly}
                                    label={"Major Summary"}
                                    value={(self.edit_item.summary).to_owned()}
                                    onchange={ ctx.link().callback( EditHindranceMessage::UpdateSummary) }
                                />
                                <InputText
                                    readonly={ctx.props().readonly}
                                    label={"Minor Summary"}
                                    value={(self.edit_item.summary_minor).to_owned()}
                                    onchange={ ctx.link().callback( EditHindranceMessage::UpdateSummaryMinor) }
                                />
                            } else {
                                <InputText
                                    readonly={ctx.props().readonly}
                                    label={"Summary"}

                                    value={(self.edit_item.summary).to_owned()}
                                    onchange={ ctx.link().callback( EditHindranceMessage::UpdateSummary) }
                                />
                            }

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
                                onchange={ ctx.link().callback( EditHindranceMessage::UpdateDescription) }
                            />

                            <TextArea
                                readonly={ctx.props().readonly}
                                label={"Counts As Other Hindrance"}
                                value={self.edit_item.counts_as_other.join("\n")}
                                onchange={ ctx.link().callback( EditHindranceMessage::UpdateCountsAsOther) }
                            />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "effects" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Effects"}</legend>

                    if self.edit_item.minor_or_major {
                        <div class="row full-width">
                            <div class="col-md-6">
                                <EffectsEntry
                                    readonly={ctx.props().readonly}
                                    label={"Major Effects"}
                                    value={self.edit_item.effects.clone()}
                                    onchange={ ctx.link().callback( EditHindranceMessage::UpdateEffects) }
                                />
                            </div>
                            <div class="col-md-6">
                                <EffectsEntry
                                    readonly={ctx.props().readonly}
                                    label={"Minor Effects"}
                                    value={self.edit_item.effects_minor.clone()}
                                    onchange={ ctx.link().callback( EditHindranceMessage::UpdateMinorEffects ) }
                                />
                            </div>
                        </div>
                    } else {

                        <div class="row full-width">
                            <div class="col-md-6">
                                <EffectsEntry
                                    readonly={ctx.props().readonly}
                                    label={"Effects"}
                                    value={self.edit_item.effects.clone()}
                                    onchange={ ctx.link().callback( EditHindranceMessage::UpdateEffects) }
                                />
                            </div>
                        </div>
                    }
                    <InputCheckbox
                        label="Always show the long name (the name with (major) or (minor) appended"
                        readonly={ctx.props().readonly}
                        checked={self.edit_item.always_show_long_name}
                        onchange={ctx.link().callback( EditHindranceMessage::UpdateAlwaysShowLongName )}
                    />
                </fieldset>
            }

            if current_page.as_str() == "selection" || current_page.as_str() == "__all__" {
                <fieldset class={"fieldset"}>
                    <legend>{"Selection"}</legend>
                    <div class="row full-width">
                        <div class="col-md-6">
                            <ConflictsEntry
                                readonly={ctx.props().readonly}
                                label={"Conflicts"}
                                value={self.edit_item.conflicts.clone()}
                                onchange={ ctx.link().callback( EditHindranceMessage::UpdateConflicts) }
                            />
                        </div>
                        <div class="col-md-6">

                            <InputCheckbox
                                label="Hidden on Character Sheet"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.hidden_on_character_sheet}
                                onchange={ctx.link().callback( EditHindranceMessage::UpdateHiddenOnCharacterSheet )}
                            />

                            <InputCheckbox
                                label="Needs to Specify"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.needs_to_specify}
                                onchange={ctx.link().callback( EditHindranceMessage::UpdateNeedsToSpecify )}
                            />

                            <InputCheckbox
                                label="Can be taken more than once"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.can_be_taken_more_than_once}
                                onchange={ctx.link().callback( EditHindranceMessage::UpdateCanBeTakenMoreThanOnce )}
                            />

                            <InputCheckbox
                                label="Cannot be selected."
                                description="Must be added through an add_hindrance modline."
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.cannot_be_selected}
                                onchange={ctx.link().callback( EditHindranceMessage::UpdateCannotBeSelected )}
                            />

                        </div>
                    </div>
                </fieldset>
            }
                </div>
            </div>
        }
    }
}

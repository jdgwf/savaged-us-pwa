use std::ops::Index;

use crate::components::admin::book_select::BookSelect;
use crate::components::effects_entry::EffectsEntry;
use crate::components::tertiary_menu::{TertiaryMenu, TertiaryMenuItem};
use crate::libs::global_vars::GlobalVars;
use savaged_libs::book::Book;
use savaged_libs::player_character::weapon::{Weapon, WeaponProfile};
use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::libs::local_storage_shortcuts::{
    get_local_storage_bool, get_local_storage_string, set_local_storage_bool,
};
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::input_text::InputText;
use standard_components::ui::markdown_editor::MarkdownEditor;
use standard_components::ui::textarea::TextArea;
// use standard_components::ui::textarea::TextArea;
use yew::prelude::*;

use super::weapon_profile::EditWeaponProfile;


#[derive(Properties, PartialEq)]
pub struct EditWeaponProps {
    pub global_vars: GlobalVars,
    pub edit_item: Weapon,

    #[prop_or_default]
    pub book_list: Vec<Book>,

    pub on_changed_callback: Callback<Weapon>,

    #[prop_or_default]
    pub form_title: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub for_admin: bool,
}

pub enum EditWeaponMessage {
    ChangePage(String),
    ToggleNoPages(String),

    UpdateName(String),

    UpdateSummary(String),
    UpdateDescription(String),

    UpdateEffects( Vec<String> ),

    UpdateBookID(u32),
    UpdatePage(String),
    UpdateActive(bool),
    UpdateNoSelect(bool),

    UpdateWeaponProfiles( Vec<WeaponProfile> ),
}

pub struct EditWeapon {
    edit_item: Weapon,
    local_storage_page_name: String,
}

impl Component for EditWeapon {
    type Message = EditWeaponMessage;
    type Properties = EditWeaponProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditWeapon {
            edit_item: ctx.props().edit_item.clone(),
            local_storage_page_name: "weapon_edit_form_page".to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: EditWeaponMessage) -> bool {
        match msg {
            EditWeaponMessage::ChangePage(new_value) => {
                if new_value != "__all__".to_owned() {
                    set_local_storage_string(&self.local_storage_page_name, new_value);
                }
                return true;
            }

            EditWeaponMessage::UpdateWeaponProfiles(new_value) => {

                self.edit_item.profiles = new_value.clone();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditWeaponMessage::ToggleNoPages(_new_value) => {
                let new_value = get_local_storage_bool("edit_forms_one_page", false);
                set_local_storage_bool("edit_forms_one_page", !new_value);
                return true;
            }

            EditWeaponMessage::UpdateName(new_value) => {
                self.edit_item.name = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditWeaponMessage::UpdatePage(new_value) => {
                self.edit_item.page = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditWeaponMessage::UpdateSummary(new_value) => {
                self.edit_item.summary = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditWeaponMessage::UpdateDescription(new_value) => {
                self.edit_item.description = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }

            EditWeaponMessage::UpdateActive(new_value) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditWeaponMessage::UpdateNoSelect(new_value) => {
                self.edit_item.active = new_value.to_owned();
                ctx.props().on_changed_callback.emit(self.edit_item.clone());
                return true;
            }
            EditWeaponMessage::UpdateEffects( new_value ) => {
                // let mut nv: Vec<String> = Vec::new();

                // for val in new_value.().split("\n") {
                //     nv.push( val.to_owned() );
                // }

                self.edit_item.effects = new_value.clone();
                ctx.props().on_changed_callback.emit( self.edit_item.clone());
                return true;
            }

            EditWeaponMessage::UpdateBookID(new_value) => {
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
                tag: "profiles".to_owned(),
                label: "Profiles".to_owned(),
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

        let toggle_no_pages = ctx.link().callback(EditWeaponMessage::ToggleNoPages);

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

        let change_page_callback_form = ctx.link().callback(EditWeaponMessage::ChangePage);
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

        let valid_pages = vec!["general", "admin", "profiles", "effects", "selection"];
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

        let readonly = ctx.props().readonly;
        let weapon_profiles = self.edit_item.profiles.clone();
        let update_weapon_profiles = ctx.link().callback(EditWeaponMessage::UpdateWeaponProfiles);


        html! {
            <div class="edit-form">
            {header}
            <div class="form-flex">
            if (current_page.as_str() == "admin" || current_page.as_str() == "__all__" ) && ctx.props().global_vars.current_user.has_admin_access() && ctx.props().for_admin {

                <fieldset class={"fieldset"}>
                    <legend>{"Admin"}</legend>

                    <div class="row">
                        <div class="col-md-6">
                            <InputCheckbox
                                label="Active"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.active}
                                onchange={ctx.link().callback( EditWeaponMessage::UpdateActive )}
                            />
                            <InputCheckbox
                                label="No Select"
                                readonly={ctx.props().readonly}
                                checked={self.edit_item.no_select}
                                onchange={ctx.link().callback( EditWeaponMessage::UpdateNoSelect )}
                            />
                        </div>
                        <div class="col-md-6">
                            <BookSelect
                                readonly={ctx.props().readonly}
                                current_user={ctx.props().global_vars.current_user.clone()}
                                book_list={book_list}
                                label={"Book"}
                                value={self.edit_item.book_id}
                                onchange={ ctx.link().callback( EditWeaponMessage::UpdateBookID) }
                            />
                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Page Number"}
                                inline={true}
                                value={(self.edit_item.page).to_owned()}
                                onchange={ ctx.link().callback( EditWeaponMessage::UpdatePage) }
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
                                onchange={ ctx.link().callback( EditWeaponMessage::UpdateName) }
                            />

                            <InputText
                                readonly={ctx.props().readonly}
                                label={"Summary"}

                                value={(self.edit_item.summary).to_owned()}
                                onchange={ ctx.link().callback( EditWeaponMessage::UpdateSummary) }
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
                                onchange={ ctx.link().callback( EditWeaponMessage::UpdateDescription) }
                            />
                        </div>
                    </div>
                </fieldset>
            }


            if current_page.as_str() == "profiles" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Weapon Profiles"}</legend>

                    <EditWeaponProfile
                        // global_vars={self.props().global_var}
                        readonly={readonly}
                        disable_removal_of_first={true}
                        weapon_profiles={weapon_profiles}
                        description={Some("All weapons have at least one profile, and cannot be deleted. Some weapons have more than one attack mode, either multiple ammunition types or different stats for one or two hands.".to_owned())}
                        weapon_profiles_updated={move |nv| {
                            // let update_profiles = ctx.link().callback( EditArmorMessage::UpdateIntegratedWeapon);
                            update_weapon_profiles.emit( nv )
                        }

                        }
                    />



                </fieldset>
            }

            if current_page.as_str() == "effects" || current_page.as_str() == "__all__"  {
                <fieldset class={"fieldset"}>
                    <legend>{"Effects"}</legend>
                    <div class="row full-width">
                        <div class="col-md-6">

                            <EffectsEntry
                                readonly={ctx.props().readonly}
                                label={"Effects"}
                                value={self.edit_item.effects.clone()}
                                onchange={ ctx.link().callback( EditWeaponMessage::UpdateEffects) }
                            />
                        </div>
                    </div>
                </fieldset>
            }

            if current_page.as_str() == "selection" || current_page.as_str() == "__all__" {
                <fieldset class={"fieldset"}>
                    <legend>{"Selection"}</legend>
                    // <TextArea
                    //     readonly={ctx.props().readonly}
                    //     label={"Conflicts"}
                    //     value={self.edit_item.conflicts.join("\n")}
                    //     onchange={ ctx.link().callback( EditWeaponMessage::UpdateConflicts) }
                    // />
                </fieldset>
            }
                </div>
            </div>
        }
    }
}

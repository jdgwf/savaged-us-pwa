use savaged_libs::player_character::weapon::WeaponProfile;
// use gloo_console::log;
use standard_components::ui::{nbsp::Nbsp, input_text::InputText, input_number::InputNumber, input_checkbox::InputCheckbox};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditWeaponProfileProps {
    // pub global_vars: GlobalVars,
    pub weapon_profiles: Vec<WeaponProfile>,

    pub weapon_profiles_updated: Callback<Vec<WeaponProfile>>,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub readonly: bool,

    #[prop_or_default]
    pub disable_removal_of_first: bool,
}

pub enum EditWeaponProfileMessage {

    UpdateIntegratedWeapon( Vec<WeaponProfile> ),
    AddIntegratedWeapon( MouseEvent ),
    RemoveIntegratedWeapon( usize ),
}

pub struct EditWeaponProfile {
    weapon_profiles: Vec<WeaponProfile>,
}

impl Component for EditWeaponProfile {
    type Message = EditWeaponProfileMessage;
    type Properties = EditWeaponProfileProps;

    fn create(ctx: &Context<Self>) -> Self {
        EditWeaponProfile {
            weapon_profiles: ctx.props().weapon_profiles.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: EditWeaponProfileMessage) -> bool {
        match msg {

            EditWeaponProfileMessage::UpdateIntegratedWeapon(new_value) => {

                self.weapon_profiles = new_value.clone();
                ctx.props().weapon_profiles_updated.emit(self.weapon_profiles.clone());
                return true;
            }

            EditWeaponProfileMessage::AddIntegratedWeapon( _event ) => {
                self.weapon_profiles.push(
                    WeaponProfile::default()
                );
                ctx.props().weapon_profiles_updated.emit(self.weapon_profiles.clone());
                return true;
            }

            EditWeaponProfileMessage::RemoveIntegratedWeapon(new_value) => {

                self.weapon_profiles.remove( new_value );

                ctx.props().weapon_profiles_updated.emit(self.weapon_profiles.clone());
                return true;
            }

        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.weapon_profiles = ctx.props().weapon_profiles.clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut description = html! {<></>};
        match &ctx.props().description {
            Some(desc) => {
                description = html! {<p>{desc.to_owned()}</p>};
            }
            None => {}
        }
        let header = html! {
            <>
                {description.to_owned()}
            </>
        };

        let mut profile_index = 0;

        let read_only = ctx.props().readonly;

        html! {
            <div>

                    <button
                        class="btn btn-primary pull-right"
                        type="button"
                        onclick={ctx.link().callback( EditWeaponProfileMessage::AddIntegratedWeapon )}
                    >
                        <fa class="fa-plus" /><Nbsp />{"Add Profile"}
                    </button>
                    {header}

                    /*
name:  "".to_string(),
            damage:  "".to_string(),
            damage_with_brackets:  "".to_string(),
            damage_original:  "".to_string(),
            parry_modifier:  0,
            range:  "".to_string(),
            reach:  0,
            : false,
            rof:  0,
            shots:  0,
            current_shots:  0,

            heavy_weapon: false,
            : false,
            : false,
            notes:  "".to_string(),
            equipped: false,

            to_hit_mod:  0,

            damage_dice_base:  "".to_string(),
            damage_dice_base_plus:  0,

            is_shield: false,
            : false,

            usable_in_melee: false,
            : false,
            ap:  0,
            ap_vs_rigid_armor_only:  0,

            vtt_only: false,

            skill_name:  "".to_string(),
            skill_value:  "".to_string(),
                    */

            if self.weapon_profiles.len() == 0 {
                <div class="text-center">
                    <hr />
                    {"There are are no weapon profiles, click on the Add Profile button to add one."}
                </div>
            } else {
                <>
                {self.weapon_profiles.clone().into_iter().map( move |weapon_profile| {

                        // let edit_item = self.edit_item.clone();
                        // let read_only = ctx.props().readonly;
                        profile_index += 1;

                        let weapon_profiles_name = self.weapon_profiles.clone();
                        // let weapon_profiles_min_str = self.weapon_profiles.clone();
                        // let weapon_profiles_base_range = self.weapon_profiles.clone();
                        let weapon_profiles_range = self.weapon_profiles.clone();
                        let weapon_profiles_ap = self.weapon_profiles.clone();
                        let weapon_profiles_melee_only = self.weapon_profiles.clone();
                        let weapon_profiles_requires_2_hands = self.weapon_profiles.clone();
                        let weapon_profiles_thrown_weapon = self.weapon_profiles.clone();
                        let weapon_profiles_counts_as_innate = self.weapon_profiles.clone();
                        let weapon_profiles_damage = self.weapon_profiles.clone();
                        let weapon_profiles_add_strength_to_damage = self.weapon_profiles.clone();

                        let update_profiles_callback_name = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profiles_callback_min_str = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profiles_callback_base_range = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_range = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_ap = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_melee_only = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_requires_2_hands = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_thrown_weapon = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_counts_as_innate = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_damage= ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let update_profiles_callback_add_strength_to_damage = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let remove_profile = ctx.link().callback(EditWeaponProfileMessage::RemoveIntegratedWeapon).clone();

                        let profile_one_read_only = read_only || (ctx.props().disable_removal_of_first == true && profile_index == 1);
                        // let mut profile_one_name = weapon_profile.name.to_owned();
                        // if profile_one_read_only {
                        //     profile_one_name = "(default)".to_owned();
                        // }
                        html!{
                        <fieldset class="fieldset">
                            if profile_one_read_only {
                                <legend class="small-text">{"Primary Profile"}</legend>
                            }

                                if !profile_one_read_only {
                                    <div>
                                        if ctx.props().disable_removal_of_first == false || profile_index > 1 {
                                            <button
                                                class="btn btn-danger pull-right"
                                                type="button"
                                                onclick={ Callback::from( move | _nv | {
                                                    let remove_profile = remove_profile.clone();

                                                    remove_profile.emit(profile_index - 1)
                                                }) }
                                            >
                                                <fa class="fa fa-trash" /><Nbsp />{"Remove Profile"}
                                            </button>

                                            <InputText
                                                readonly={read_only}
                                                label={"Alternate Profile Name"}
                                                value={weapon_profile.name.to_owned()}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_name = weapon_profiles_name.clone();
                                                    let update_profiles_callback_name = update_profiles_callback_name.clone();

                                                    profile_name[ profile_index - 1 ].name = nv;
                                                    update_profiles_callback_name.emit(profile_name)
                                                }) }
                                            />
                                        }
                                    </div>
                                }
                                <div class="row">
                                    <div class="col-md-6">

                                            // <SelectMinimumStrength
                                            //     label={"Minimum Strength"}
                                            //     readonly={read_only}
                                            //     inline={true}
                                            //     value={(weapon_profile.minimum_strength.to_string()).to_owned()}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_min_str = profile_min_str.clone();
                                            //         let update_profiles_callback_min_str = update_profiles_callback_min_str.clone();

                                            //         profile_min_str[ profile_index - 1 ].minimum_strength = nv;
                                            //         update_profiles_callback_min_str.emit(profile_min_str)
                                            //     }) }
                                            // />

                                            <InputText
                                                readonly={read_only}
                                                label={"Damage"}
                                                value={weapon_profile.damage.to_owned()}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_damage = weapon_profiles_damage.clone();
                                                    let update_profiles_callback_damage = update_profiles_callback_damage.clone();

                                                    profile_damage[ profile_index - 1 ].damage = nv;
                                                    update_profiles_callback_damage.emit(profile_damage)
                                                }) }
                                            />

                                            <InputCheckbox
                                                readonly={read_only}
                                                label={"Strength adds to damage"}
                                                checked={weapon_profile.add_strength_to_damage}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_add_strength_to_damage = weapon_profiles_add_strength_to_damage.clone();
                                                    let update_profiles_callback_add_strength_to_damage = update_profiles_callback_add_strength_to_damage.clone();

                                                    profile_add_strength_to_damage[ profile_index - 1 ].add_strength_to_damage = nv;
                                                    update_profiles_callback_add_strength_to_damage.emit(profile_add_strength_to_damage)
                                                }) }
                                            />

                                            <InputNumber
                                                readonly={read_only}
                                                label={"Armor Penetration (AP)"}
                                                step={"1"}
                                                inline={true}
                                                value={weapon_profile.ap as f32}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_ap = weapon_profiles_ap.clone();
                                                    let update_profiles_callback_ap = update_profiles_callback_ap.clone();

                                                    profile_ap[ profile_index - 1 ].ap = nv as i32;
                                                    update_profiles_callback_ap.emit(profile_ap)
                                                }) }
                                            />

                                            <InputCheckbox
                                                readonly={read_only}
                                                label={"Melee Only"}
                                                checked={weapon_profile.melee_only}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_melee_only = weapon_profiles_melee_only.clone();
                                                    let update_profiles_callback_melee_only = update_profiles_callback_melee_only.clone();

                                                    profile_melee_only[ profile_index - 1 ].melee_only = nv;
                                                    update_profiles_callback_melee_only.emit(profile_melee_only)
                                                }) }
                                            />

                                            <InputCheckbox
                                                readonly={read_only}
                                                label={"Thrown Weapon"}
                                                checked={weapon_profile.thrown_weapon}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_thrown_weapon = weapon_profiles_thrown_weapon.clone();
                                                    let update_profiles_callback_thrown_weapon = update_profiles_callback_thrown_weapon.clone();

                                                    profile_thrown_weapon[ profile_index - 1 ].thrown_weapon = nv;
                                                    update_profiles_callback_thrown_weapon.emit(profile_thrown_weapon)
                                                }) }
                                            />

                                            <InputText
                                                readonly={read_only}
                                                label={"Range"}
                                                value={weapon_profile.range.to_owned()}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_range = weapon_profiles_range.clone();
                                                    let update_profiles_callback_range = update_profiles_callback_range.clone();

                                                    profile_range[ profile_index - 1 ].range = nv;
                                                    update_profiles_callback_range.emit(profile_range)
                                                }) }
                                            />

                                            // <InputNumber
                                            //     readonly={read_only}
                                            //     label={"Secondary Armor Value"}
                                            //     step={"1"}
                                            //     inline={true}
                                            //     value={weapon_profile.secondary_armor_value as f32}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_sav = profile_sav.clone();
                                            //         let update_profiles_callback_sav = update_profiles_callback_sav.clone();

                                            //         profile_sav[ profile_index - 1 ].secondary_armor_value = nv as u32;
                                            //         update_profiles_callback_sav.emit(profile_sav)
                                            //     }) }
                                            // />

                                            // <InputNumber
                                            //     readonly={read_only}
                                            //     label={"Toughness Bonus"}
                                            //     step={"1"}
                                            //     inline={true}
                                            //     value={weapon_profile.toughness as f32}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_tb = profile_tb.clone();
                                            //         let update_profiles_callback_tb = update_profiles_callback_tb.clone();

                                            //         profile_tb[ profile_index - 1 ].toughness = nv as u32;
                                            //         update_profiles_callback_tb.emit(profile_tb)
                                            //     }) }
                                            // />

                                            // <InputCheckbox
                                            //     label="Heavy Armor"
                                            //     readonly={read_only}
                                            //     checked={weapon_profile.heavy}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_ha = profile_ha.clone();
                                            //         let update_profiles_callback_ha = update_profiles_callback_ha.clone();

                                            //         profile_ha[ profile_index - 1 ].heavy = nv;
                                            //         update_profiles_callback_ha.emit(profile_ha)
                                            //     }) }
                                            // />
                                        </div>

                                        <div class="col-md-6">
                                            <InputCheckbox
                                                readonly={read_only}
                                                label={"Counts as an innate weapon (for martial arts damage upgrades, etc)"}
                                                checked={weapon_profile.counts_as_innate}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_counts_as_innate = weapon_profiles_counts_as_innate.clone();
                                                    let update_profiles_callback_counts_as_innate = update_profiles_callback_counts_as_innate.clone();

                                                    profile_counts_as_innate[ profile_index - 1 ].counts_as_innate = nv;
                                                    update_profiles_callback_counts_as_innate.emit(profile_counts_as_innate)
                                                }) }
                                            />

                                            <InputCheckbox
                                                readonly={read_only}
                                                label={"Requires 2 Hands"}
                                                checked={weapon_profile.requires_2_hands}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_requires_2_hands = weapon_profiles_requires_2_hands.clone();
                                                    let update_profiles_callback_requires_2_hands = update_profiles_callback_requires_2_hands.clone();

                                                    profile_requires_2_hands[ profile_index - 1 ].requires_2_hands = nv;
                                                    update_profiles_callback_requires_2_hands.emit(profile_requires_2_hands)
                                                }) }
                                            />
                                            // <EffectsEntry
                                            //     readonly={read_only}
                                            //     description="These effects will apply when this item is equipped"
                                            //     label={"Effects"}
                                            //     value={weapon_profile.effects.clone()}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_effects = profile_effects.clone();
                                            //         let update_profiles_callback_effects = update_profiles_callback_effects.clone();

                                            //         profile_effects[ profile_index - 1 ].effects = nv;
                                            //         update_profiles_callback_effects.emit(profile_effects)
                                            //     }) }
                                            // />
                                        </div>
                                    </div>

                        </fieldset>
                        }
                    }

                ).collect::<Html>()}
                </>
            }

            </div>
        }
    }
}

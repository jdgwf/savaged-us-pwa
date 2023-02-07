use crate::libs::global_vars::GlobalVars;
use savaged_libs::player_character::weapon::WeaponProfile;
use gloo_console::log;
use standard_components::ui::{nbsp::Nbsp, input_text::InputText};
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
            requires_2_hands: false,
            rof:  0,
            shots:  0,
            current_shots:  0,

            heavy_weapon: false,
            melee_only: false,
            counts_as_innate: false,
            notes:  "".to_string(),
            equipped: false,

            to_hit_mod:  0,

            damage_dice_base:  "".to_string(),
            damage_dice_base_plus:  0,

            is_shield: false,
            thrown_weapon: false,

            usable_in_melee: false,
            add_strength_to_damage: false,
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

                        let weapon_profile_name = self.weapon_profiles.clone();
                        // let weapon_profiles_min_str = self.weapon_profiles.clone();
                        // let weapon_profiles_av = self.weapon_profiles.clone();
                        // let weapon_profiles_sav = self.weapon_profiles.clone();
                        // let weapon_profiles_tb = self.weapon_profiles.clone();
                        // let weapon_profiles_ha = self.weapon_profiles.clone();
                        // let weapon_profiles_effects = self.weapon_profiles.clone();
                        let update_profile_callback_name = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profile_callback_min_str = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profile_callback_av = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profile_callback_sav = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profile_callback_tb = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profile_callback_ha = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        // let update_profile_callback_effects = ctx.link().callback(EditWeaponProfileMessage::UpdateIntegratedWeapon).clone();
                        let remove_profile = ctx.link().callback(EditWeaponProfileMessage::RemoveIntegratedWeapon).clone();

                        let profile_one_read_only = read_only || (ctx.props().disable_removal_of_first == true && profile_index == 1);
                        let mut profile_one_name = weapon_profile.name.to_owned();
                        if profile_one_read_only {
                            profile_one_name = "(default)".to_owned();
                        }
                        html!{
                        <fieldset class="fieldset">
                            <table class="full-width">
                                <tbody>
                                    <tr>
                                        <td colspan="2">
                                        if ctx.props().disable_removal_of_first == false || profile_index > 1 {
                                            <button
                                                class="btn btn-danger pull-right"
                                                type="button"
                                                onclick={ Callback::from( move | nv | {
                                                    let remove_profile = remove_profile.clone();

                                                    remove_profile.emit(profile_index - 1)
                                                }) }
                                            >
                                                <fa class="fa fa-trash" /><Nbsp />{"Remove Profile"}
                                            </button>
                                        }
                                            <InputText
                                                readonly={profile_one_read_only}
                                                label={"Name"}
                                                value={(profile_one_name).to_owned()}
                                                onchange={ Callback::from( move | nv | {
                                                    let mut profile_name = weapon_profile_name.clone();
                                                    let update_profile_callback_name = update_profile_callback_name.clone();

                                                    profile_name[ profile_index - 1 ].name = nv;
                                                    update_profile_callback_name.emit(profile_name)
                                                }) }
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>


                                            // <SelectMinimumStrength
                                            //     label={"Minimum Strength"}
                                            //     readonly={read_only}
                                            //     inline={true}
                                            //     value={(weapon_profile.minimum_strength.to_string()).to_owned()}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_min_str = profile_min_str.clone();
                                            //         let update_profile_callback_min_str = update_profile_callback_min_str.clone();

                                            //         profile_min_str[ profile_index - 1 ].minimum_strength = nv;
                                            //         update_profile_callback_min_str.emit(profile_min_str)
                                            //     }) }
                                            // />

                                            // <InputNumber
                                            //     readonly={read_only}
                                            //     label={"Armor Value"}
                                            //     step={"1"}
                                            //     inline={true}
                                            //     value={weapon_profile.armor_value as f32}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_av = profile_av.clone();
                                            //         let update_profile_callback_av = update_profile_callback_av.clone();

                                            //         profile_av[ profile_index - 1 ].armor_value = nv as u32;
                                            //         update_profile_callback_av.emit(profile_av)
                                            //     }) }
                                            // />

                                            // <InputNumber
                                            //     readonly={read_only}
                                            //     label={"Secondary Armor Value"}
                                            //     step={"1"}
                                            //     inline={true}
                                            //     value={weapon_profile.secondary_armor_value as f32}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_sav = profile_sav.clone();
                                            //         let update_profile_callback_sav = update_profile_callback_sav.clone();

                                            //         profile_sav[ profile_index - 1 ].secondary_armor_value = nv as u32;
                                            //         update_profile_callback_sav.emit(profile_sav)
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
                                            //         let update_profile_callback_tb = update_profile_callback_tb.clone();

                                            //         profile_tb[ profile_index - 1 ].toughness = nv as u32;
                                            //         update_profile_callback_tb.emit(profile_tb)
                                            //     }) }
                                            // />

                                            // <InputCheckbox
                                            //     label="Heavy Armor"
                                            //     readonly={read_only}
                                            //     checked={weapon_profile.heavy}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_ha = profile_ha.clone();
                                            //         let update_profile_callback_ha = update_profile_callback_ha.clone();

                                            //         profile_ha[ profile_index - 1 ].heavy = nv;
                                            //         update_profile_callback_ha.emit(profile_ha)
                                            //     }) }
                                            // />
                                        </td>
                                        <td>
                                            // <EffectsEntry
                                            //     readonly={read_only}
                                            //     description="These effects will apply when this item is equipped"
                                            //     label={"Effects"}
                                            //     value={weapon_profile.effects.clone()}
                                            //     onchange={ Callback::from( move | nv | {
                                            //         let mut profile_effects = profile_effects.clone();
                                            //         let update_profile_callback_effects = update_profile_callback_effects.clone();

                                            //         profile_effects[ profile_index - 1 ].effects = nv;
                                            //         update_profile_callback_effects.emit(profile_effects)
                                            //     }) }
                                            // />
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
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

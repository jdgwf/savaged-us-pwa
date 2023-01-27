use crate::components::image_uploader::ImageUploader;
use crate::components::ui_page::UIPage;
use crate::libs::fetch_api::fetch_api_with_value;
use crate::libs::fetch_api::update_user;
use crate::libs::global_vars::GlobalVars;
use chrono::Utc;
use chrono_tz::TZ_VARIANTS;
use gloo_console::error;
use gloo_console::log;
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::user::User;
use serde_json::Error;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::input_text::InputText;
use standard_components::ui::markdown_editor::MarkdownEditor;
use standard_components::ui::nbsp::Nbsp;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SettingsPublicProps {
    pub global_vars: GlobalVars,
}

pub enum SettingsPublicMessage {
    SaveShareSettings(MouseEvent),
    ResetShareSettings(MouseEvent),
    UpdateSharedSettingsSaved(String),
    UpdateDisplayName(String),
    UpdateTwitter(String),
    UpdateBio(String),
    SaveTimezone(Event),
    UpdateTimezoneMessage(String),
    UpdateUsername(String),
    SaveYourInformation(MouseEvent),
    ResetYourInformation(MouseEvent),
    UsernameIsAvailable(bool),
    UpdateUsernameSaved(String),
    ImageChanged(String),
    UpdateShowProfileImage(bool),
    UpdateShowUserPage(bool),
}

pub struct SettingsPublic {
    current_user: User,
    edit_bio: String,

    edit_share_display_name: String,
    edit_twitter: String,
    edit_share_show_profile_image: bool,
    edit_show_user_page: bool,

    edit_username: String,
    update_username_message: String,
    update_username_can_save: bool,
    update_username_notification_class: String,

    update_shared_settings_message: String,
    update_tz_message: String,
}

fn your_information_can_save(
    original_show_user_page: bool,
    current_show_user_page: bool,
    update_username_can_save: bool,
    is_current_username: bool,
) -> bool {
    if original_show_user_page != current_show_user_page {
        if !is_current_username {
            return update_username_can_save;
        }
        return true;
    } else {
        return update_username_can_save;
    }
}

impl Component for SettingsPublic {
    type Message = SettingsPublicMessage;
    type Properties = SettingsPublicProps;

    fn create(ctx: &Context<Self>) -> Self {
        let global_vars = ctx.props().global_vars.clone();

        set_document_title(
            global_vars.site_title.to_owned(),
            "Public Settings".to_owned(),
            global_vars.server_side_renderer,
        );
        SettingsPublic {
            edit_username: global_vars.current_user.username.clone(),
            edit_bio: global_vars.current_user.share_bio.clone(),
            edit_share_display_name: global_vars.current_user.share_display_name.clone(),
            edit_twitter: global_vars.current_user.twitter.clone(),
            edit_share_show_profile_image: global_vars.current_user.share_show_profile_image,
            edit_show_user_page: global_vars.current_user.show_user_page,
            current_user: global_vars.current_user.clone(),
            update_shared_settings_message: "".to_owned(),
            update_tz_message: "".to_owned(),
            update_username_message:
                "This is your current Username. Start typing in the field above to change it."
                    .to_owned(),
            update_username_notification_class: "alert alert-info text-center".to_owned(),
            update_username_can_save: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: SettingsPublicMessage) -> bool {
        match msg {
            SettingsPublicMessage::SaveYourInformation(_event) => {
                let api_root = ctx.props().global_vars.api_root.clone();
                let login_token = ctx.props().global_vars.login_token.clone();

                let updated_username_notification = ctx
                    .link()
                    .callback(SettingsPublicMessage::UpdateUsernameSaved)
                    .clone();

                let global_vars = ctx.props().global_vars.clone();

                let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                let username = self.edit_username.clone();
                let edit_share_display_name = self.edit_share_display_name.clone();
                spawn_local(async move {
                    let api_root = api_root.clone();
                    let login_token = login_token.clone();

                    let username = username.clone();
                    let edit_share_display_name = edit_share_display_name.clone();
                    let global_vars = global_vars.clone();

                    let result = fetch_api_with_value(
                        (api_root + "/user/save-username").to_owned(),
                        "".to_owned(),
                        login_token,
                        username.clone(),
                        "username".to_owned(),
                    )
                    .await;

                    match result {
                        Ok(value) => {
                            // let success_result = value.into_serde::< bool >();
                            let success_result: Result<bool, Error> =
                                JsValueSerdeExt::into_serde(&value);
                            match success_result {
                                Ok(_is_available_value) => {
                                    let edit_share_display_name = edit_share_display_name.clone();
                                    let mut global_vars = global_vars.clone();
                                    // self.current_user.share_display_name = edit_share_display_name.to_owned();
                                    global_vars.current_user.share_display_name =
                                        edit_share_display_name.to_owned();
                                    global_vars.current_user.username = username.clone();

                                    update_user(
                                        global_vars,
                                        update_global_vars,
                                        updated_username_notification,
                                        "".to_owned(),
                                        false,
                                    );
                                }
                                Err(err) => {
                                    // let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                                    // set_notifications.emit( Vec::new() );
                                    // error!( &err_string  );
                                    error!("update() data error", &err.to_string());
                                }
                            }
                        }
                        Err(err) => {
                            // set_notifications.emit( Vec::new() );
                            // console::error_2("get_data_via_fetch Err()", &err );
                            error!("update() fetch error", &err);
                        }
                    }
                });

                true
            }
            SettingsPublicMessage::ResetYourInformation(_event) => {
                self.edit_username = self.current_user.username.clone();
                self.edit_show_user_page = self.current_user.show_user_page;
                self.update_username_can_save = false;
                self.update_username_message =
                    "This is your current Username. Start typing in the field above to change it."
                        .to_owned();
                self.update_username_notification_class = "alert alert-info text-center".to_owned();
                true
            }

            SettingsPublicMessage::SaveTimezone(event) => {
                let input: HtmlInputElement = event.target_unchecked_into();
                self.current_user.timezone = input.value().to_owned();
                let updated_tz_notification = ctx
                    .link()
                    .callback(SettingsPublicMessage::UpdateTimezoneMessage)
                    .clone();

                let mut global_vars = ctx.props().global_vars.clone();

                global_vars.current_user = self.current_user.clone();

                let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                update_user(
                    global_vars,
                    update_global_vars,
                    updated_tz_notification,
                    "".to_owned(),
                    false,
                );

                true
            }

            SettingsPublicMessage::UpdateShowProfileImage(new_value) => {
                self.edit_share_show_profile_image = new_value;
                true
            }

            SettingsPublicMessage::UpdateShowUserPage(new_value) => {
                // let input: HtmlInputElement = event.target_unchecked_into();
                self.current_user.show_user_page = new_value;
                // let updated_tz_notification = ctx.link().callback(SettingsPublicMessage::UpdateTimezoneMessage).clone();

                let mut global_vars = ctx.props().global_vars.clone();

                global_vars.current_user = self.current_user.clone();

                let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                update_user(
                    global_vars,
                    update_global_vars,
                    Callback::noop(),
                    "".to_owned(),
                    false,
                );

                true
                // self.edit_show_user_page = new_value;
                // self.update_username_can_save = your_information_can_save(
                //     self.edit_show_user_page,
                //     self.current_user.show_user_page,
                //     self.update_username_can_save,
                //     self.edit_username == self.current_user.username,
                // );
                // true
            }

            SettingsPublicMessage::UpdateUsername(new_value) => {
                self.edit_username = new_value.to_owned().to_lowercase();
                let s: String = self
                    .edit_username
                    .chars()
                    .map(|x| match x {
                        '-' => '-',
                        '_' => '_',
                        '0'..='9' => x,
                        'a'..='z' => x,
                        _ => ' ',
                    })
                    .collect();

                self.edit_username = s.replace(" ", "").clone();

                let api_root = ctx.props().global_vars.api_root.clone();
                let login_token = ctx.props().global_vars.login_token.clone();
                let set_is_available = ctx
                    .link()
                    .callback(SettingsPublicMessage::UsernameIsAvailable);
                let username = self.edit_username.clone();

                spawn_local(async move {
                    let api_root = api_root.clone();
                    let login_token = login_token.clone();
                    let username = username.clone();

                    let result = fetch_api_with_value(
                        (api_root + "/user/username-available").to_owned(),
                        "".to_owned(),
                        login_token,
                        username,
                        "username".to_owned(),
                    )
                    .await;

                    match result {
                        Ok(value) => {
                            // let success_result = value.into_serde::< bool >();
                            let success_result: Result<bool, Error> =
                                JsValueSerdeExt::into_serde(&value);
                            match success_result {
                                Ok(is_available_value) => set_is_available.emit(is_available_value),
                                Err(err) => {
                                    // let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                                    // set_notifications.emit( Vec::new() );
                                    // log!( &err_string  );
                                    error!(
                                        "SettingsPublicMessage::UpdateUsername data error",
                                        &err.to_string()
                                    );
                                }
                            }
                        }
                        Err(err) => {
                            // set_notifications.emit( Vec::new() );
                            // console::error_2("get_data_via_fetch Err()", &err );
                            error!("SettingsPublicMessage::UpdateUsername data error", &err);
                        }
                    }
                });

                true
            }

            SettingsPublicMessage::ImageChanged(new_url) => {
                let mut remove_image = false;

                let mut global_vars = ctx.props().global_vars.clone();

                global_vars.current_user.updated_on = Some(Utc::now());
                if new_url == "".to_owned() {
                    // log!("Removing User Image");
                    remove_image = true;
                    global_vars.current_user.image_url = "".to_owned();
                    global_vars.current_user.profile_image = "".to_owned();
                } else {
                    global_vars.current_user.image_url = new_url.to_owned();
                    global_vars.current_user.profile_image = "webp".to_owned();
                }

                // ctx.props().global_vars = global_vars.clone();

                let mut global_vars = ctx.props().global_vars.clone();

                global_vars.current_user = self.current_user.clone();

                let update_global_vars = ctx.props().global_vars.update_global_vars.clone();

                update_global_vars.emit(global_vars.clone());

                update_user(
                    global_vars,
                    update_global_vars,
                    Callback::noop(),
                    "".to_owned(),
                    remove_image,
                );

                true
            }

            SettingsPublicMessage::UpdateUsernameSaved(saved) => {
                if saved == "User Updated".to_owned() {
                    self.current_user.username = self.edit_username.to_owned();
                    self.current_user.show_user_page = self.edit_show_user_page.to_owned();
                    self.update_username_can_save = false;
                    self.update_username_message =
                        "Your username has been successfully changed!".to_owned();
                    self.update_username_notification_class =
                        "alert alert-success text-center".to_owned();
                } else {
                    // log!("UpdateUsernameSaved saved", saved);
                    self.update_username_can_save = true;
                    self.update_username_message =
                        "Oh no! Something went wrong! Please try again.".to_owned();
                    self.update_username_notification_class =
                        "alert alert-danger text-center".to_owned();
                }
                true
            }

            SettingsPublicMessage::UsernameIsAvailable(is_available) => {
                // log!( "UsernameIsAvailable", is_available );
                if self.edit_username.len() < 3 {
                    self.update_username_can_save = false;
                    self.update_username_message =
                        "Username must be 3 characters or more".to_owned();
                    self.update_username_notification_class =
                        "alert alert-danger text-center".to_owned();
                } else {
                    if self.edit_username == self.current_user.username {
                        self.update_username_can_save = false;
                        self.update_username_message = "This is your current Username. Start typing in the field above to change it.".to_owned();
                        self.update_username_notification_class =
                            "alert alert-info text-center".to_owned();
                    } else {
                        if is_available {
                            self.update_username_can_save = true;
                            self.update_username_message =
                                "Username is available! Press Save below to make it yours!"
                                    .to_owned();
                            self.update_username_notification_class =
                                "alert alert-success text-center".to_owned();
                        } else {
                            self.update_username_can_save = false;
                            self.update_username_message =
                                "Sorry, this username has been taken.".to_owned();
                            self.update_username_notification_class =
                                "alert alert-danger text-center".to_owned();
                        }
                    }
                }

                self.update_username_can_save = your_information_can_save(
                    self.edit_show_user_page,
                    self.current_user.show_user_page,
                    self.update_username_can_save,
                    self.edit_username == self.current_user.username,
                );

                true
            }

            SettingsPublicMessage::UpdateBio(new_value) => {
                self.edit_bio = new_value.to_owned();
                true
            }

            SettingsPublicMessage::UpdateTwitter(new_value) => {
                self.edit_twitter = new_value.to_owned();
                true
            }

            SettingsPublicMessage::UpdateDisplayName(new_value) => {
                self.edit_share_display_name = new_value.to_owned();
                true
            }

            SettingsPublicMessage::SaveShareSettings(_event) => {
                self.current_user.twitter = self.edit_twitter.to_owned();
                self.current_user.share_display_name = self.edit_share_display_name.to_owned();
                self.current_user.share_show_profile_image = self.edit_share_show_profile_image;
                self.current_user.share_bio = self.edit_bio.to_owned();

                let updated_user_notification = ctx
                    .link()
                    .callback(SettingsPublicMessage::UpdateSharedSettingsSaved)
                    .clone();
                // let reset_user_notification = ctx.link().callback(SettingsPublicMessage::ResetShareSettings(())).clone();

                let mut global_vars = ctx.props().global_vars.clone();

                global_vars.current_user = self.current_user.clone();

                let update_global_vars = ctx.props().global_vars.update_global_vars.clone();

                update_user(
                    global_vars,
                    update_global_vars,
                    updated_user_notification,
                    "".to_owned(),
                    false,
                );

                true
            }

            SettingsPublicMessage::ResetShareSettings(_event) => {
                self.edit_twitter = self.current_user.twitter.to_owned();
                self.edit_share_display_name = self.current_user.share_display_name.to_owned();
                self.edit_bio = self.current_user.share_bio.to_owned();
                true
            }

            SettingsPublicMessage::UpdateSharedSettingsSaved(message) => {
                self.update_shared_settings_message = message.clone();
                true
            }

            SettingsPublicMessage::UpdateTimezoneMessage(message) => {
                self.update_tz_message = message.clone();
                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &SettingsPublicProps) -> bool {
        self.current_user = ctx.props().global_vars.current_user.clone();

        self.edit_username = self.current_user.username.clone();
        self.edit_bio = self.current_user.share_bio.clone();
        self.edit_share_display_name = self.current_user.share_display_name.clone();
        self.edit_twitter = self.current_user.twitter.clone();
        self.edit_share_show_profile_image = self.current_user.share_show_profile_image;
        self.edit_show_user_page = self.current_user.show_user_page;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let global_vars = ctx.props().global_vars.clone();
        let mut global_vars = ctx.props().global_vars.clone();

        global_vars.current_menu = "main-user-login".to_owned();
        global_vars.current_sub_menu = "settings-public".to_owned();

        if global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
                    page_title="Settings"

                >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            };
        }

        if global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
                    page_title="Settings"

                >
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
                </UIPage>
            };
        }

        let mut share_settings_save_disabled = true;

        if global_vars.current_user.share_display_name != self.edit_share_display_name
            || self.current_user.share_bio != self.edit_bio
            || self.current_user.twitter != self.edit_twitter
        {
            share_settings_save_disabled = false;
        }

        html! {
            <UIPage
                global_vars={global_vars.clone()}
                page_title="Public Settings"

            >
                <h2><i class={"fa-solid fa-globe"}></i><Nbsp />{"Public Settings"}</h2>
                <p class={"text-center"}>
                    {"Anything here can potentially be shared on the public side, so if you wish to remain anonymous to everyone this is the place to do it!"}
                </p>
                <div class={"row"}>
                    <div class={"col-xs-12 col-md-6"}>
                        <fieldset class={"fieldset"}>
                            <legend>{"Username & User Page Settings"}</legend>

                            <InputCheckbox
                                label={" Show Public Profile Page"}
                                checked={self.edit_show_user_page}
                                onchange={ctx.link().callback( SettingsPublicMessage::UpdateShowUserPage )}
                            >

                                <div class={"small-text"}>{"If you'd like a personal page showing your settings from above and your current public shares, check this checkbox."}</div>
                            </InputCheckbox>
                            if self.current_user.show_user_page {
                                <InputText
                                    label={"Your Personal Page URL"}
                                    readonly={true}
                                    value={self.current_user.get_user_url()}
                                />
                            }

                            <InputText
                                label={"Your Username"}
                                description={"All lower case. Only alpha numeric characters or a dash or underscore are allowed."}
                                value={self.edit_username.clone()}
                                onchange={ctx.link().callback( SettingsPublicMessage::UpdateUsername )}
                            />

                            if !self.update_username_message.is_empty() {
                                <div class={self.update_username_notification_class.clone()}>{self.update_username_message.clone()}</div>
                            }

                            <div class="text-right">
                                <button
                                    type="button"
                                    class="btn btn-secondary"
                                    disabled={&self.edit_username == &self.current_user.username && self.edit_show_user_page == self.current_user.show_user_page}
                                    onclick={ctx.link().callback( SettingsPublicMessage::ResetYourInformation )}
                                >
                                    <i class="fa fa-cancel" /><Nbsp />{"Cancel"}
                                </button>
                                <Nbsp />
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    disabled={!self.update_username_can_save}
                                    onclick={ctx.link().callback( SettingsPublicMessage::SaveYourInformation )}
                                >
                                    <i class="fa fa-floppy-disk" /><Nbsp />{"Save"}
                                </button>
                            </div>

                        </fieldset>
                        <fieldset class={"fieldset"}>
                        <legend>{"Share Settings"}</legend>

                        <InputText
                            label={"Show your name as"}
                            // readonly={true}
                            value={self.edit_share_display_name.clone()}
                            onchange={ctx.link().callback( SettingsPublicMessage::UpdateDisplayName )}
                        />

                        <InputText
                            label={"Your Twitter Handle"}
                            // readonly={true}
                            value={self.edit_twitter.clone()}
                            onchange={ctx.link().callback( SettingsPublicMessage::UpdateTwitter )}
                        >
                        <div class={"small-text"}>{"Feel free to use @YourHandle or just YourHandle without the @ - we'll format and link it properly. If non-empty this will show up on User Badge and, if you're a WildCard subscriber, your Bio"}</div>
                        </InputText>

                        <InputCheckbox
                            label={"Show profile image"}
                            checked={self.edit_share_show_profile_image}
                            onchange={ctx.link().callback( SettingsPublicMessage::UpdateShowProfileImage )}
                        >
                            <div class={"small-text"}>{"This will show your image on the General page on your User Badge and, if you're a WildCard subscriber, your Bio"}</div>
                        </InputCheckbox>

                        <MarkdownEditor
                            label={"A Brief Bio"}
                            // readonly={true}
                            value={self.edit_bio.clone()}
                            onchange={ctx.link().callback( SettingsPublicMessage::UpdateBio )}
                            description={"This will show up on your shared user page."}
                        />

                        if !self.update_shared_settings_message.is_empty() {
                            <div class={"alert alert-success"}>
                                // {self.update_shared_settings_message.clone()}
                                {"Share Settings Saved!"}
                            </div>
                        }

                        <div class="text-right">
                            <button
                                type="button"
                                class="btn btn-secondary"
                                disabled={share_settings_save_disabled}
                                onclick={ctx.link().callback( SettingsPublicMessage::ResetShareSettings )}
                            >
                                <i class="fa fa-cancel" /><Nbsp />{"Cancel"}
                            </button>
                            <Nbsp />
                            <button
                                type="button"
                                class="btn btn-primary"
                                disabled={share_settings_save_disabled}
                                onclick={ctx.link().callback( SettingsPublicMessage::SaveShareSettings )}
                            >
                                <i class="fa fa-floppy-disk" /><Nbsp />{"Save"}
                            </button>
                        </div>

                    </fieldset>
                    </div>

                    <div class={"col-xs-12 col-md-6"}>
                        <fieldset class={"fieldset"}>
                            <legend>{"Timezone"}</legend>

                            <div class={"alert alert-info"}>{"The timezone is listed here only because in the future shared games may display your timezone as a convenience with other players and GMs"}</div>
                            <div class={"text-center small-text"}>{"This will adjust the dates and times displayed on the website to your local time while you're logged in"}</div>

                            <label>
                            <select
                                value={self.current_user.timezone.to_owned()}
                                onchange={ctx.link().callback( SettingsPublicMessage::SaveTimezone )}
                            >
                            {TZ_VARIANTS.into_iter().map(|tz| {
                                html!{
                                    if tz.to_string() == self.current_user.timezone {
                                        <option selected=true value={tz.to_string()}>{tz.to_string()}</option>
                                    } else {
                                        <option value={tz.to_string()}>{tz.to_string()}</option>
                                    }
                                }}).collect::<Html>()}
                            </select>
                            </label>

                            if !self.update_tz_message.is_empty() {
                                <div class={"alert alert-success"}>
                                    {"Timezone Updated!"}
                                </div>
                            }

                            <div class={"text-center"}><strong>{"Selected Zone Current Time:"}</strong><Nbsp />{self.current_user.format_datetime( Utc::now(), false, true, true )}</div>
                        </fieldset>

                        <fieldset class={"fieldset"}>
                            <legend>{"User Portrait"}</legend>
                            <ImageUploader
                                global_vars={global_vars.clone()}
                                upload_url={"".to_owned()}
                                label={"User Image Upload".to_owned()}
                                is_default_image={self.current_user.profile_image.is_empty()}
                                image_name={"user".to_owned()}
                                image_style={"border-radius: 50%;".to_owned()}
                                on_changed_callback={ctx.link().callback( SettingsPublicMessage::ImageChanged )}
                                image_url={self.current_user.get_image(&ctx.props().global_vars.server_root ).clone()}
                            />
                            <br />

                        </fieldset>

                    </div>
                </div>

            </UIPage>

        }
    }
}

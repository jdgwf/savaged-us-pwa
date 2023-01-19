use savaged_libs::user::User;
use yew_router::prelude::*;
use yew::prelude::*;
use standard_components::ui::input_text::InputText;
use standard_components::ui::input_checkbox::InputCheckbox;
use standard_components::ui::nbsp::Nbsp;
use standard_components::libs::set_document_title::set_document_title;
use crate::components::ui_page::UIPage;
use crate::libs::fetch_api::update_user;
use serde_json;
use crate::libs::global_vars::GlobalVars;
use savaged_libs::hidden_banner::HiddenBanner;
use super::UserRoute;
use gloo_console::error;
// use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

#[derive(Properties, PartialEq)]
pub struct SettingsPrivateProps {
    pub global_vars: GlobalVars,
}

pub enum SettingsPrivateMessage {
    UpdateVerifyPassword( String ),
    UpdatePassword( String ),
    UpdateFirstName( String ),
    UpdateLastName( String ),
    UpdateEmail( String ),
    SaveInfo( MouseEvent ),
    ResetInfo( MouseEvent ),
    UpdateInformationSaved( String ),
    PasswordsUpdated( String ),
    SetReceiveNotifications( bool ),
    SetTurnOffAdvanceOptions( bool ),
    RemoveHiddenBanner( u32 ),
    ResetPasswords( MouseEvent),
    SavePasswords( MouseEvent),
}

pub struct SettingsPrivate {
    current_user: User,
    first_name: String,
    last_name: String,
    email: String,
    update_info_message: String,
    password_reset_message: String,
    password_reset_class: String,
    password: String,
    verify_password: String,
}

fn check_passwords (
    password: String,
    verify_password: String,
)-> ( String, String ) {
    let mut message_class = "alert alert-info".to_owned();
    let mut message_text = "To update your password, just type it twice in the fields above.".to_owned();

    if !password.is_empty() || !verify_password.is_empty() {
        message_class = "alert alert-warning".to_owned();
        message_text = "Be sure to fill in both fields!".to_owned();

        if password.len() < 8  {
            message_class = "alert alert-danger".to_owned();
            message_text = "Your password is too short!".to_owned();
        }
        if !password.is_empty() && !verify_password.is_empty() {

            if password == verify_password {
                message_class = "alert alert-success".to_owned();
                message_text = "Looks good! You're ready to set your password!".to_owned();
            } else {
                message_class = "alert alert-danger".to_owned();
                message_text = "Your passwords do not match!".to_owned();
            }
        }

    }

    return ( message_class, message_text );
}

impl Component for SettingsPrivate {
    type Message = SettingsPrivateMessage;
    type Properties = SettingsPrivateProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();

        set_document_title(global_vars.site_title.to_owned(), "Private Settings".to_owned(), global_vars.server_side_renderer,);
        SettingsPrivate {
            current_user: global_vars.current_user.clone(),
            first_name: global_vars.current_user.first_name.to_owned(),
            last_name: global_vars.current_user.last_name.to_owned(),
            email: global_vars.current_user.email.to_owned(),
            update_info_message: "".to_owned(),
            password_reset_message: "To update your password, just type it twice in the fields above.".to_owned(),
            password_reset_class: "alert alert-info".to_owned(),
            password: "".to_owned(),
            verify_password: "".to_owned(),
        }
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: SettingsPrivateMessage,
    ) -> bool {

        match msg {

            SettingsPrivateMessage::SaveInfo( _event ) => {

                self.current_user.first_name = self.first_name.to_owned();
                self.current_user.last_name = self.last_name.to_owned();
                self.current_user.email = self.email.to_owned();

                let updated_user_notification = ctx.link().callback(SettingsPrivateMessage::UpdateInformationSaved).clone();
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

            SettingsPrivateMessage::ResetInfo( _event ) => {

                self.first_name = self.current_user.first_name.to_owned();
                self.last_name = self.current_user.last_name.to_owned();
                self.email = self.current_user.email.to_owned();
                true
            }
            SettingsPrivateMessage::SavePasswords( _event ) => {

                let updated_user_notification = ctx.link().callback(SettingsPrivateMessage::PasswordsUpdated).clone();
                let mut global_vars = ctx.props().global_vars.clone();

                global_vars.current_user = self.current_user.clone();

                let update_global_vars = ctx.props().global_vars.update_global_vars.clone();
                update_user(
                    global_vars,
                    update_global_vars,
                    updated_user_notification,
                    self.password.to_owned(),
                    false,
                );

                true
            }

            SettingsPrivateMessage::SetReceiveNotifications( new_value ) => {
                self.current_user.notify_email = new_value;

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
            }

            SettingsPrivateMessage::SetTurnOffAdvanceOptions( new_value ) => {
                self.current_user.turn_off_advance_limits = new_value;

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
            }

            SettingsPrivateMessage::UpdateInformationSaved( message ) => {
                self.update_info_message = message.clone();
                true
            }

            SettingsPrivateMessage::PasswordsUpdated( _message ) => {
                self.password = "".to_owned();
                self.verify_password = "".to_owned();
                self.password_reset_class = "alert alert-success".to_owned();
                self.password_reset_message = "Congratulations! Your password has been reset!".to_owned();
                true
            }

            SettingsPrivateMessage::ResetPasswords( _event ) => {
                self.password_reset_class = "alert alert-info".to_owned();
                self.password_reset_message = "To update your password, just type it twice in the fields above.".to_owned();
                self.password = "".to_owned();
                self.verify_password = "".to_owned();
                true
            }

            SettingsPrivateMessage::UpdateVerifyPassword( new_value ) => {

                self.verify_password = new_value.to_owned();
                (self.password_reset_class, self.password_reset_message) = check_passwords( self.password.clone(), self.verify_password.clone() );
                true
            }
            SettingsPrivateMessage::UpdatePassword( new_value ) => {

                self.password = new_value.to_owned();
                (self.password_reset_class, self.password_reset_message) = check_passwords( self.password.clone(), self.verify_password.clone() );
                true
            }

            SettingsPrivateMessage::UpdateFirstName( new_value ) => {

                self.first_name = new_value.to_owned();
                true
            }
            SettingsPrivateMessage::UpdateLastName( new_value ) => {
                self.last_name = new_value.to_owned();
                true
            }
            SettingsPrivateMessage::UpdateEmail( new_value ) => {
                self.email = new_value.to_owned();
                true
            }

            SettingsPrivateMessage::RemoveHiddenBanner( remove_id ) => {

                let mut hidden_banners: Vec<HiddenBanner> = Vec::new();
                let mut new_hidden_banners: Vec<HiddenBanner> = Vec::new();
                let hidden_banners_result : Result<Vec<HiddenBanner>, serde_json::Error> = serde_json::from_str(  &self.current_user.hidden_banners.as_ref() );
                match hidden_banners_result {
                    Ok( post_val ) => {
                        hidden_banners = post_val;
                    }
                    Err( err ) => {
                        error!("SettingsPrivateMessage::RemoveHiddenBanner hidden_banners_result data parse error", err.to_string());
                    }
                }

                for banner in hidden_banners.into_iter() {
                    if banner.id != remove_id {
                        new_hidden_banners.push( banner.clone() );
                    }
                }

                let to_string_result = serde_json::to_string( &new_hidden_banners);
                match to_string_result {
                    Ok( string_value ) => {
                        self.current_user.hidden_banners = string_value.clone();
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
                    }
                    Err( _error ) => {

                    }
                }

                true
            }

        }

    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &SettingsPrivateProps,
    ) -> bool {

        self.current_user = ctx.props().global_vars.current_user.clone();

        self.first_name = self.current_user.first_name.to_owned();
        self.last_name = self.current_user.last_name.to_owned();
        self.email = self.current_user.email.to_owned();

        true
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        // let global_vars = ctx.props().global_vars.clone();
        let mut global_vars = ctx.props().global_vars.clone();
        global_vars.current_sub_menu = "settings_private".to_owned();

        if ctx.props().global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="Settings"
                    submenu_tag={"user".to_owned()}
                >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            }
        }

        if ctx.props().global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="Settings"
                    submenu_tag={"user".to_owned()}
                >
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
                </UIPage>
            }
        }

        let mut your_info_save_disabled = true;

        if
            self.first_name != self.current_user.first_name
                ||
            self.last_name != self.current_user.last_name
                ||
            self.email != self.current_user.email
        {
            your_info_save_disabled = false;
        }

        let mut hidden_banners: Vec<HiddenBanner> = Vec::new();
        let hidden_banners_result : Result<Vec<HiddenBanner>, serde_json::Error> = serde_json::from_str(  &self.current_user.hidden_banners.as_ref() );
        match hidden_banners_result {
            Ok( post_val ) => {
                hidden_banners = post_val;
            }
            Err( err ) => {
                error!("view() hidden_banners_result data parse error", err.to_string());
            }
        }



        html! {
            <UIPage
                global_vars={global_vars}
                page_title="Private Settings"
                submenu_tag={"user".to_owned()}
            >
            <h2><i class={"fa-solid fa-user-secret"}></i><Nbsp />{"Private Settings"}</h2>
            <div class={"alert alert-success text-center"}>
                {"The data in the section is strictly between you and Savaged.us. We won't share anything here with anyone else."}
            </div>
            <input
                type={"email"}
                value=""
                style="height: 0; width: 0; padding: 0; margin: 0;position: absolute; top: -100px;left: -100px;"
                tabindex="-1"
            />
            <input
                name="password"
                type={"password"}
                value=""
                style="height: 0; width: 0; padding: 0; margin: 0;position: absolute; top: -100px;left: -100px;"
                tabindex="-1"
            />
            <input
                name="repeat-password"
                type={"password"}
                value=""
                style="height: 0; width: 0; padding: 0; margin: 0;position: absolute; top: -100px;left: -100px;"
                tabindex="-1"
            />
            <div class={"row"}>
                <div class="col-xs-12 col-md-6">
                    <fieldset class={"fieldset"}>
                        <legend>{"Your Information"}</legend>
                        <InputText
                            label="First Name"
                            value={self.first_name.to_owned()}
                            onchange={ctx.link().callback( SettingsPrivateMessage::UpdateFirstName )}
                        />
                        <InputText
                            label="Last Name"
                            value={self.last_name.to_owned()}
                            onchange={ctx.link().callback( SettingsPrivateMessage::UpdateLastName )}
                        />
                        <InputText
                            label="Email Address"
                            value={self.email.to_owned()}
                            input_type="email"
                            placeholder="me@example.com"
                            onchange={ctx.link().callback( SettingsPrivateMessage::UpdateEmail )}
                        >
                            <div class="small-text">
                                {"This is also your login."}
                            </div>
                        </InputText>
                        if !self.update_info_message.is_empty() {
                            <div class={"alert alert-success"}>
                                {self.update_info_message.clone()}
                            </div>
                        }
                        <div class="text-right">
                            <button
                                type="button"
                                class="btn btn-secondary"
                                disabled={your_info_save_disabled}
                                onclick={ctx.link().callback( SettingsPrivateMessage::ResetInfo )}
                            >
                                <i class="fa fa-cancel" /><Nbsp />{"Cancel"}
                            </button>
                            <Nbsp />
                            <button
                                type="button"
                                class="btn btn-primary"
                                disabled={your_info_save_disabled}
                                onclick={ctx.link().callback( SettingsPrivateMessage::SaveInfo )}
                            >
                                <i class="fa fa-floppy-disk" /><Nbsp />{"Save"}
                            </button>
                        </div>
                    </fieldset>
                    <fieldset class={"fieldset"}>
                        <legend>{"Email Notifications"}</legend>
                        <InputCheckbox
                            label="Receive Email Notifications"
                            checked={self.current_user.notify_email}
                            onchange={ctx.link().callback( SettingsPrivateMessage::SetReceiveNotifications )}
                        >
                            <div class="small-text">
                                {"If you'd like to no longer receive standard email notifications (like feedback notifications, etc), uncheck the following setting. You'll still be notified in your"}
                                <Nbsp />
                                <Link<UserRoute> to={UserRoute::Notifications}>{"Notifications"}</Link<UserRoute>>
                                <Nbsp />{"area"}
                            </div>
                        </InputCheckbox>
                    </fieldset>
                </div>
                <div class="col-xs-12 col-md-6">
                    <fieldset class={"fieldset"}>
                        <legend>{"Change Password"}</legend>

                        <InputText
                            placeholder="Type your new password here"
                            value={self.password.to_owned()}
                            input_type={"password"}
                            onchange={ctx.link().callback( SettingsPrivateMessage::UpdatePassword )}
                        />
                        <InputText
                            placeholder="Verify your new password"
                            value={self.verify_password.to_owned()}
                            input_type={"password"}
                            onchange={ctx.link().callback( SettingsPrivateMessage::UpdateVerifyPassword )}
                        />
                        <div class={&self.password_reset_class}>
                            {&self.password_reset_message}
                        </div>
                        if
                        !self.password_reset_class.find("success").is_none()
                        &&
                        self.password_reset_message.find("reset").is_none()

                        {
                            <div class="text-right">
                                <button
                                    type="button"
                                    class="btn btn-secondary"
                                    onclick={ctx.link().callback( SettingsPrivateMessage::ResetPasswords )}
                                >
                                    <i class="fa fa-cancel" /><Nbsp />{"Cancel"}
                                </button>
                                <Nbsp />
                                <button
                                    type="button"
                                    class="btn btn-primary"
                                    onclick={ctx.link().callback( SettingsPrivateMessage::SavePasswords )}
                                >
                                    <i class="fa fa-floppy-disk" /><Nbsp />{"Save"}
                                </button>
                            </div>
                        }
                    </fieldset>
                    <fieldset class={"fieldset"}>
                        <legend>{"App Options"}</legend>
                        <InputCheckbox
                            label="Turn off Advance Limits"
                            checked={self.current_user.turn_off_advance_limits}
                            onchange={ctx.link().callback( SettingsPrivateMessage::SetTurnOffAdvanceOptions )}
                        >
                            <div class="small-text">
                                {"This will both remove the banner on the Character Generator Traits page warning to use the Advances for advancing character skills and attributes and turn off the read-only flag when a character has advanced."}
                            </div>
                        </InputCheckbox>
                    </fieldset>
                    <fieldset class={"fieldset"}>
                        <legend>{"Hidden Banners"}</legend>
                        if hidden_banners.len() > 0 {
                            <ul class={"styleless"}>
                            {hidden_banners.into_iter().map(|hidden_banner| {
                                html!{
                                    <li key={hidden_banner.id}>
                                        <button
                                            type="button"
                                            class={"btn btn-danger btn-xs"}
                                            onclick={ctx.link().callback( move |_| SettingsPrivateMessage::RemoveHiddenBanner( hidden_banner.id ) )}
                                        >
                                            <i class={"fa fa-trash"} />
                                        </button><Nbsp />
                                        { format!("{}!",hidden_banner.label) }
                                    </li>
                                }
                                }).collect::<Html>()
                            }
                            </ul>
                        } else {
                            <div class={"text-center"}>{"You have no hidden banners"}</div>
                        }

                    </fieldset>
                </div>
            </div>
            </UIPage>
        }

    }
}

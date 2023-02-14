use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::components::ui_page::UIPage;
use crate::libs::fetch_api::fetch_api_send_token;
use crate::libs::site_vars::SiteVars;
use gloo_console::error;
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::user::LoginToken;
use serde_json::Error;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::input_text::InputText;
use standard_components::ui::nbsp::Nbsp;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SettingsDevicesProps {
    pub site_vars: SiteVars,
}

pub enum SettingsDevicesMessages {
    UpdateLoginItems(Vec<LoginToken>),
}

pub struct SettingsDevices {}

impl Component for SettingsDevices {
    type Message = SettingsDevicesMessages;
    type Properties = SettingsDevicesProps;

    fn create(ctx: &Context<Self>) -> Self {
        let site_vars = ctx.props().site_vars.clone();

        set_document_title(
            site_vars.site_title.to_owned(),
            "Device Login Tokens".to_owned(),
            site_vars.server_side_renderer,
        );
        SettingsDevices {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: SettingsDevicesMessages) -> bool {
        match msg {
            SettingsDevicesMessages::UpdateLoginItems(login_tokens) => {
                let mut site_vars = ctx.props().site_vars.clone();

                site_vars.current_user.login_tokens = login_tokens.clone();

                ctx.props().site_vars.update_site_vars.emit(site_vars);

                return true;
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let site_vars = ctx.props().site_vars.clone();

        let mut login_tokens = site_vars.current_user.login_tokens.clone();

        login_tokens.sort_by(|a, b| b.last_seen.cmp(&a.last_seen));

        if site_vars.user_loading {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Settings"

                >
                <div class={"text-center"}>
                    <br />
                    {"Loading..."}
                </div>
                </UIPage>
            };
        }

        if site_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Settings"

                >
                <div class={"text-center"}>
                    <br />
                    {"You are not logged in!"}
                </div>
                </UIPage>
            };
        }

        // let open_confirmation_dialog = ctx.props().site_vars.open_confirmation_dialog.clone();
        // let update_site_vars = ctx.props().site_vars.update_site_vars.clone();
        // let site_vars = ctx.props().site_vars.clone();

        let update_login_tokens = ctx
            .link()
            .callback(SettingsDevicesMessages::UpdateLoginItems);

        let mut site_vars = ctx.props().site_vars.clone();
        let site_vars2 = ctx.props().site_vars.clone();

        site_vars.current_sub_menu = "settings-devices".to_owned();
        site_vars.current_menu = "main-user-login".to_owned();
        let open_confirmation_dialog = site_vars.open_confirmation_dialog.clone();
        html! {
            <UIPage
site_vars={site_vars}
                page_title="Devices"
            >
                <h2><i class={"fa-solid fa-computer"}></i><Nbsp />{"Device Login Tokens"}</h2>

                <table class={"edit-table alternating"}>
                    <thead>
                    <tr>
                        <th>
                            {"Friendly Name"}
                        </th>
                        <th class={"min-width no-wrap"}>{"Registered"}</th>
                        <th class={"min-width no-wrap"}>{"Last Seen On"}</th>
                        <th class={"min-width no-wrap"}>{"Last Seen IP"}</th>
                        <th rowspan={2}>
                            <Nbsp />
                        </th>
                    </tr>
                    <tr>
                        <th colspan={4}>{"Browser String"}</th>
                    </tr>
                    </thead>
                    {login_tokens.into_iter().map(move |device| {
                        html! {
                            <SettingsDeviceLineItem
                                site_vars={site_vars2.clone()}
                                // update_site_vars={ctx.props().site_vars.update_site_vars.clone()}
                                open_confirmation_dialog={open_confirmation_dialog.clone()}
                                token={device}
                                update_login_tokens={update_login_tokens.clone()}
                            />
                        }
                    }).collect::<Html>()}

                </table>
            </UIPage>

        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SettingsDeviceLineItemProps {
    pub site_vars: SiteVars,
    // pub update_site_vars: Callback<SiteVars>,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
    pub token: LoginToken,
    pub update_login_tokens: Callback<Vec<LoginToken>>,
}

pub enum SettingsDeviceLineItemMessage {
    DeleteDevice(MouseEvent),
    SaveDeviceName(MouseEvent),
    UpdateFriendlyName(String),
}

pub struct SettingsDeviceLineItem {
    // site_vars: SiteVars,
    friendly_name: String,
}

impl Component for SettingsDeviceLineItem {
    type Message = SettingsDeviceLineItemMessage;
    type Properties = SettingsDeviceLineItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        // let site_vars = ctx.props().site_vars.clone();

        SettingsDeviceLineItem {
            // site_vars: site_vars,
            friendly_name: ctx.props().token.friendly_name.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: SettingsDeviceLineItemMessage) -> bool {
        match msg {
            SettingsDeviceLineItemMessage::DeleteDevice(_e) => {
                let login_token = ctx.props().site_vars.login_token.clone();
                let current_token = ctx.props().token.token.clone();
                let update_login_tokens = ctx.props().update_login_tokens.clone();
                let endpoint =
                    ctx.props().site_vars.api_root.clone() + &"/user/token-remove".to_owned();
                spawn_local(async move {
                    let fetch_result =
                        fetch_api_send_token(endpoint, login_token, current_token, "".to_owned())
                            .await;

                    match fetch_result {
                        Ok(login_tokens_replace) => {
                            // let vec_val_result = login_tokens_replace.into_serde::< Vec<LoginToken> >();
                            let vec_val_result: Result<Vec<LoginToken>, Error> =
                                JsValueSerdeExt::into_serde(&login_tokens_replace);
                            match vec_val_result {
                                Ok(vec_val) => {
                                    // log!( format!("result {:?}", vec_val) );
                                    update_login_tokens.emit(vec_val);
                                }
                                Err(err) => {
                                    let err_string: String = format!("SettingsDeviceLineItemMessage::DeleteDevice Serde Err(): {}", &err);
                                    error!(&err_string);
                                }
                            }
                        }
                        Err(err) => {
                            error!("SettingsDeviceLineItemMessage::DeleteDevice", &err);
                        }
                    }
                });

                return true;
            }

            SettingsDeviceLineItemMessage::SaveDeviceName(_e) => {
                let update_login_tokens = ctx.props().update_login_tokens.clone();
                let login_token = ctx.props().site_vars.login_token.clone();
                let current_token = ctx.props().token.token.clone();
                let friendly_name = self.friendly_name.to_owned();
                let endpoint = ctx.props().site_vars.api_root.clone()
                    + &"/user/token-update-name".to_owned();
                spawn_local(async move {
                    let fetch_result =
                        fetch_api_send_token(endpoint, login_token, current_token, friendly_name)
                            .await;

                    match fetch_result {
                        Ok(login_tokens_replace) => {
                            // let vec_val_result = login_tokens_replace.into_serde::< Vec<LoginToken> >();
                            let vec_val_result: Result<Vec<LoginToken>, Error> =
                                JsValueSerdeExt::into_serde(&login_tokens_replace);
                            match vec_val_result {
                                Ok(vec_val) => {
                                    update_login_tokens.emit(vec_val);
                                }
                                Err(err) => {
                                    let err_string: String = format!("SettingsDeviceLineItemMessage::SaveDeviceName Serde Err(): {}", &err);
                                    error!(&err_string);
                                }
                            }
                        }
                        Err(err) => {
                            error!("SettingsDeviceLineItemMessage::SaveDeviceName", &err);
                        }
                    }
                });

                return true;
            }

            SettingsDeviceLineItemMessage::UpdateFriendlyName(new_value) => {
                self.friendly_name = new_value.to_owned();

                return true;
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &SettingsDeviceLineItemProps) -> bool {
        self.friendly_name = ctx.props().token.friendly_name.clone();
        return true;
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let device = ctx.props().token.clone();

        html! {
            <tbody>
                <tr>
                    <td class={"no-wrap"}>
                    <form>
                        <div class={"flex"}>

                            <div class="flex-grow">

                            if device.logged_out {
                                <div class="text-center">
                                if !self.friendly_name.is_empty() {
                                    <>

                                        <strong>{self.friendly_name.clone()}</strong>
                                        <Nbsp />{"-"}<Nbsp />
                                    </>
                                }
                                    {"This device was manually logged out"}

                                </div>
                            } else {

                                <InputText
                                    label_class={"plain"}
                                    value={self.friendly_name.clone()}
                                    placeholder={"Enter a friendly name"}
                                    onchange={ctx.link().callback(SettingsDeviceLineItemMessage::UpdateFriendlyName)}
                                />

                            }
                            </div>
                            if !device.logged_out {
                            <button
                                type="submit"
                                class={"btn btn-primary"}
                                disabled={self.friendly_name == ctx.props().token.friendly_name}
                                onclick={ctx.link().callback( SettingsDeviceLineItemMessage::SaveDeviceName )}
                            >
                                <i class={"fa fa-save"} />
                            </button>

                            }

                        </div>
                        </form>
                    </td>
                    <td class={"min-width no-wrap"}>{ctx.props().site_vars.current_user.format_datetime( device.registered.clone(), false, false, false)}</td>
                    <td class={"min-width no-wrap"}>{ctx.props().site_vars.current_user.format_datetime( device.last_seen.clone(), false, false, false)}</td>
                    <td class={"min-width no-wrap"}>{device.last_seen_ip}</td>
                    <td rowspan={2}>
                        if device.token != ctx.props().site_vars.login_token {
                            <button
                                class={"btn btn-danger"}
                                onclick={ctx.link().callback( SettingsDeviceLineItemMessage::DeleteDevice )}
                            >
                                <i class={"fa fa-trash"} />
                            </button>
                        }
                    </td>
                </tr>
                <tr>
                    <td colspan={4}>{device.browser}</td>
                </tr>
                // <tr>
                //     <td colspan={4}>{device.token}</td>
                // </tr>
            </tbody>
        }
    }
}

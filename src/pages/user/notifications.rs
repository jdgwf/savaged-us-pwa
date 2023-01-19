use yew::prelude::*;
use standard_components::ui::nbsp::Nbsp;

use crate::components::ui_page::UIPage;
use standard_components::libs::set_document_title::set_document_title;
use standard_components::ui::raw_html::RawHtml;
use standard_components::ui::input_checkbox::InputCheckbox;
use savaged_libs::utils::date_formatting::convert_utc_to_datetime;
// use crate::components::confirmation_dialog::ConfirmationDialogDefinition;

use crate::libs::fetch_api::fetch_api;
use crate::libs::fetch_api::fetch_api_for_id;
use crate::libs::fetch_api::fetch_api_for_id_with_value;
use gloo_console::{ error };
use wasm_bindgen_futures::spawn_local;
use gloo_utils::format::JsValueSerdeExt;
use crate::libs::global_vars::GlobalVars;

// use savaged_libs::utils::success_return::Vec<Notification>;
use savaged_libs::notification::Notification;
use serde_json::Error;

#[derive(Properties, PartialEq)]
pub struct UserNotificationsProps {
    pub global_vars: GlobalVars,
}

pub enum UserNotificationsMessage {
    SetNotifications( Vec<Notification> ),
    DeleteMessage( u32 ),
    SetMessageReadUnread( u32, bool ),
    DeleteBasicAdmin(),
    MarkAllRead(),
}

pub struct UserNotifications {
    notifications: Vec<Notification>,
    loading: bool,
}

fn get_notifications(
    api_root: String,
    login_token: String,
    set_notifications: Callback<Vec<Notification>>,

) {
    spawn_local (
        async move {
            let result = fetch_api(
                (api_root + "/notifications/get").to_owned(),
                "".to_owned(),
                login_token,
            ).await;

            match result {
                Ok( value ) => {
                    // let vec_val_result = value.into_serde::< Vec<Notification> >();
                    let vec_val_result: Result<Vec<Notification>, Error> = JsValueSerdeExt::into_serde(&value);
                    match vec_val_result {
                        Ok( vec_val ) => {
                            set_notifications.emit( vec_val.clone() );
                        }
                        Err( err ) => {
                            let err_string: String = format!("get_notifications Serde Err(): {}", &err);
                            set_notifications.emit( Vec::new() );
                            error!( &err_string  );
                        }
                    }

                }
                Err( err ) => {
                    set_notifications.emit( Vec::new() );
                    error!("get_notifications Err()", &err );
                }
            }
        }
    );
}

fn mark_all_read(
    api_root: String,
    login_token: String,
    set_notifications: Callback<Vec<Notification>>,
) {
    let api_root = api_root.clone();
    let login_token = login_token.clone();
    let set_notifications = set_notifications.clone();
    spawn_local (
        async move {

            let api_root = api_root.clone();
            let login_token = login_token.clone();
            let set_notifications = set_notifications.clone();

            let api_root_notify = api_root.clone();
            let login_token_notify = login_token.clone();
            let set_notifications_notify = set_notifications.clone();

            let result = fetch_api(
                (api_root + "/notifications/set-all-read").to_owned(),
                "".to_owned(),
                login_token,
            ).await;

            match result {
                Ok( value ) => {
                    let success_result: Result<Vec<Notification>, Error> = JsValueSerdeExt::into_serde(&value);
                    // let success_result = value.into_serde::< Vec<Notification> >();

                    match success_result {
                        Ok( _success_value ) => {

                            get_notifications(
                                api_root_notify,
                                login_token_notify,
                                set_notifications_notify,
                            );
                        }
                        Err( err ) => {
                            // let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                            set_notifications.emit( Vec::new() );
                            error!( "mark_all_read data Err()", &err.to_string()  );
                        }
                    }

                }
                Err( err ) => {
                    set_notifications.emit( Vec::new() );
                    error!("mark_all_read Err()", &err );
                }
            }
        }
    );
}

fn delete_basic_admin(
    api_root: String,
    login_token: String,
    set_notifications: Callback<Vec<Notification>>,
) {
    let api_root = api_root.clone();
    let login_token = login_token.clone();
    let set_notifications = set_notifications.clone();
    spawn_local (
        async move {

            let api_root = api_root.clone();
            let login_token = login_token.clone();
            let set_notifications = set_notifications.clone();

            let api_root_notify = api_root.clone();
            let login_token_notify = login_token.clone();
            let set_notifications_notify = set_notifications.clone();

            let result = fetch_api(
                (api_root + "/notifications/delete-basic-admin").to_owned(),
                "".to_owned(),
                login_token,
            ).await;

            match result {
                Ok( value ) => {
                    // let success_result = value.into_serde::< Vec<Notification> >();
                    let success_result: Result<Vec<Notification>, Error> = JsValueSerdeExt::into_serde(&value);
                    match success_result {
                        Ok( _success_value ) => {

                            get_notifications(
                                api_root_notify,
                                login_token_notify,
                                set_notifications_notify,
                            );
                        }
                        Err( err ) => {
                            // let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                            set_notifications.emit( Vec::new() );
                            // log!( &err_string  );
                            error!( "delete_basic_admin data Err()", &err.to_string()  );
                        }
                    }

                }
                Err( err ) => {
                    set_notifications.emit( Vec::new() );
                    // console::error_2("get_data_via_fetch Err()", &err );
                    error!( "delete_basic_admin data Err()", &err  );
                }
            }
        }
    );
}

fn delete_notification(
    api_root: String,
    login_token: String,
    set_notifications: Callback<Vec<Notification>>,
    notification_id: u32,
) {

    let api_root = api_root.clone();
    let login_token = login_token.clone();
    let set_notifications = set_notifications.clone();
    let notification_id = notification_id.clone();

    spawn_local (
        async move {

            let api_root = api_root.clone();
            let login_token = login_token.clone();
            let set_notifications = set_notifications.clone();
            let notification_id = notification_id.clone();

            let api_root_notify = api_root.clone();
            let login_token_notify = login_token.clone();
            let set_notifications_notify = set_notifications.clone();

            let result = fetch_api_for_id(
                (api_root + "/notifications/set-deleted").to_owned(),
                login_token,
                notification_id,
                "notification_id".to_owned(),
            ).await;

            match result {
                Ok( value ) => {
                    // let success_result = value.into_serde::< Vec<Notification> >();
                    let success_result: Result<Vec<Notification>, Error> = JsValueSerdeExt::into_serde(&value);
                    match success_result {
                        Ok( _success_value ) => {

                            get_notifications(
                                api_root_notify,
                                login_token_notify,
                                set_notifications_notify,
                            );
                        }
                        Err( err ) => {
                            // let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                            // set_notifications.emit( Vec::new() );
                            // log!( &err_string  );
                            error!( "delete_notification data Err()", &err.to_string()  );
                        }
                    }

                }
                Err( err ) => {
                    // set_notifications.emit( Vec::new() );
                    // console::error_2("get_data_via_fetch Err()", &err );
                    error!( "delete_notification data Err()", &err  );
                }
            }
        }
    );
}

fn set_read_notification(
    api_root: String,
    login_token: String,
    set_notifications: Callback<Vec<Notification>>,
    notification_id: u32,
    new_value: bool,
) {

    let api_root = api_root.clone();
    let login_token = login_token.clone();
    let set_notifications = set_notifications.clone();
    let notification_id = notification_id.clone();

    // log!(&(format!("set_read_notification new_value: {}", &new_value)) );

    let mut new_value_string = "0".to_owned();
    if new_value {
        new_value_string = "1".to_owned();
    }
    // log!(&(format!("set_read_notification new_value_string: {}", &new_value_string)) );
    spawn_local (
        async move {

            let api_root = api_root.clone();
            let login_token = login_token.clone();
            let set_notifications = set_notifications.clone();
            let notification_id = notification_id.clone();

            let api_root_notify = api_root.clone();
            let login_token_notify = login_token.clone();
            let set_notifications_notify = set_notifications.clone();

            let result = fetch_api_for_id_with_value(
                (api_root + "/notifications/set-read").to_owned(),
                login_token,
                notification_id,
                "notification_id".to_owned(),
                new_value_string,
                "read".to_owned(),
            ).await;

            match result {
                Ok( value ) => {
                    // let success_result = value.into_serde::< Vec<Notification> >();
                    let success_result: Result<Vec<Notification>, Error> = JsValueSerdeExt::into_serde(&value);
                    match success_result {
                        Ok( _success_value ) => {

                            get_notifications(
                                api_root_notify,
                                login_token_notify,
                                set_notifications_notify,
                            );
                        }
                        Err( err ) => {
                            // let err_string: String = format!("get_data_via_fetch Serde Err(): {}", &err);
                            // set_notifications.emit( Vec::new() );
                            // error!( &err_string  );
                            error!( "set_read_notification data Err()", &err.to_string() );
                        }
                    }

                }
                Err( err ) => {
                    // set_notifications.emit( Vec::new() );
                    // error!("get_data_via_fetch Err()", &err );
                    error!( "set_read_notification data Err()", &err  );
                }
            }
        }
    );
}

impl Component for UserNotifications {
    type Message = UserNotificationsMessage;
    type Properties = UserNotificationsProps;

    fn create(
        ctx: &Context<Self>
    ) -> Self {

        let global_vars = ctx.props().global_vars.clone();
        let login_token = global_vars.login_token.to_owned();
        let set_notifications = ctx.link().callback(
            UserNotificationsMessage::SetNotifications
        );
        let api_root = global_vars.api_root.to_owned();

        if !global_vars.server_side_renderer {
            get_notifications(
                api_root,
                login_token,
                set_notifications,
            );

            set_document_title(global_vars.site_title.to_owned(), "Notifications".to_owned(), global_vars.server_side_renderer,);
        }
        UserNotifications {
            notifications: Vec::new(),
            loading: true,
        }
    }

    // fn changed(
    //     &mut self,
    //     ctx: &Context<Self>,
    //     _props: &UserNotificationsProps,
    // ) -> bool {

    //     ctx.props().global_vars = ctx.props().global_vars.clone();
    //     true
    // }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: UserNotificationsMessage,
    ) -> bool {

        match msg {
            UserNotificationsMessage::SetNotifications( new_value ) => {
                // log!("SetNotifications called");
                self.notifications = new_value.clone();
                self.loading = false;
                let mut new_count = 0;
                for msg in &self.notifications {
                    if msg.read < 1 {
                        new_count += 1;
                    }
                }

                // let mut updated_user = ctx.props().global_vars.current_user.clone();
                let mut global_vars = ctx.props().global_vars.clone();
                global_vars.current_user.unread_notifications = new_count;

                // log!("SetNotifications updated_user", updated_user.id, new_count);
                if global_vars.current_user.id > 0 {
                    let update_global_vars = global_vars.update_global_vars.clone();
                    update_global_vars.emit( global_vars.clone() );
                }

                return true;
            }
            UserNotificationsMessage::DeleteMessage( message_id ) => {

                // reload notifications
                let login_token = ctx.props().global_vars.login_token.to_owned();
                let set_notifications = ctx.link().callback(UserNotificationsMessage::SetNotifications);
                let api_root = ctx.props().global_vars.api_root.to_owned();

                delete_notification(
                    api_root,
                    login_token,
                    set_notifications,
                    message_id,
                );

                return true;
            }

            UserNotificationsMessage::SetMessageReadUnread( message_id, read_unread ) => {

                let login_token = ctx.props().global_vars.login_token.to_owned();
                let set_notifications = ctx.link().callback(UserNotificationsMessage::SetNotifications);
                let api_root = ctx.props().global_vars.api_root.to_owned();

                // log!(&(format!("read_unread: {}", &read_unread)) );

                set_read_notification(
                    api_root,
                    login_token,
                    set_notifications,
                    message_id,
                    read_unread,
                );

                return true;
            }

            UserNotificationsMessage::MarkAllRead() => {

                let login_token = ctx.props().global_vars.login_token.to_owned();
                let set_notifications = ctx.link().callback(UserNotificationsMessage::SetNotifications);
                let api_root = ctx.props().global_vars.api_root.to_owned();

                mark_all_read(
                    api_root,
                    login_token,
                    set_notifications,

                );

                return true;
            }
            UserNotificationsMessage::DeleteBasicAdmin() => {

                let login_token = ctx.props().global_vars.login_token.to_owned();
                let set_notifications = ctx.link().callback(UserNotificationsMessage::SetNotifications);
                let api_root = ctx.props().global_vars.api_root.to_owned();

                delete_basic_admin(
                    api_root,
                    login_token,
                    set_notifications,
                );

                return true;
            }

        }
    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let global_vars = ctx.props().global_vars.clone();

        let notifications = self.notifications.clone();

        if global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
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

        if global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars.clone()}
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

        let mut global_vars = ctx.props().global_vars.clone();

        global_vars.current_sub_menu = "settings_notifications".to_owned();

        html! {
        <UIPage
            global_vars={global_vars}
            page_title="Notifications"
            submenu_tag={"user".to_owned()}
        >
                <h2><i class={"fa-solid fa-radio"}></i><Nbsp />{"My Notifications"}</h2>
                <div>

                    <div class="width-50-perc">
                        <button
                            type="button"
                            class={"btn btn-primary"}
                            onclick={ctx.link().callback(move |_| UserNotificationsMessage::MarkAllRead())}
                        >

                            <i class="fa fa-check-square" /><Nbsp />{"Mark all Read"}

                        </button>
                    </div>
                if ctx.props().global_vars.current_user.is_admin {
                    <div class="width-50-perc text-right">
                        <button
                            type="button"
                            class={"btn btn-primary"}
                            onclick={ctx.link().callback(move |_| UserNotificationsMessage::DeleteBasicAdmin())}
                        >
                            <i class={"fa-solid fa-trash"}></i><Nbsp />{"Delete Basic Admin"}
                        </button>
                    </div>
                }
                </div>
                <table class={"edit-table"}>
                    <thead>
                        <tr>
                            <th class={"min-width"}>{"Read"}</th>
                            <th class={"min-width"}>{"Date"}</th>
                            <th colspan={2}>{"Subject"}</th>
                        </tr>
                    </thead>
                    if self.loading {
                        <tbody>
                            <tr>
                                <td colspan={4} class={"text-center"}>
                                    <br />
                                    {"Loading...."}
                                    <br />
                                    <br />
                                </td>
                            </tr>
                        </tbody>
                    } else {
                        if self.notifications.len() == 0 {
                            <tbody>
                                <tr>
                                    <td colspan={4} class={"text-center"}>
                                        <br />
                                        {"You have no notifications"}
                                        <br />
                                        <br />
                                    </td>
                                </tr>
                            </tbody>
                        } else {
                            <>
                            {notifications.into_iter().map(|msg| {
                            html!{
                                <tbody>
                                    <tr>
                                        <th class={"min-width text-center"}>
                                            <InputCheckbox
                                                checked={msg.read > 0}
                                                label_class={"plain"}
                                                onchange={ctx.link().callback(move | checked | UserNotificationsMessage::SetMessageReadUnread(msg.id, checked ))}
                                            />
                                        </th>
                                        <th>{convert_utc_to_datetime(msg.created_on.unwrap(), false)}</th>

                                        <th>{msg.subject}</th>
                                        <th class={"min-width text-center"}>
                                            <button
                                                type="button"
                                                class={"btn btn-danger"}
                                                onclick={ctx.link().callback(move |_| UserNotificationsMessage::DeleteMessage(msg.id))}
                                            >
                                                <i class={"fa-solid fa-trash"}></i>
                                            </button>
                                        </th>
                                    </tr>
                                    <tr>
                                        <td colspan={4}>
                                            <td>
                                                <RawHtml inner_html={msg.message} />
                                            </td>
                                        </td>
                                    </tr>
                                </tbody>
                            }
                            }).collect::<Html>()}
                            </>
                        }
                    }

                </table>
            </UIPage>

        }

    }
}

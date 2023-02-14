use crate::components::{
    alerts::AlertDefinition, confirmation_dialog::ConfirmationDialogDefinition,
};
use savaged_libs::{
    user::User, websocket_message::WebSocketMessage, web_content::WebContent,
};
use web_sys::MouseEvent;
use yew::Callback;
use yew_router::history::AnyHistory;

use super::global_vars::GlobalVars;


#[derive(Clone, PartialEq, Debug)]
pub struct SiteVars{
    pub add_alert: Callback<AlertDefinition>,
    pub api_root: String,
    pub current_menu: String,
    pub current_sub_menu: String,
    pub current_user: User,
    pub hide_popup_menus_callback: Callback<MouseEvent>,
    pub login_token: String,
    pub logout_callback: Callback<MouseEvent>,
    pub offline: bool,
    pub open_confirmation_dialog: Callback<ConfirmationDialogDefinition>,
    pub send_websocket: Callback<WebSocketMessage>,
    pub server_root: String,
    pub server_side_renderer: bool,
    pub server_side_renderer_history: Option<AnyHistory>,
    pub show_mobile_menu: bool,
    pub site_title: String,
    pub toggle_mobile_menu_callback: Callback<MouseEvent>,
    pub update_site_vars: Callback<SiteVars>,
    pub update_global_vars: Callback<GlobalVars>,
    pub user_loading: bool,

}


impl Default for SiteVars {
    fn default() -> Self {
        Self {
            add_alert: Callback::noop(),
            api_root: "".to_owned(),
            current_menu: "".to_owned(),
            current_sub_menu: "".to_owned(),
            current_user: User::default(),
            hide_popup_menus_callback: Callback::noop(),
            login_token: "".to_owned(),
            logout_callback: Callback::noop(),
            offline: true,
            open_confirmation_dialog: Callback::noop(),
            send_websocket: Callback::noop(),
            server_root: "".to_owned(),
            server_side_renderer: false,
            server_side_renderer_history: None,
            show_mobile_menu: false,
            site_title: "".to_owned(),
            toggle_mobile_menu_callback: Callback::noop(),
            update_site_vars: Callback::noop(),
            update_global_vars: Callback::noop(),
            user_loading: true,

        }
    }
}

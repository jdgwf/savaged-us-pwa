use savaged_libs::admin_libs::AdminDeletePackage;
use savaged_libs::admin_libs::AdminSavePackage;
use savaged_libs::admin_libs::FetchAdminParameters;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use savaged_libs::user::User;
use serde_json;
use web_sys::File;
use web_sys::FormData;
use web_sys::Blob;
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::spawn_local;
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::user::UserUpdateResult;
use gloo_console::error;
use gloo_console::log;
use crate::libs::global_vars::GlobalVars;
use serde_json::Error;

pub async fn fetch_api_delete_game_data_row(
    endpoint: String,
    req: AdminDeletePackage,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    // log!("fetch_api_delete_game_data_row 1", endpoint.clone());
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        serde_json::to_string(&req).unwrap().as_str()
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;
    // log!("fetch_api_delete_game_data_row 2", endpoint.clone());
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // log!("fetch_api_delete_game_data_row 3", endpoint.clone());
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub async fn fetch_api_save_game_data_row(
    endpoint: String,
    req: AdminSavePackage,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    // log!("fetch_api_for_id 1", endpoint.clone());
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        serde_json::to_string(&req).unwrap().as_str()
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;
    // log!("fetch_api_for_id 2", endpoint.clone());
    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // log!("fetch_api_for_id 3", endpoint.clone());
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}
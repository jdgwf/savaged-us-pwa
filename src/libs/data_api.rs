use gloo_console::error;
use gloo_console::log;
use gloo_utils::format::JsValueSerdeExt;
use savaged_libs::player_character::game_data_package::GameDataPackage;
use savaged_libs::save_db_row::SaveDBRow;
use savaged_libs::user::User;
use serde_json::Error;
use serde_json;
use crate::local_storage::index_db_get_saves;
use crate::local_storage::index_db_save_game_data;
use crate::local_storage::index_db_save_saves;
use crate::local_storage::index_db_get_game_data;
use super::site_vars::SiteVars;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn get_game_data(
    login_token: String,
    site_vars: &SiteVars,
    last_updated: Option<String>,
) -> bool {
    if site_vars.server_side_renderer {
        return false;
    }

    // log!("get_game_data called", last_updated.to_owned());

    let endpoint = site_vars.api_root.to_owned() + &"/game-data/get";
    let mut opts = RequestInit::new();
    // log!("get_game_data 1", &endpoint);

    let mut data_body = "".to_owned();

    match last_updated {
        Some( lu ) => {
            data_body = format!(
                "
                {{
                \"login_token\": \"{}\",
                \"last_updated\": \"{}\"

            }}",
                &login_token,
                &lu.to_owned(),
            );
        }
        None => {
            data_body = format!(
                "
                {{
                \"login_token\": \"{}\"

            }}",
                &login_token,
            );
        }
    }

    // log!("get_game_data data_body", data_body.to_owned());
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        data_body.as_ref(),
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(endpoint.as_str(), &opts).unwrap();
    // log!("fetch_api_for_id 2", endpoint.clone());
    let _res = request
        .headers()
        .set("Accept", "application/vnd.github.v3+json");

    let _req = request.headers().set("Content-Type", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    // log!("fetch_api_for_id 3", endpoint.clone());
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json().unwrap()).await;

    match json {
        Ok(value) => {
            let vec_val_result: Result<Option<GameDataPackage>, Error> =
                JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    // log!( format!("get_game_data vec_val.clone().unwrap().books.len {:?}", vec_val.clone().unwrap().books.len()) );
                    // site_vars.update_game_data.emit(vec_val.clone());
                    match vec_val{
                        Some( game_data )=> {
                            let _results = index_db_save_game_data(game_data).await;
                            let gd = index_db_get_game_data().await;
                            site_vars.update_game_data.emit(gd);
                            // log!( format!(" index_db_save_game_data results {:?}", _results))
                            // index_db_
                        }
                        None => {

                        }
                    }


                    return true;
                }
                Err(err) => {
                    let err_string: String = format!("get_game_data Serde Err(): {}", &err);
                    // set_notifications.emit(Vec::new());
                    error!(&err_string);
                    return false;
                }
            }
        }
        Err(err) => {
            // set_notifications.emit(Vec::new());
            error!("get_game_data Err()", &err);
            return false;
        }
    }


}

pub async fn logout_session(
    site_vars: &SiteVars,
) -> bool {

    if site_vars.server_side_renderer {
        return false;
    }
    let endpoint = site_vars.api_root.to_owned() + &"/auth/logout";
    let mut opts = RequestInit::new();
    // log!("get_saves 1", &endpoint);
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!(
            "
            {{
        }}")
        .as_ref(),
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(endpoint.as_str(), &opts).unwrap();
    // log!("fetch_api_for_id 2", endpoint.clone());
    let _res = request
        .headers()
        .set("Accept", "application/vnd.github.v3+json");

    let _req = request.headers().set("Content-Type", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    return true;
}

pub async fn get_saves(
    login_token: String,
    site_vars: &SiteVars,
    last_updated: Option<String>,
) -> bool {

    if site_vars.server_side_renderer {
        return false;
    }


    // log!("get_saves called", last_updated.to_owned());
    let endpoint = site_vars.api_root.to_owned() + &"/saves/get";
    let mut opts = RequestInit::new();
    // log!("get_saves 1", &endpoint);
    opts.method("POST");


    let mut data_body = "".to_owned();

    match last_updated {
        Some( lu ) => {
            data_body = format!(
                "
                {{
                \"login_token\": \"{}\",
                \"last_updated\": \"{}\"

            }}",
                &login_token,
                &lu.to_owned(),
            );
        }
        None => {
            data_body = format!(
                "
                {{
                \"login_token\": \"{}\"

            }}",
                &login_token,
            );
        }
    }

    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        data_body.as_ref(),
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(endpoint.as_str(), &opts).unwrap();
    // log!("fetch_api_for_id 2", endpoint.clone());
    let _res = request
        .headers()
        .set("Accept", "application/vnd.github.v3+json");

    let _req = request.headers().set("Content-Type", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // log!("fetch_api_for_id 3", endpoint.clone());
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json().unwrap()).await;

    match json {
        Ok(value) => {
            let vec_val_result: Result<Vec<SaveDBRow>, Error> =
                JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    // log!("get_saves setting value", vec_val.clone().len());
                    // site_vars.update_saves.emit(vec_val.clone());
                    let _results = index_db_save_saves(site_vars.server_root.clone(), vec_val.clone()).await;

                    let saves_result = index_db_get_saves().await;
                    match saves_result {
                        Some(saves) => {
                            site_vars.update_saves.emit(saves);
                        }
                        None => {}
                    }
                    return true;
                }
                Err(err) => {
                    let err_string: String = format!("get_saves Serde Err(): {}", &err);
                    // set_notifications.emit(Vec::new());
                    error!(&err_string);
                    return false;
                }
            }
        }
        Err(err) => {
            // set_notifications.emit(Vec::new());
            error!("get_saves Err()", &err);
            return false;
        }
    }


}

pub async fn get_current_user(
    login_token: String,
    site_vars: &SiteVars,
) -> bool {

    if site_vars.server_side_renderer {
        return false;
    }
    let endpoint = site_vars.api_root.to_owned() + &"/auth/get-user-data";
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!(
            "
            {{
            \"login_token\": \"{}\"
        }}",
            &login_token,
        )
        .as_ref(),
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(endpoint.as_str(), &opts).unwrap();
    // log!("fetch_api_for_id 2", endpoint.clone());
    let _res = request
        .headers()
        .set("Accept", "application/vnd.github.v3+json");

    let _req = request.headers().set("Content-Type", "application/json");

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // log!("fetch_api_for_id 3", endpoint.clone());
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json().unwrap()).await;

    match json {
        Ok(value) => {
            let vec_val_result: Result<Option<User>, Error> =
                JsValueSerdeExt::into_serde(&value);
            match vec_val_result {
                Ok(vec_val) => {
                    match vec_val {
                        Some( user ) => {
                            // log!("XXXX", user.id);
                            site_vars.update_current_user.emit(user.clone());
                            return true;
                        }
                        None => {
                            return false;
                        }
                    }

                }
                Err(err) => {
                    let err_string: String = format!("get_current_user Serde Err(): {}", &err);
                    // set_notifications.emit(Vec::new());
                    error!(&err_string);
                    return false;
                }
            }
        }
        Err(err) => {
            // set_notifications.emit(Vec::new());
            error!("get_current_user Err()", &err);
            return false;
        }
    }


}
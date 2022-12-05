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

pub async fn fetch_api(
    endpoint: String,
    api_key: String,
    login_token: String,
) -> Result<JsValue, JsValue> {

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!("{{\"api_key\": \"{}\", \"login_token\": \"{}\"}}", &api_key, &login_token).as_ref()
    ) ));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub async fn update_current_user(
    api_root: String,
    api_key: String,
    login_token: String,
    current_user: User,
    new_password: String,
    repeat_password: String,
    remove_image: bool,
) -> Result<JsValue, JsValue> {
    let endpoint = api_root + &"/auth/update-settings".to_owned();

    let mut opts = RequestInit::new();
    opts.method("POST");

    let mut current_user_string = "".to_owned();

    let current_user_convert = &serde_json::to_string( &current_user );
    match current_user_convert {
        Ok( value ) => {
            current_user_string = value.to_owned();
        }
        Err( _ ) => {
            return Err(JsValue::default());
        }
    }

    let post_val_convert : Result<serde_json::Value, serde_json::Error> = serde_json::from_str(  &current_user_string.as_ref() );
    match post_val_convert {
        Ok( mut post_val ) => {

            post_val["api_key"] = serde_json::Value::String(api_key.to_owned());
            post_val["login_token"] = serde_json::Value::String(login_token.to_owned());
            post_val["password"] = serde_json::Value::String(new_password.to_owned());
            post_val["repeat_password"] = serde_json::Value::String(repeat_password.to_owned());
            post_val["remove_image"] = serde_json::Value::Bool(remove_image);
            post_val["current_user"] = serde_json::Value::String( current_user_string.clone() );

            let post_val_string = post_val.to_string();

            let post_value = &wasm_bindgen::JsValue::from_str(
                &post_val_string
            );

            opts.body(
                Some(post_value),
            );
            opts.mode(RequestMode::Cors);
            let request = Request::new_with_str_and_init(&endpoint, &opts)?;

            request
                .headers()
                .set("Accept", "application/vnd.github.v3+json")?;

            request.headers().set("Content-Type", "application/json" )?;

            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();

            // Convert this other `Promise` into a rust `Future`.
            let json = JsFuture::from(resp.json()?).await?;

            // log!("XY", &json);
            return Ok(json);
        }

        Err( _ ) => {
            return Err(JsValue::default());
        }
    }

}

pub async fn fetch_api_for_id(
    endpoint: String,
    login_token: String,
    id: u32,
    remote_id_name: String,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    // log!("fetch_api_for_id 1", endpoint.clone());
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!("
            {{
            \"login_token\": \"{}\",
            \"{}\": \"{}\"
        }}",
            &login_token,
            &remote_id_name, &id
            ).as_ref()
        ) )
    );
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

pub async fn fetch_api_for_id_with_value(
    endpoint: String,
    login_token: String,
    id: u32,
    remote_id_name: String,
    new_value: String,
    new_value_name: String,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!("
            {{
            \"login_token\": \"{}\",
            \"{}\": \"{}\",
            \"{}\": \"{}\"

            }}",
            &login_token,
            &remote_id_name, &id,
            &new_value_name, &new_value,

        ).as_ref()
    ) ));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(
        &endpoint,
         &opts
    )?;

    // log!("fetch_api_for_id_with_value", &endpoint);

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.clone().dyn_into().unwrap();
    // let resp2: Response = resp_value.clone().dyn_into().unwrap();
    // let text = JsFuture::from(resp2.text()?).await?;
    // log!("fetch_api_for_id_with_value esp.text", &text);

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub async fn savaged_login(
    endpoint: String,
    email: String,
    password: String,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!("{{\"email\": \"{}\",\"password\": \"{}\"}}", &email, &password ).as_ref()
    )));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub fn update_user(
    global_vars: GlobalVars,
    update_global_vars: Callback<GlobalVars>,
    updated_user_notification: Callback<String>,
    new_password: String,
    remove_image: bool,
) {
    spawn_local (
        async move {

            let global_vars = global_vars.clone();
            let updated_user_notification = updated_user_notification.clone();

            let result = update_current_user(
                global_vars.api_root.clone(),
                "".to_owned(),
                global_vars.login_token.clone(),
                global_vars.current_user.clone(),
                new_password.to_owned(),
                new_password.to_owned(),
                remove_image,
            ).await;

            match result {
                Ok( value ) => {

                    update_global_vars.emit( global_vars.clone() );
                    // let update_value_result = value.into_serde::<UserUpdateResult>();
                    let update_value_result: Result<UserUpdateResult, Error> = JsValueSerdeExt::into_serde(&value);
                    match update_value_result {
                        Ok( update_value ) => {

                            updated_user_notification.emit( update_value.message );
                        }
                        Err( err ) => {
                            error!("update_current_user Err()", err.to_string() );
                        }
                    }

                }
                Err( err ) => {
                    error!("update_current_user Err()", err );
                }
            }
        }
    );
}

pub async fn upload_user_image(
    api_root: String,
    login_token: String,
    upload_type: String,
    file: Option<File>,
    uploaded_url_callback: Callback<String>,
) -> Result<JsValue,JsValue>  {

    let endpoint = api_root.clone() + &"/auth/set-user-image-data".to_owned();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let form_data_request_result = FormData::new();

    match form_data_request_result {
        Ok( form_data ) => {
            form_data.append_with_str("login_token", login_token.as_str() )?;
            form_data.append_with_str("type", upload_type.as_str() )?;

            match file {
                Some( ref actual_file ) => {

                    let file_name = &actual_file.name().to_owned();
                    let blob_file = Blob::from( actual_file.clone() );
                    form_data.append_with_blob_and_filename(
                        "image",
                        &blob_file,
                        &file_name.as_str(),
                    )?;
                }
                None => {

                }
            }

            opts.body(Some(&form_data) );
        }
        Err(err) => {
            error!("upload_user_image creating form data  e", &err );
        }

    }
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    opts.body( Some(&wasm_bindgen::JsValue::from_str("") ) );

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // log!("upload_user_image sent", &json );
    // let parsed_result = json.into_serde::< FileUploadResults >();
    let parsed_result: Result<FileUploadResults, Error> = JsValueSerdeExt::into_serde(&json);
    match parsed_result {
        Ok( result ) => {
            if result.success {
                // log!("upload_user_image success", &result.image_url );
                uploaded_url_callback.emit( result.image_url );
            }
        }

        Err( _err ) => {

        }
    }

    Ok( json )
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FileUploadResults {
    pub success: bool,
    pub message: String,
    pub image_url: String,
}
// pub async fn clear_user_image(
//     api_root: String,
//     login_token: String,
//     upload_type: String,
//     cleared_url_callback: Callback<String>,
// ) {
//     warn!("clear_user_image called" );
//     let endpoint = api_root.clone() + &"/auth/set-user-image-data".to_owned();
//     log!("clear_user_image upload_type", upload_type.clone() );
//     log!("clear_user_image endpoint", endpoint.clone() );
//     cleared_url_callback.emit("".to_owned());

//     // let mut opts = RequestInit::new();
//     // opts.method("POST");
//     // // // opts.body(Some(&wasm_bindgen::JsValue::from_str(
//     // // format!("{{\"api_key\": \"{}\", \"login_token\": \"{}\", \"{}\": \"{}\"}}", &api_key, &login_token, &var_name, &var_value).as_ref()
//     // // // )));
//     // // opts.
//     // opts.mode(RequestMode::Cors);
//     // let request = Request::new_with_str_and_init(&endpoint, &opts)?;

//     // request
//     //     .headers()
//     //     .set("Accept", "application/vnd.github.v3+json")?;

//     // request.headers().set("Content-Type", "application/json" )?;

//     // let window = web_sys::window().unwrap();
//     // let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

//     // assert!(resp_value.is_instance_of::<Response>());
//     // let resp: Response = resp_value.dyn_into().unwrap();

//     // // Convert this other `Promise` into a rust `Future`.
//     // let json = JsFuture::from(resp.json()?).await?;

//     // Ok(json)
// }

pub async fn fetch_api_with_value(
    endpoint: String,
    api_key: String,
    login_token: String,
    var_value: String,
    var_name: String,
) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!("{{\"api_key\": \"{}\", \"login_token\": \"{}\", \"{}\": \"{}\"}}", &api_key, &login_token, &var_name, &var_value).as_ref()
    ) ));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

pub async fn fetch_api_send_token(
    endpoint: String,
    login_token: String,
    selected_token: String,
    new_value: String, // not used in delete

) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&wasm_bindgen::JsValue::from_str(
        format!("
            {{\"login_token\": \"{}\",
            \"selected_token\": \"{}\",
            \"new_value\": \"{}\"

            }}",
            &login_token,
            &selected_token,
            &new_value,

        ).as_ref()
    ) ));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&endpoint, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    request.headers().set("Content-Type", "application/json" )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

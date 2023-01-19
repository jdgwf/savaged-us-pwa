use chrono::Utc;
use crate::libs::fetch_api::upload_user_image;
use crate::libs::global_vars::GlobalVars;
use wasm_bindgen_futures::spawn_local;
use web_sys::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ImageUploaderProps {
    pub global_vars: GlobalVars,
    pub image_url: String,
    pub label: String,
    pub image_name: String,
    pub upload_url: String,
    pub image_style: String,
    pub on_changed_callback: Callback< String >,
    pub is_default_image: bool,
}

 pub enum ImageUploaderMessage {
    SelectFile(Option<File>),
    ClearFile( MouseEvent ),
}

pub struct ImageUploader {
    image_url: String,
    image_name: String,
    upload_url: String,
    file: Option<File>,
}

impl Component for ImageUploader {
    type Message = ImageUploaderMessage;
    type Properties = ImageUploaderProps;

    fn create(
        ctx: &Context<Self>,
    ) -> Self {
        ImageUploader {
            image_url: ctx.props().image_url.clone(),
            image_name: ctx.props().image_name.clone(),
            upload_url: ctx.props().upload_url.clone(),
            file: None,
        }
    }

    fn changed(
        &mut self,
        ctx: &Context<Self>,
        _props: &ImageUploaderProps,
    ) -> bool {
        self.image_url = ctx.props().image_url.clone();
        self.image_name = ctx.props().image_name.clone();
        self.upload_url = ctx.props().upload_url.clone();
        true
    }

    fn update(
        &mut self,
        ctx: &Context<Self>,
        msg: ImageUploaderMessage,
    ) -> bool {
        match msg {
            ImageUploaderMessage::SelectFile(file) => {
                self.file = file.clone();
                let api_root = ctx.props().global_vars.api_root.clone();
                let login_token = ctx.props().global_vars.login_token.clone();
                let file = file.clone();
                let upload_url_callback = ctx.props().on_changed_callback.clone();
                spawn_local (
                    async move {

                        let _ = upload_user_image(
                            api_root,
                            login_token,
                            "user".to_owned(),
                            file.clone(),
                            upload_url_callback, // for now
                            true, // crop square for user image
                        ).await;
                    }
                );

                false

            },
            ImageUploaderMessage::ClearFile( _e ) => {
                self.file = None;
                self.image_url = "".to_owned();
                ctx.props().on_changed_callback.emit( "".to_owned() );

                true
            },
        }

    }

    fn view(
        &self,
        ctx: &Context<Self>,
    ) -> Html {

        let image_style = ctx.props().image_style.clone();
        let label = ctx.props().label.clone();

        let select_file_callback = ctx.link().callback(ImageUploaderMessage::SelectFile);

        let onchange = move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let file_list = input.files();

            match file_list {
                Some( files )  => {
                    select_file_callback.emit( files.item(0) );
                }
                None => {

                }
            }
        };

        html! {
            <div class={"image-uploader"}>

                <img style={image_style} src={self.image_url.clone()+ &"?v=".to_string() + &Utc::now().timestamp_micros().to_string()} />
                <br />

                <br />

                <div class={"row"}>
                    <div class="col-sm-6 text-center">
                    <label class={"plain"}>{label}<br />
                        <input
                            accept=".jpg, .png, .jpeg, .webp"
                            type={"file"}
                            multiple={false}
                            onchange={onchange}
                        />
                    </label>
                    </div>
                    <div class="col-sm-6 text-center">

                        if !ctx.props().is_default_image && !ctx.props().image_url.is_empty() {
                            <button
                                type="button"
                                class={"btn btn-danger"}
                                onclick={ctx.link().callback( ImageUploaderMessage::ClearFile )}
                            >
                                {"Clear Image"}
                            </button>
                        }

                    </div>
                </div>
                <label class={"plain text-center"}>
                    <div class={"small-text"}>{"You may upload, JPG, PNG, or WEBP images"}</div>
                </label>

            </div>

        }

    }
}

use yew::{function_component, Properties, Html, html};
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use standard_components::libs::set_document_title::set_document_title;

use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct InfoTechProps {
    pub global_vars: GlobalVars,
}
#[function_component(InfoTech)]
pub fn info_tech(
    props: &InfoTechProps,
) -> Html {
    set_document_title(
        props.global_vars.site_title.to_owned(),
        "Tech".to_owned(),
        props.global_vars.no_calls,
    );

    let mut global_vars = props.global_vars.clone();
    global_vars.current_sub_menu = "info-tech".to_owned();

    html! {
    <UIPage
        global_vars={global_vars}
        page_title="Info"
        submenu_tag={"info".to_owned()}
    >
            <h2><i class="fa fa-microchip" /><Nbsp />{"Technologies Used"}</h2>
            <p class="text-center"><strong>{"Version"}<Nbsp />{env!("CARGO_PKG_VERSION")}</strong></p>
            <p>{"This web app uses the following technologies:"}</p>

            <ul>
                <li>
                    <a href={"https://www.rust-lang.org"}>{"Rust"}</a>
                    <Nbsp />{"-"}<Nbsp />
                    {"All logic and compilation to .wasm"}
                </li>
                <li>
                    <a href={"https://webassembly.org"}>{"WebAssembly (WASM)"}</a>
                    <Nbsp />{"-"}<Nbsp />
                    {"WebAssembly - a NextGen platform for the web and beyond."}
                </li>
                <li>
                    <a href={"http://yew.rs"}>{"Yew"}</a>
                    <Nbsp />{"-"}<Nbsp />
                    {"A React-like web framework written in rust which compiles to .wasm"}
                </li>
                <li>
                    <a href={"https://actix.rs/"}>{"Actix"}</a>
                    <Nbsp />{"-"}<Nbsp />
                    {"A \"blazingly fast\" web server written in Rust"}
                </li>
                <li>
                    <a href={"https://trunkrs.dev/"}>{"Trunk"}</a>
                    <Nbsp />{"-"}<Nbsp />
                    {"A building and serving tool which helps make web app development in Yew much easier for the developer."}
                </li>
                <li>
                    <a href={"https://sass-lang.com/"}>{"SCSS"}</a>
                    <Nbsp />{"-"}<Nbsp />
                    {"A CSS precompiler which, for me, makes CSS so much cleaner to edit."}
                </li>
            </ul>
        </UIPage>
    }
}


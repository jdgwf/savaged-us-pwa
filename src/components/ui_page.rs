use web_sys::MouseEvent;
use yew::{function_component, Properties, Html, html, Children, Callback};
use crate::components::menu_main::MenuMain;
use crate::components::menu_mobile::MenuMobile;
use crate::libs::global_vars::GlobalVars;
use standard_components::libs::set_document_title::set_document_title;

// use standard_components::ui::nbsp::Nbsp;
#[derive(Properties, PartialEq)]
pub struct UIPageProps {
    pub global_vars: GlobalVars,
    pub page_title: String,

    #[prop_or_default]
    pub submenu_tag: String,

    #[prop_or_default]
    pub children: Children,

}
#[function_component(UIPage)]
pub fn ui_page(
    props: &UIPageProps,
) -> Html {
    set_document_title(
        props.global_vars.site_title.to_owned(),
        props.page_title.to_owned(),
        props.global_vars.server_side_renderer,
    );

    // let on_click_toggle_mobile_menu = Callback::from( move | _e: MouseEvent | {
    //     // toggle_mobile_menu.emit( true );
    // });

    let mut active_class = "content-pane";

    if props.global_vars.show_mobile_menu {
        active_class = "content-pane show-mobile-menu";
    }

    let mut mobile_active_class = "mobile-menu";

    if props.global_vars.show_mobile_menu {
        mobile_active_class = "mobile-menu show-mobile-menu";
    }

    html! {

        <>
        <header>
            <div class={"width-limit"}>
            <img src="/images/svgd-us.webp" class={"main-logo"} />
            </div>
            <h1>{"Savaged.us v4"}</h1>
            <MenuMain
                global_vars={props.global_vars.clone()}
                submenu_tag={props.submenu_tag.clone()}
            />
            // <div class={"width-limit"}>
            //     // {submenu}
            // </div>
        </header>

        <div class={"position-relative"}>

            <div class={mobile_active_class}>
                <MenuMobile
                    global_vars={props.global_vars.clone()}
                />
            </div>

            <div class={active_class}>



                <div class={"main-content"}>

                    { for props.children.iter() }

                    <footer class="text-center">{("Using server ").to_owned() + &props.global_vars.server_root}</footer>
                </div>
            </div>
        </div>

        </>
    }
}
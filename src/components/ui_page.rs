use crate::components::menu_main::MenuMain;
use crate::components::menu_mobile::MenuMobile;
use crate::libs::site_vars::SiteVars;
use standard_components::libs::set_document_title::set_document_title;
use yew::virtual_dom::VNode;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct UIPageProps {
    // pub site_vars: SiteVars,
    pub site_vars: SiteVars,

    pub page_title: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub modal: Option<VNode>,
}

#[function_component(UIPage)]
pub fn ui_page(props: &UIPageProps) -> Html {
    if !props.site_vars.server_side_renderer {
        set_document_title(
            props.site_vars.site_title.to_owned(),
            props.page_title.to_string(),
            props.site_vars.server_side_renderer,
        );
    }

    let toggle_mobile_menu_callback = props.site_vars.toggle_mobile_menu_callback.clone();

    // let on_click_toggle_mobile_menu = Callback::from( move | _e: MouseEvent | {
    //     // toggle_mobile_menu.emit( true );
    // });

    let mut active_class = "content-pane";

    if props.site_vars.show_mobile_menu {
        active_class = "content-pane show-mobile-menu";
    }

    let mut mobile_active_class = "mobile-menu";

    if props.site_vars.show_mobile_menu {
        mobile_active_class = "mobile-menu show-mobile-menu";
    }

    let mut modal_html = html! {<></>};
    match &props.modal {
        Some(modal) => {
            modal_html = modal.clone();
        }
        None => {}
    }

    html! {

        <>
        {modal_html}
        <header>
            <div class={"width-limit"}>
            <img src="/images/svgd-us.webp" class={"main-logo"} />
            </div>
            <h1>{"Savaged.us v4"}</h1>
            <MenuMain
                toggle_mobile_menu_callback={toggle_mobile_menu_callback}
                site_vars={props.site_vars.clone()}
            />
            // <div class={"width-limit"}>
            //     // {submenu}
            // </div>
        </header>

        <div class={"content-holder"}>

            <div class={mobile_active_class}>
                <MenuMobile
                    site_vars={props.site_vars.clone()}
                />
            </div>

            <div class={active_class}>

                <div class={"main-content"}>

                    { for props.children.iter() }

                </div>
            </div>
        </div>
        <footer class="text-center">
            {("Using server ").to_owned() + &props.site_vars.server_root}<br />
            {("Version ").to_owned() + &props.site_vars.app_version}<br />
        </footer>
        </>
    }
}

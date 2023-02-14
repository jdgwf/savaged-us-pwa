pub mod about;
pub mod contact_us;
pub mod partners;
pub mod privacy_policy;
pub mod tech;
pub mod todos;

use crate::libs::site_vars::SiteVars;
use crate::libs::websocket_set_location;
use crate::pages::error404::Error404;
use about::InfoAbout;
use savaged_libs::web_content::WebContent;
use contact_us::InfoContactUs;
use partners::InfoPartners;
use privacy_policy::InfoPrivacyPolicy;
use tech::InfoTech;
use todos::InfoTodos;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum InfoRoute {
    #[at("/info/about")]
    InfoAbout,

    #[at("/info/tech")]
    InfoTech,

    #[at("/info/to-dos")]
    InfoTodos,

    #[at("/info/privacy-policy")]
    InfoPrivacyPolicy,

    #[at("/info/partners")]
    InfoPartners,

    #[at("/info/contact-us")]
    InfoContactUs,

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: InfoRoute,
    site_vars: SiteVars,
    web_content: Option<WebContent>,
) -> Html {
    // let mut site_vars = site_vars.clone();

    websocket_set_location(
        site_vars.send_websocket.clone(),
        format!("{:?}", routes ),
    );

    match routes {
        InfoRoute::InfoAbout => html! {
            <InfoAbout
                site_vars={site_vars}
            />
        },

        InfoRoute::InfoTech => html! {
            <InfoTech
                site_vars={site_vars}

            />
        },

        InfoRoute::InfoTodos => html! {
            <InfoTodos
                site_vars={site_vars}
            />
        },

        InfoRoute::InfoContactUs => html! {
            <InfoContactUs
                site_vars={site_vars}
            />
        },

        InfoRoute::InfoPrivacyPolicy => html! {
            <InfoPrivacyPolicy
                site_vars={site_vars}
            />
        },

        InfoRoute::InfoPartners => html! {
            <InfoPartners
                site_vars={site_vars}
            />
        },

        InfoRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct InfoRouterProps {
    pub site_vars: SiteVars,
    pub web_content: Option<WebContent>,
}

pub struct InfoRouterMessage {}

pub struct InfoRouter {}

impl Component for InfoRouter {
    type Message = InfoRouterMessage;
    type Properties = InfoRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        InfoRouter {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().site_vars.clone();

        site_vars.current_menu = "main-info".to_owned();
        let web_content = ctx.props().web_content.clone();
        if ctx.props().site_vars.server_side_renderer {
            let history = ctx
                .props()
                .site_vars
                .server_side_renderer_history
                .as_ref()
                .unwrap()
                .clone();

            html! {

                <Router
                    history={history}
                >
                    <div class={"main-content"}>
                        <Switch<InfoRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    site_vars.clone(),
                                    web_content.clone(),
                                )
                            }
                        />
                    </div>
                </Router>
            }
        } else {
            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<InfoRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    site_vars.clone(),
                                    web_content.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }
    }
}

pub mod home;
pub mod article;

use article::HelpArticle;
use savaged_libs::help_article::HelpArticleSection;
use crate::libs::site_vars::SiteVars;
use crate::libs::websocket_set_location;
use crate::pages::error404::Error404;
// use about::HelpAbout;
// use contact_us::HelpContactUs;
// use partners::HelpPartners;
// use privacy_policy::HelpPrivacyPolicy;
// use tech::HelpTech;
// use todos::HelpTodos;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum HelpRoute {

    #[at("/help/registration/:tag")]
    HelpRegistration { tag: String },

    #[at("/help/saves/:tag")]
    HelpSaves { tag: String },

    #[at("/help/characters/:tag")]
    HelpCharacters { tag: String },

    #[at("/help/campaigns/:tag")]
    HelpCampaigns { tag: String },

    #[at("/help/vehicles/:tag")]
    HelpVehicles { tag: String },

    #[at("/404")]
    NotFound,
}

fn content_switch(routes: HelpRoute, site_vars: SiteVars) -> Html {
    // let mut site_vars = site_vars.clone();

    websocket_set_location(
        site_vars.send_websocket.clone(),
        format!("{:?}", routes ),
    );

    match routes {
        HelpRoute::HelpRegistration{ tag } => html! {
            <HelpArticle
                site_vars={site_vars}
                tag={tag}
                section={HelpArticleSection::Registration.to_owned()}
            />
        },
        HelpRoute::HelpSaves{ tag } => html! {
            <HelpArticle
                site_vars={site_vars}
                tag={tag}
                section={HelpArticleSection::Saves.to_owned()}
            />
        },
        HelpRoute::HelpCharacters{ tag } => html! {
            <HelpArticle
                site_vars={site_vars}
                tag={tag}
                section={HelpArticleSection::Characters.to_owned()}
            />
        },
        HelpRoute::HelpCampaigns{ tag } => html! {
            <HelpArticle
                site_vars={site_vars}
                tag={tag}
                section={HelpArticleSection::Campaigns.to_owned()}
            />
        },
        HelpRoute::HelpVehicles{ tag } => html! {
            <HelpArticle
                site_vars={site_vars}
                tag={tag}
                section={HelpArticleSection::Vehicles.to_owned()}
            />
        },

        HelpRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct HelpRouterProps {
    pub site_vars: SiteVars,
}

pub struct HelpRouterMessage {}

pub struct HelpRouter {}

impl Component for HelpRouter {
    type Message = HelpRouterMessage;
    type Properties = HelpRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        HelpRouter {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().site_vars.clone();

        site_vars.current_menu = "main-help".to_owned();

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
                        <Switch<HelpRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    site_vars.clone(),
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
                        <Switch<HelpRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    site_vars.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }
    }
}

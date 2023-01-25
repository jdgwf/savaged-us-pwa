pub mod about;
pub mod contact_us;
pub mod partners;
pub mod privacy_policy;
pub mod tech;
pub mod todos;

use crate::libs::global_vars::GlobalVars;
use crate::libs::websocket_set_location;
use crate::pages::error404::Error404;
use about::InfoAbout;
use contact_us::InfoContactUs;
use partners::InfoPartners;
use privacy_policy::InfoPrivacyPolicy;
use tech::InfoTech;
use todos::InfoTodos;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Routable, PartialEq)]
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

fn content_switch(routes: InfoRoute, global_vars: GlobalVars) -> Html {
    // let mut global_vars = global_vars.clone();

    websocket_set_location(
        global_vars.send_websocket.clone(),
        format!("{:?}", routes ),
    );


    match routes {
        InfoRoute::InfoAbout => html! {
            <InfoAbout
                global_vars={global_vars}
            />
        },

        InfoRoute::InfoTech => html! {
            <InfoTech
                global_vars={global_vars}

            />
        },

        InfoRoute::InfoTodos => html! {
            <InfoTodos
                global_vars={global_vars}
            />
        },

        InfoRoute::InfoContactUs => html! {
            <InfoContactUs
                global_vars={global_vars}
            />
        },

        InfoRoute::InfoPrivacyPolicy => html! {
            <InfoPrivacyPolicy
                global_vars={global_vars}
            />
        },

        InfoRoute::InfoPartners => html! {
            <InfoPartners
                global_vars={global_vars}
            />
        },

        InfoRoute::NotFound => html! {
            <Error404
                global_vars={global_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct InfoRouterProps {
    pub global_vars: GlobalVars,
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
        let mut global_vars = ctx.props().global_vars.clone();

        global_vars.current_menu = "main-info".to_owned();

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx
                .props()
                .global_vars
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
                                    global_vars.clone(),
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
                                    global_vars.clone(),
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }
    }
}

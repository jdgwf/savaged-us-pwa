pub mod edit;
pub mod list;
pub mod view;
use crate::libs::global_vars::GlobalVars;
use crate::pages::error404::Error404;
use crate::pages::user::saves::edit::UserSavesEdit;
use crate::pages::user::saves::list::UserSavesList;
use crate::pages::user::saves::view::UserSavesView;
use yew::prelude::*;
use yew::html;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum UserSavesRoute {

    #[at("/me/saves")]
    List,

    #[at("/me/saves/edit/")]
    RedirectEdit1,
    #[at("/me/saves/edit")]
    RedirectEdit2,

    #[at("/me/saves/view/")]
    RedirectView1,
    #[at("/me/saves/view")]
    RedirectView2,

    #[at("/me/saves/add/:save_type")]
    Add {save_type: String},

    #[at("/me/saves/view/:uuid")]
    View {uuid: String},

    #[at("/me/saves/edit/:uuid")]
    Edit {uuid: String},

    #[at("/404")]
    NotFound,

}

fn content_switch(
    routes: UserSavesRoute,
    global_vars: GlobalVars,
) -> Html {

    let mut global_vars = global_vars.clone();

    if global_vars.current_user.id > 0 {
        global_vars.current_menu = "main-my-stuff".to_owned();
    } else {
        global_vars.current_menu = "".to_owned();
    }



    match routes {

        UserSavesRoute::List => html! {
            <UserSavesList
                global_vars={global_vars}
            />
        },

        UserSavesRoute::Edit {uuid} => html! {
            <UserSavesEdit
                uuid={uuid}
                global_vars={global_vars}
            />
        },

        UserSavesRoute::Add {save_type} => html! {
            <UserSavesEdit
                uuid={""}
                new_save_type={Some(save_type)}
                global_vars={global_vars}
            />
        },

        UserSavesRoute::View {uuid} => html! {
            <UserSavesView
                uuid={uuid}
                global_vars={global_vars}
            />
        },

        UserSavesRoute::RedirectView1 |
        UserSavesRoute::RedirectView2 |
        UserSavesRoute::RedirectEdit1 |
        UserSavesRoute::RedirectEdit2 => html! {
            <Redirect<UserSavesRoute>
                to={UserSavesRoute::List}
            />
        },

        UserSavesRoute::NotFound => html! {
            <Error404
                global_vars={global_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct UserSavesRouterProps {
    pub global_vars: GlobalVars,
}

pub enum UserSavesRouterMessage {
}
pub struct UserSavesRouter {
    // global_vars: GlobalVars,
}

impl Component for UserSavesRouter {
    type Message = UserSavesRouterMessage;
    type Properties = UserSavesRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {

        UserSavesRouter {
            // global_vars: ctx.props().global_vars.clone(),
        }
    }

    // fn update(
    //     &mut self,
    //     _ctx: &Context<Self>,
    //     msg: UserSavesRouterMessage
    // ) -> bool {

    //     match msg {
    //         UserSavesRouterMessage::ChangeFilter( filter_type ) => {
    //             // log!("ChangeFilter", filter_type);
    //             set_local_storage_string( "saves_filter", filter_type);
    //         }

    //         UserSavesRouterMessage::ChangeFolder( folder_name ) => {
    //             // log!("ChangeFolder", folder);
    //             set_local_storage_string( "saves_folder", folder_name);
    //         }
    //     }
    //     true
    // }

    fn view(
        &self,
        ctx: &Context<Self>
    ) -> Html {

        if ctx.props().global_vars.server_side_renderer {
            let history = ctx.props().global_vars.server_side_renderer_history.as_ref().unwrap().clone();
            let global_vars = ctx.props().global_vars.clone();

            html! {

                <Router
                    history={history}
                >
                    <div class={"main-content"}>
                        <Switch<UserSavesRoute>
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
            let global_vars = ctx.props().global_vars.clone();
            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<UserSavesRoute>
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


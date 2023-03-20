pub mod edit;
pub mod list;
pub mod view;
use crate::libs::global_vars::GlobalVars;
use crate::libs::site_vars::SiteVars;
use crate::pages::error404::Error404;
use crate::pages::user::saves::edit::UserSavesEdit;
use crate::pages::user::saves::list::UserSavesList;
use crate::pages::user::saves::view::UserSavesView;
use savaged_libs::player_character::game_data_package::GameDataPackage;
use savaged_libs::save_db_row::SaveDBRow;
use yew::html;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
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
    Add { save_type: String },

    #[at("/me/saves/view/:uuid")]
    View { uuid: String },

    #[at("/me/saves/edit/:uuid")]
    Edit { uuid: String },

    #[at("/404")]
    NotFound,
}

fn content_switch(
    routes: UserSavesRoute,
    mut site_vars: &SiteVars,
    game_data: &Option<GameDataPackage>,
    saves: &Option<Vec<SaveDBRow>>,
) -> Html {
    let mut site_vars = site_vars.clone();

    if site_vars.current_user.id > 0 {
        site_vars.current_menu = "main-my-stuff".to_owned();
    } else {
        site_vars.current_menu = "".to_owned();
    }

    match routes {
        UserSavesRoute::List => html! {
            <UserSavesList
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
            />
        },

        UserSavesRoute::Edit { uuid } => html! {
            <UserSavesEdit
                uuid={uuid}
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
            />
        },

        UserSavesRoute::Add { save_type } => html! {
            <UserSavesEdit
                uuid={""}
                new_save_type={Some(save_type)}
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
            />
        },

        UserSavesRoute::View { uuid } => html! {
            <UserSavesView
                uuid={uuid}
                site_vars={site_vars}
                game_data={game_data.clone()}
                saves={saves.clone()}
            />
        },

        UserSavesRoute::RedirectView1
        | UserSavesRoute::RedirectView2
        | UserSavesRoute::RedirectEdit1
        | UserSavesRoute::RedirectEdit2 => html! {
            <Redirect<UserSavesRoute>
                to={UserSavesRoute::List}
            />
        },

        UserSavesRoute::NotFound => html! {
            <Error404
                site_vars={site_vars}
            />
        },
    }
}

#[derive(Properties, PartialEq)]
pub struct UserSavesRouterProps {
    pub site_vars: SiteVars,
    pub game_data: Option<GameDataPackage>,
    pub saves: Option<Vec<SaveDBRow>>,}

pub enum UserSavesRouterMessage {}
pub struct UserSavesRouter {
    // site_vars: SiteVars,
}

impl Component for UserSavesRouter {
    type Message = UserSavesRouterMessage;
    type Properties = UserSavesRouterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UserSavesRouter {
            // site_vars: ctx.props().site_vars.clone(),
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let site_vars = ctx.props().site_vars.clone();
        let game_data = ctx.props().game_data.clone();
        let saves = ctx.props().saves.clone();

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
                            <Switch<UserSavesRoute>
                                render={
                                    move |routes|
                                    content_switch(
                                        routes,
                                        &site_vars,
                                        &game_data,
                                        &saves,

                                    )
                                }
                            />
                        </div>
                    </Router>
            }
        } else {
            // let site_vars = ctx.props().site_vars.clone();
            // let game_data = ctx.props().game_data.clone();
            // let saves = ctx.props().saves.clone();
            html! {

                <BrowserRouter>
                    <div class={"main-content"}>
                        <Switch<UserSavesRoute>
                            render={
                                move |routes|
                                content_switch(
                                    routes,
                                    &site_vars,
                                    &game_data,
                                    &saves,
                                )
                            }
                        />
                    </div>
                </BrowserRouter>
            }
        }
    }
}

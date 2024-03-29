use crate::components::ui_page::UIPage;
use crate::libs::site_vars::SiteVars;
use crate::pages::error404::Error404;
use savaged_libs::player_character::game_data_package::GameDataPackage;
use savaged_libs::save_db_row::SaveDBRow;
use standard_components::ui::nbsp::Nbsp;
use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserSavesViewProps {

    pub uuid: String,

    pub site_vars: SiteVars,
    pub game_data: Option<GameDataPackage>,
    pub saves: Option<Vec<SaveDBRow>>,
}

pub enum UserSavesViewMessage {
    // ChangeFilter(String),
    // ChangeFolder(String),
}
pub struct UserSavesView {
    // site_vars: SiteVars,
    // save: Option<SaveDBRow>,
}

impl Component for UserSavesView {
    type Message = UserSavesViewMessage;
    type Properties = UserSavesViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UserSavesView {
            // site_vars: ctx.props().site_vars.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: UserSavesViewMessage) -> bool {
        match msg {
            // UserSavesViewMessage::ChangeFilter( filter_type ) => {
            //     // log!("ChangeFilter", filter_type);
            //     set_local_storage_string( "saves_filter", filter_type);
            //     true
            // }

            // UserSavesViewMessage::ChangeFolder( folder_name ) => {
            //     // log!("ChangeFolder", folder);
            //     set_local_storage_string( "saves_folder", folder_name);
            //     true
            // }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut site_vars = ctx.props().site_vars.clone();

        site_vars.current_menu = "main-my-stuff".to_owned();
        site_vars.current_sub_menu = "user-data-saves".to_owned();

        if site_vars.user_loading {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="My Saves"

                >
                    <div class={"text-center"}>
                        <br />
                        {"Loading..."}
                    </div>

                </UIPage>
            };
        }

        if site_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="My Saves"

                >
                    <div class={"text-center"}>
                        <br />
                        {"You are not logged in!"}
                    </div>

                </UIPage>
            };
        }

        let mut save: Option<SaveDBRow> = None;

        match ctx.props().saves.clone() {
            Some(local_saves) => {
                for item in local_saves {
                    if item.uuid == ctx.props().uuid {
                        save = Some(item.clone());
                    }
                }
            }
            None => {}
        }
        match &save {
            Some(save) => {
                let mut form =
                    html! { <div class="text-center">{"TODO: Unhandled Save Type"}</div>};
                match save.save_type.as_ref() {
                    "character" => {
                        form =
                            html! { <div class="text-center">{"TODO: Character View Form"}</div>};
                    }
                    "setting" => {
                        form = html! { <div class="text-center">{"TODO: Setting View Form"}</div>};
                    }
                    "race" => {
                        form = html! { <div class="text-center">{"TODO: Race View Form"}</div>};
                    }
                    "bestiary" => {
                        form = html! { <div class="text-center">{"TODO: Bestiary View Form"}</div>};
                    }
                    "gear" => {
                        form = html! { <div class="text-center">{"TODO: Gear View Form"}</div>};
                    }
                    "weapon" => {
                        form = html! { <div class="text-center">{"TODO: Weapon View Form"}</div>};
                    }
                    "armor" => {
                        form = html! { <div class="text-center">{"TODO: Armor View Form"}</div>};
                    }
                    "hindrances" => {
                        form =
                            html! { <div class="text-center">{"TODO: Hindrance View Form"}</div>};
                    }
                    "edges" => {
                        form = html! { <div class="text-center">{"TODO: Edge View Form"}</div>};
                    }
                    "starship" => {
                        form = html! { <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }
                    "power-armor" => {
                        form = html! { <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }
                    "vehicle" => {
                        form = html! { <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }
                    "walker" => {
                        form = html! { <div class="text-center">{"TODO: SciFi Vehicle 2014 View Form"}</div>};
                    }

                    _ => {
                        html! { <div class="text-center">{format!("Unhandled Save Type: {}", &save.save_type) }</div>};
                    }
                }
                return html! {
                <UIPage
                    site_vars={site_vars}
                    page_title="Viewing Save"

                >
                    <strong>{"Save UUID:"}</strong><Nbsp />{&save.uuid}<br />
                    <strong>{"Save Name:"}</strong><Nbsp />{&save.name}<br />
                    // {"Type:"}<Nbsp />{&save.save_type}<br />
                    <br />
                    {form}
                </UIPage>};
            }
            None => {
                return html! {
                    html! {
                        <Error404
                            site_vars={site_vars}
                        />
                    }
                }
            }
        }
    }
}

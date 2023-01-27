// use crate::components::confirmation_dialog::ConfirmationDialogDefinition;
use crate::components::ui_page::UIPage;
use crate::libs::global_vars::GlobalVars;
use crate::pages::error404::Error404;
use savaged_libs::save_db_row::SaveDBRow;
// use standard_components::libs::local_storage_shortcuts::set_local_storage_string;
use standard_components::ui::nbsp::Nbsp;
use yew::html;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserSavesViewProps {
    // #[prop_or_default]
    // pub set_submenu: Callback<SubmenuData>,
    // pub on_logout_action: Callback<MouseEvent>,
    pub uuid: String,
    pub global_vars: GlobalVars,
}

pub enum UserSavesViewMessage {
    // ChangeFilter(String),
    // ChangeFolder(String),
}
pub struct UserSavesView {
    // global_vars: GlobalVars,
    // save: Option<SaveDBRow>,
}

impl Component for UserSavesView {
    type Message = UserSavesViewMessage;
    type Properties = UserSavesViewProps;

    fn create(_ctx: &Context<Self>) -> Self {
        UserSavesView {
            // global_vars: ctx.props().global_vars.clone(),
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
        let mut global_vars = ctx.props().global_vars.clone();

        global_vars.current_menu = "main-my-stuff".to_owned();
        global_vars.current_sub_menu = "user-data-saves".to_owned();

        if global_vars.user_loading {
            return html! {
                <UIPage
                    global_vars={global_vars}
                    page_title="My Saves"

                >
                    <div class={"text-center"}>
                        <br />
                        {"Loading..."}
                    </div>

                </UIPage>
            };
        }

        if global_vars.current_user.id == 0 {
            return html! {
                <UIPage
                    global_vars={global_vars}
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

        match ctx.props().global_vars.clone().saves {
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
                    global_vars={global_vars.clone()}
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
                            global_vars={global_vars}
                        />
                    }
                }
            }
        }
    }
}

use savaged_libs::{
    player_character::game_data_package::GameDataPackage,
    save_db_row::SaveDBRow, web_content::WebContent,
};
use std::rc::Rc;
use yew::prelude::*;
use super::site_vars::SiteVars;

#[derive(Clone, PartialEq, Debug)]
pub struct GlobalVars {
    pub game_data: Option<GameDataPackage>,
    pub saves: Option<Vec<SaveDBRow>>,
    pub web_content: Option<WebContent>,
    pub site_vars: SiteVars,
}

impl Default for GlobalVars {
    fn default() -> Self {
        Self {
            game_data: None,
            saves: None,
            web_content: None,
            site_vars: SiteVars::default(),
        }
    }
}

impl Reducible for GlobalVars {
    type Action = GlobalVars;

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        // action
        self
    }
}

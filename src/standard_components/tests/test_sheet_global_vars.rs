use serde::{Serialize, Deserialize};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TestSheetGlobalVars {
    pub api_key: String,
    pub test2: String,
    pub check1: bool,
    pub check2: bool,
    pub check3: bool,
    pub to_dos: Vec<PlaceholderToDo>,

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct PlaceholderToDo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

impl Default for TestSheetGlobalVars {
    fn default() -> Self {
        Self {
            api_key: "".to_owned(),
            test2: "".to_owned(),
            check1: false,
            check2: false,
            check3: false,
            to_dos: Vec::new(),
        }
    }
}

impl Reducible for TestSheetGlobalVars {
    type Action = TestSheetGlobalVars;

    fn reduce(
        self: Rc<Self>,
        _action: Self::Action,
    ) -> Rc<Self> {
        // action
        self
    }
}
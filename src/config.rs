use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::value::ValueItem;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Command {
    #[serde()]
    pub dir: Option<String>,
    #[serde(alias = "cmd", alias = "prog")]
    pub program: Option<String>,
    #[serde()]
    pub args: Option<Vec<ValueItem>>,
    #[serde(alias = "vals")]
    pub values: Option<HashMap<String, ValueItem>>,
    #[serde()]
    pub matrix: Option<Vec<Vec<ValueItem>>>,
    #[serde(alias = "keepcp")]
    pub keepchildprocess: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(alias = "vals")]
    pub values: Option<HashMap<String, ValueItem>>,
    #[serde()]
    pub cmds: Option<HashMap<String, Command>>,
}

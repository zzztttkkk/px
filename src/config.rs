use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::value::ValueItem;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Command {
    #[serde()]
    pub name: Option<String>,
    #[serde()]
    pub dir: Option<String>,
    #[serde()]
    pub cmd: Option<String>,
    #[serde()]
    pub args: Option<Vec<ValueItem>>,
    #[serde(alias = "vals")]
    pub values: Option<HashMap<String, ValueItem>>,
    #[serde()]
    pub matrix: Option<Vec<Vec<ValueItem>>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(alias = "vals")]
    pub values: Option<HashMap<String, ValueItem>>,
    #[serde()]
    pub cmds: Option<HashMap<String, Command>>,
}

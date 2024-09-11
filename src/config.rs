use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::value::ValueItem;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Command {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "dir")]
    pub dir: Option<String>,
    #[serde(rename = "cmd")]
    pub cmd: Option<String>,
    #[serde(rename = "args")]
    pub args: Option<Vec<ValueItem>>,
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, ValueItem>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "values")]
    pub values: Option<HashMap<String, ValueItem>>,
    #[serde(rename = "cmds")]
    pub cmds: Option<HashMap<String, Command>>,
}

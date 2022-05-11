use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TaranConfig {
    pub macros: Vec<MacroEntry>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct MacroEntry {
    pub key: String,
    pub description: String,
    pub macro_type: MacroType,
    pub type_string: Option<String>,
    pub commands: Option<Vec<CommandEntry>>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CommandEntry {
    pub fn_name: String,
    pub delay: Option<i64>,
    pub args: Vec<String>
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MacroType {
    Typing,
    Function,
    MetaFunction,
}

pub fn load_config() -> TaranConfig {
    println!("{:?}", std::env::current_dir());

    let data = fs::read_to_string("./config.json").expect("Unable to read config...");

    serde_json::from_str(&data).unwrap()
}
use serde::{Deserialize, Serialize};
use std::fs;
pub mod command_types;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TaranConfig {
    pub macros: Vec<MacroType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MacroType {
    MacroTyping(MacroTyping),
    Command(command_types::MacroCommand),
    MetaCommand(command_types::MacroMetaCommand),
}

pub fn load_config() -> TaranConfig {
    let data = fs::read_to_string("./config.json").expect("Unable to read config...");

    serde_json::from_str(&data).unwrap()
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct MacroTyping {
    pub key: String,
    pub desc: String,
    pub text: String,
}

use serde::{Deserialize, Serialize};
use core::panic;
use std::fs;
pub mod command_types;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TaranConfig {
    pub macros: Vec<MacroType>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MacroType {
    Typing(Typing),
    Command(command_types::MacroCommand),
    MetaCommand(command_types::MacroMetaCommand),
}

impl MacroType {
    pub fn key(&self) -> &String {
        match self {
            MacroType::Typing(val) => &val.key,
            MacroType::Command(val) => val.key(),
            MacroType::MetaCommand(val) => &val.key,
        }
    }
}

pub fn load_config() -> TaranConfig {
    let data = fs::read_to_string("./config.json").expect("Unable to read config...");

    let config = serde_json::from_str(&data).unwrap();

    verify_config(&config);

    return config;
}

fn verify_config(config: &TaranConfig) {
    let macros = &config.macros.clone();
    let mut visited: Vec<String> = Vec::new();

    for m in macros.iter() {
        if visited.contains(m.key()) {
            panic!("Found dupilicate top-level macro key: {}", m.key());
        }

        visited.push(m.key().to_string());
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Typing {
    pub key: String,
    pub desc: Option<String>,
    pub text: String,
}

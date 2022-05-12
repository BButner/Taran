use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MacroCommand {
    LatestDir(CommandLatestDir),
    LatestFile(CommandLatestFile),
    Cmd(Cmd),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct MacroMetaCommand {
    pub key: String,
    pub desc: String,
    pub commands: Vec<MacroCommand>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CommandLatestDir {
    pub key: String,
    pub desc: Option<String>,
    pub implicit: Option<bool>, // By default is true
    pub path: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CommandLatestFile {
    pub key: String,
    pub desc: Option<String>,
    pub implicit: Option<bool>, // By default is true
    pub path: Option<String>,
    pub ext: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Cmd {
    pub key: String,
    pub desc: Option<String>,
    pub command: String,
    pub args: String,
}

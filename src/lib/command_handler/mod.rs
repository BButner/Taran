use std::process::Command;
pub mod typing_handler;

use crate::lib::directories::latest::{
    handle_command_newest_directory, handle_command_newest_file,
};

use super::config::{
    command_types::{Cmd, MacroCommand},
    MacroType,
};

pub fn run_macro(entry: &MacroType) {
    let mut return_value: Option<String> = None;

    match entry {
        MacroType::Typing(cmd) => {
            typing_handler::handle_typing_macro(&cmd);
        }
        MacroType::Command(_) => {
            //TODO Handle Command
        }
        MacroType::MetaCommand(cmd) => {
            for command in &cmd.commands {
                match command {
                    MacroCommand::LatestDir(cmd) => {
                        return_value = handle_command_newest_directory(&return_value, &cmd)
                    }
                    MacroCommand::LatestFile(cmd) => {
                        return_value = handle_command_newest_file(&return_value, &cmd)
                    }
                    MacroCommand::Cmd(cmd) => {
                        return_value = handle_command_cmd(&return_value, &cmd)
                    }
                };

                println!("{:?}", return_value);
            }
        }
    }

    println!("{:?}", return_value);
}

const ARG_DELIMITER: &str = "{arg}";

fn handle_command_cmd(return_value: &Option<String>, cmd: &Cmd) -> Option<String> {
    if !cmd.command.contains(ARG_DELIMITER) && !&cmd.args.contains(ARG_DELIMITER) {
        panic!("Cmd {} does not contain {ARG_DELIMITER}!", cmd.key);
    }

    if return_value.is_none() {
        panic!("Cmd {} did not get a value passed!", cmd.key);
    }

    let command_parsed = cmd.command.replace(ARG_DELIMITER, &return_value.clone().unwrap());
    let args_parsed = cmd.args.replace(ARG_DELIMITER, &return_value.clone().unwrap());

    let command = if cfg!(target_os = "windows") {
        Command::new("powershell").args([&command_parsed, &args_parsed]).spawn()
    } else {
        Command::new(command_parsed).arg(args_parsed).spawn()
    };

    if command.is_err() {
        println!("Error on attempting to launch {}...", cmd.key);
        println!("Error: {}", command.err().unwrap());
    }

    None
}

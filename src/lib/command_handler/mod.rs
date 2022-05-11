use std::any::Any;

use crate::lib::{
    config::MacroEntry,
    directories::latest::{handle_command_newest_directory, handle_command_newest_file},
};

pub fn run_meta_command(entry: &MacroEntry) {
    let mut return_value: Option<String> = None;

    if entry.commands.is_some() {
        for command in entry.commands.clone().unwrap() {
            if command.args.len() < 1 && return_value.is_none() {
                eprintln!("Could not continue as no args were specified, and the previous function did not return a value...");
            }

            match command.fn_name.as_str() {
                "get_newest_directory" => {
                    if !expect_args(&return_value, &command.args) {
                        eprintln!("No args found for {}", command.fn_name);

                        return;
                    }

                    return_value = handle_command_newest_directory(&return_value, &command);
                }
                "get_newest_file_in_dir" => {
                    if !expect_args(&return_value, &command.args) {
                        eprintln!("No args found for {}", command.fn_name);
                        return;
                    }

                    return_value = handle_command_newest_file(&return_value, &command);
                }
                _ => {
                    eprintln!("Could not find function {}", command.fn_name);
                }
            }

            println!("{:?}", return_value);
        }
    }
}

fn expect_args(return_value: &Option<String>, args: &Vec<String>) -> bool {
    return_value.is_some() || args.len() > 0
}

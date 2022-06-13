mod lib;

use crate::lib::command_handler::{run_macro, typing_handler::type_macro};
use chrono::Utc;
use clap::Parser;
use lib::config::load_config;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'k', long = "key", default_value = "")]
    key: String,
    #[clap(short = 'r', long = "raw", default_value = "")]
    raw: String,
}

fn main() {
    let start = Utc::now().time();

    let args = Args::parse();

    if args.raw.len() > 0 {
        type_macro(&args.raw);
    } else {
        if args.key.len() < 1 {
            panic!("No key specified...");
        }

        let config = load_config();

        let cmd = config.macros.iter().find(|m| m.key() == &args.key);

        if cmd.is_none() {
            panic!("Could not find macro with key {}", args.key);
        } else {
            let macro_command = cmd.unwrap();

            run_macro(macro_command);
        }
    }

    let end = Utc::now().time();
    println!("Execution Time: {}ms", (end - start).num_milliseconds());
}

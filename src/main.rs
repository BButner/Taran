mod lib;

use chrono::Utc;
use lib::{config::load_config, command_handler::run_meta_command};

fn main() {
    let start = Utc::now().time();
    let config = load_config();

    run_meta_command(&config.macros[0]);
    let end = Utc::now().time();
    println!("Execution Time: {}ms", (end - start).num_milliseconds());
}

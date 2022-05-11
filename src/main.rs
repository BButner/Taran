mod lib;

use lib::{config::load_config, command_handler::run_meta_command};

fn main() {
    let config = load_config();

    run_meta_command(&config.macros[0]);
}

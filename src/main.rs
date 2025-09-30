mod cli;
mod config;
mod display;
mod quotes;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    // load user config (if exists)
    let user_cfg = config::load_user_config(cli.config.clone());

    // merge into a runtime Config
    let runtime = config::make_runtime_config(user_cfg, &cli);

    // render output
    display::render(&runtime, &cli);
}

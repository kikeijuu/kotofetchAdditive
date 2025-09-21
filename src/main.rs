mod cli;
mod config;
mod display;
mod quotes;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    // load config (user) and built-ins
    let user_cfg = config::load_user_config(cli.config.clone());
    let builtin_quotes = quotes::QuotesCollection::default_with_builtins();

    // merge into a runtime Config
    let runtime = config::make_runtime_config(user_cfg, builtin_quotes, &cli);

    display::render(&runtime, &cli);
}

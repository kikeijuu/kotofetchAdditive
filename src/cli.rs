use clap::{ArgAction, Parser, ValueEnum};
use std::path::PathBuf;

/// kotofetch — show beautiful Japanese quotes in terminal
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Path to config file (TOML). Defaults to ~/.config/kotofetch/config.toml
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Override horizontal padding
    #[arg(long)]
    pub horizontal_padding: Option<usize>,

    /// Override vertical padding
    #[arg(long)]
    pub vertical_padding: Option<usize>,

    /// Override width
    #[arg(long)]
    pub width: Option<usize>,

    /// Show translation (true/false)
    #[arg(long, action = ArgAction::Set)]
    pub show_translation: Option<bool>,

    /// Translation color (hex like #888888 or named)
    #[arg(long)]
    pub translation_color: Option<String>,

    /// Make Japanese text bold
    #[arg(long)]
    pub bold: Option<bool>,

    /// Draw a border around the quote
    #[arg(long)]
    pub border: Option<bool>,

    /// Show quote source
    #[arg(long)]
    pub source: Option<bool>,

    /// Mode to pick quotes from
    #[arg(long, value_enum)]
    pub mode: Option<Mode>,

    /// Choose a specific quote by index (0-based) for reproducible output
    #[arg(long)]
    pub index: Option<usize>,

    /// Seed for random selection (0 = random by time)
    #[arg(long)]
    pub seed: Option<u64>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Mode {
    Proverb,
    Haiku,
    Anime,
    Random,
}

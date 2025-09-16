use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;

#[derive(Deserialize, Debug, Clone)]
pub struct Quote {
    pub japanese: String,
    pub translation: Option<String>,
    pub source: Option<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct QuotesCollection {
    pub proverb: Option<Vec<Quote>>,
    pub haiku: Option<Vec<Quote>>,
    pub anime: Option<Vec<Quote>>,
}

impl QuotesCollection {
    fn parse_quotes(s: &str) -> Option<Vec<Quote>> {
        #[derive(Deserialize)]
        struct Wrapper {
            quote: Vec<Quote>,
        }
        match toml::from_str::<Wrapper>(s) {
            Ok(w) => Some(w.quote),
            Err(e) => {
                eprintln!("Failed to parse quotes TOML: {}", e);
                None
            }
        }
    }

    fn parse_file(path: &PathBuf) -> Option<Vec<Quote>> {
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) => Self::parse_quotes(&content),
                Err(e) => {
                    eprintln!("Failed to read {:?}: {}", path, e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn default_with_builtins() -> Self {
        let mut out = QuotesCollection::default();

        // --- load built-ins (from repo) ---
        let prov_toml = include_str!("../quotes/proverb.toml");
        let h_toml = include_str!("../quotes/haiku.toml");
        let a_toml = include_str!("../quotes/anime.toml");

        out.proverb = Self::parse_quotes(prov_toml);
        out.haiku = Self::parse_quotes(h_toml);
        out.anime = Self::parse_quotes(a_toml);

        // --- try to override from ~/.config/kotofetch/quotes ---
        if let Some(mut cfg_dir) = config_dir() {
            cfg_dir.push("kotofetch/quotes");

            let prov_path = cfg_dir.join("proverb.toml");
            let haiku_path = cfg_dir.join("haiku.toml");
            let anime_path = cfg_dir.join("anime.toml");

            if let Some(v) = Self::parse_file(&prov_path) {
                out.proverb = Some(v);
            }
            if let Some(v) = Self::parse_file(&haiku_path) {
                out.haiku = Some(v);
            }
            if let Some(v) = Self::parse_file(&anime_path) {
                out.anime = Some(v);
            }
        }

        out
    }
}

use serde::Deserialize;

pub static BUILTIN_QUOTES: &[(&str, &str)] = &[
    ("anime.toml", include_str!("../quotes/anime.toml")),
    ("proverb.toml", include_str!("../quotes/proverb.toml")),
    ("haiku.toml", include_str!("../quotes/haiku.toml")),
];

#[derive(Deserialize, Debug, Clone)]
pub struct Quote {
    pub japanese: String,
    pub translation: Option<String>,
    pub romaji: Option<String>,
    pub source: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuotesFile {
    #[serde(rename = "quote")]
    pub quotes: Vec<Quote>,
}

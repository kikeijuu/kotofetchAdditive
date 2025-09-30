use serde::Deserialize;

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
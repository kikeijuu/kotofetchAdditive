use serde::Deserialize;

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
    pub fn default_with_builtins() -> Self {
        // parse embedded TOML files
        let mut out = QuotesCollection::default();

        // note: include_str! paths are relative to this file (src/quotes.rs -> ../quotes/...)
        let prov_toml = include_str!("../quotes/proverbs.toml");
        let h_toml = include_str!("../quotes/haiku.toml");
        let a_toml = include_str!("../quotes/anime.toml");

        let parse = |s: &str| -> Option<Vec<Quote>> {
            // the TOML files are a table array [[quote]]
            #[derive(Deserialize)]
            struct Wrapper {
                quote: Vec<Quote>,
            }
            match toml::from_str::<Wrapper>(s) {
                Ok(w) => Some(w.quote),
                Err(e) => {
                    eprintln!("Failed to parse built-in quotes: {}", e);
                    None
                }
            }
        };

        out.proverb = parse(prov_toml);
        out.haiku = parse(h_toml);
        out.anime = parse(a_toml);

        out
    }
}

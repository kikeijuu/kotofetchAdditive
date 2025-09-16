use crate::config::RuntimeConfig;
use crate::quotes::Quote;
use console::Style;
use rand::prelude::*;
use textwrap::wrap;
use unicode_width::UnicodeWidthStr;

fn simulate_font_size(s: &str, size: &str) -> String {
    match size {
        "small" => s.to_string(),
        "medium" => s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" "),
        "large"  => s.chars().map(|c| c.to_string()).collect::<Vec<_>>().join(" "),
        _ => s.to_string(),
    }
}

fn color_from_hex_like(hex: &str) -> Style {
    let lower = hex.to_lowercase();
    if lower.contains('8') || lower.contains('9') {
        Style::new().dim()
    } else if lower.contains("gray") || lower.contains("grey") {
        Style::new().dim()
    } else {
        Style::new()
    }
}

fn choose_quote(
    collection: &crate::quotes::QuotesCollection,
    mode: &str,
    rng: &mut StdRng,
    index: Option<usize>,
) -> Option<Quote> {
    let vec_opt = match mode {
        "proverb" => &collection.proverb,
        "haiku" => &collection.haiku,
        "anime" => &collection.anime,
        "random" => &None,
        _ => &collection.proverb,
    };

    if mode == "random" {
        // merge all quotes
        let mut all = Vec::new();
        if let Some(v) = &collection.proverb { all.extend(v.clone()); }
        if let Some(v) = &collection.haiku { all.extend(v.clone()); }
        if let Some(v) = &collection.anime { all.extend(v.clone()); }
        if all.is_empty() { return None; }
        if let Some(i) = index { all.get(i).cloned() } else { all.choose(rng).cloned() }
    } else if let Some(vec) = vec_opt {
        if vec.is_empty() { return None; }
        if let Some(i) = index { vec.get(i).cloned() } else { vec.choose(rng).cloned() }
    } else {
        None
    }
}

fn print_boxed(
    text_lines: Vec<String>,
    bold: bool,
    horizontal_padding: usize,
    vertical_padding: usize,
    width: usize, // if 0, auto-calc
    border: bool,
    translation: Option<&str>,
    show_translation: bool,
    translation_color_style: Style,
    source: Option<&str>,
    show_source: bool,
    source_color_style: Style,
) {
    let inner_width = if width == 0 {
        let mut max_width = 0;
        for line in text_lines.iter() {
            for wline in wrap(line, usize::MAX) {
                let line_width = UnicodeWidthStr::width(wline.as_ref());
                if line_width > max_width {
                    max_width = line_width;
                }
            }
        }
        if show_translation {
            if let Some(t) = translation {
                for wline in wrap(t, usize::MAX) {
                    let line_width = UnicodeWidthStr::width(wline.as_ref());
                    if line_width > max_width {
                        max_width = line_width;
                    }
                }
            }
        }
        max_width
    } else {
        width
    };

    let mut jap_style = Style::new().bold();
    if !bold {
        jap_style = Style::new();
    }

    if border {
        println!("┌{}┐", "─".repeat(inner_width + horizontal_padding * 2));
    }

    for _ in 0..vertical_padding {
        if border {
            println!("│{}│", " ".repeat(inner_width + horizontal_padding * 2));
        } else {
            println!("");
        }
    }

    for line in text_lines.iter() {
        let wrapped = wrap(line, inner_width);
        for wline in wrapped.iter() {
            let line_str: &str = wline.as_ref();
            let extra = inner_width.saturating_sub(UnicodeWidthStr::width(line_str));
            let mut content = line_str.to_string();
            content.push_str(&" ".repeat(extra));
            if border {
                println!("│{}{}{}│", " ".repeat(horizontal_padding), jap_style.apply_to(content), " ".repeat(horizontal_padding));
            } else {
                println!("{}{}", " ".repeat(horizontal_padding), jap_style.apply_to(content));
            }
        }
    }

    if show_translation {
        if let Some(t) = translation {
            // blank line
            if border {
                println!(
                    "│{}{}{}│",
                    " ".repeat(horizontal_padding),
                    " ".repeat(inner_width),
                    " ".repeat(horizontal_padding)
                );
            } else {
                println!("{}", " ".repeat(horizontal_padding + inner_width))
            }
            let wrapped = wrap(t, inner_width);
            for wline in wrapped.iter() {
                let line_str: &str = wline.as_ref();
                let extra = inner_width.saturating_sub(UnicodeWidthStr::width(line_str));
                let mut content = line_str.to_string();
                content.push_str(&" ".repeat(extra));
                if border {
                    println!(
                        "│{}{}{}│",
                        " ".repeat(horizontal_padding),
                        translation_color_style.apply_to(content),
                        " ".repeat(horizontal_padding)
                    );
                } else {
                    println!(
                        "{}{}",
                        " ".repeat(horizontal_padding),
                        translation_color_style.apply_to(content)
                    );
                }
            }
        }
    }

    if show_source {
        if let Some(s) = source {
            // spacing line before source
            if border {
                println!(
                    "│{}{}{}│",
                    " ".repeat(horizontal_padding),
                    " ".repeat(inner_width),
                    " ".repeat(horizontal_padding)
                );
            } else {
                println!("{}", " ".repeat(horizontal_padding + inner_width));
            }

            let wrapped = wrap(s, inner_width.saturating_sub(2)); // subtract dash and space
            for (i, wline) in wrapped.iter().enumerate() {
                let line_str = if i == 0 {
                    format!("— {}", wline)
                } else {
                    format!("  {}", wline) // align subsequent lines
                };
                let extra = inner_width.saturating_sub(UnicodeWidthStr::width(line_str.as_str()));
                let mut content = line_str.clone();
                content.push_str(&" ".repeat(extra));

                if border {
                    println!(
                        "│{}{}{}│",
                        " ".repeat(horizontal_padding),
                        source_color_style.apply_to(content),
                        " ".repeat(horizontal_padding)
                    );
                } else {
                    println!(
                        "{}{}",
                        " ".repeat(horizontal_padding),
                        source_color_style.apply_to(content)
                    );
                }
            }
        }
    }

    for _ in 0..vertical_padding {
        if border {
            println!("│{}│", " ".repeat(inner_width + horizontal_padding * 2));
        } else {
            println!("");
        }
    }

    if border {
        println!("└{}┘", "─".repeat(inner_width + horizontal_padding * 2));
    }
}

pub fn render(runtime: &RuntimeConfig, cli: &crate::cli::Cli) {
    // seed
    let seed = if cli.seed.unwrap_or(0) == 0 {
        rand::random::<u64>()
    } else {
        cli.seed.unwrap_or(runtime.seed)
    };
    let mut rng = StdRng::seed_from_u64(seed);

    // choose mode
    let mode = if let Some(m) = &cli.mode {
        match m {
            crate::cli::Mode::Proverb => "proverb",
            crate::cli::Mode::Haiku => "haiku",
            crate::cli::Mode::Anime => "anime",
            crate::cli::Mode::Random => "random",
        }
    } else {
        runtime.mode.as_str()
    };

    let quote = choose_quote(&runtime.quotes, mode, &mut rng, cli.index).unwrap_or(Quote {
        japanese: "(no quote found)".to_string(),
        translation: None,
        source: None,
    });

    let jap = simulate_font_size(&quote.japanese, &runtime.font_size);
    let jap_lines: Vec<String> = jap.lines().map(|s| s.to_string()).collect();
    let tstyle = color_from_hex_like(&runtime.translation_color);
    let show_source = runtime.source && quote.source.is_some();
    let sstyle = Style::new().dim();

    print_boxed(
        jap_lines,
        runtime.bold,
        runtime.horizontal_padding,
        runtime.vertical_padding,
        runtime.width,
        runtime.border,
        quote.translation.as_deref(),
        runtime.show_translation,
        tstyle,
        quote.source.as_deref(),
        show_source,
        sstyle,
    );
}

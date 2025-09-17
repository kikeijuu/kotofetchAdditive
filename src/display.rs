use crate::config::RuntimeConfig;
use crate::quotes::Quote;
use console::Style;
use rand::prelude::*;
use textwrap::wrap;
use unicode_width::UnicodeWidthStr;
use term_size;

fn simulate_font_size(s: &str, size: &str) -> String {
    match size {
        "small" => s.to_string(),
        "medium" => s
            .chars()
            .map(|c| if c == '\n' { "\n".to_string() } else { format!("{c} ") })
            .collect::<String>()
            .trim_end()
            .to_string(),
        "large" => s
            .chars()
            .map(|c| if c == '\n' { "\n".to_string() } else { format!("{c}  ") })
            .collect::<String>()
            .trim_end()
            .to_string(),
        _ => s.to_string(),
    }
}

fn color_from_hex_like(hex: &str) -> Style {
    let lower = hex.to_lowercase();
    if lower.contains('8') || lower.contains('9') || lower.contains("gray") || lower.contains("grey")
    {
        Style::new().dim()
    } else {
        Style::new()
    }
}

/// Center a whole line in the terminal if `centered` is true.
fn pad_to_center(line: &str, box_width: usize, centered: bool) -> String {
    if !centered {
        return line.to_string();
    }
    if let Some((term_width, _)) = term_size::dimensions() {
        if term_width > box_width {
            let pad = (term_width - box_width) / 2;
            return format!("{}{}", " ".repeat(pad), line);
        }
    }
    line.to_string()
}

/// Center text within the inner box width.
fn center_in_box(line: &str, inner_width: usize) -> String {
    let line_width = UnicodeWidthStr::width(line);
    if line_width >= inner_width {
        return line.to_string();
    }
    let total_pad = inner_width - line_width;
    let left = total_pad / 2;
    let right = total_pad - left;
    format!("{}{}{}", " ".repeat(left), line, " ".repeat(right))
}

/// Create an empty line inside the box (used for spacing).
fn blank_line(inner_width: usize, horizontal_padding: usize, border: bool) -> String {
    if border {
        format!(
            "│{}{}{}│",
            " ".repeat(horizontal_padding),
            " ".repeat(inner_width),
            " ".repeat(horizontal_padding)
        )
    } else {
        " ".repeat(horizontal_padding + inner_width)
    }
}

fn print_block(
    lines: &[String],
    style: Style,
    inner_width: usize,
    horizontal_padding: usize,
    border: bool,
    box_width: usize,
    centered: bool,
) {
    for line in lines {
        for wline in wrap(line, inner_width) {
            let content = center_in_box(wline.as_ref(), inner_width);
            let line = if border {
                format!(
                    "│{}{}{}│",
                    " ".repeat(horizontal_padding),
                    style.apply_to(content),
                    " ".repeat(horizontal_padding)
                )
            } else {
                format!("{}{}", " ".repeat(horizontal_padding), style.apply_to(content))
            };
            println!("{}", pad_to_center(&line, box_width, centered));
        }
    }
}

fn print_boxed(
    text_lines: Vec<String>,
    bold: bool,
    horizontal_padding: usize,
    vertical_padding: usize,
    width: usize,
    border: bool,
    translation: Option<&str>,
    show_translation: bool,
    translation_style: Style,
    source: Option<&str>,
    show_source: bool,
    source_style: Style,
    centered: bool,
) {
    // auto-width if none given
    let inner_width = if width == 0 {
        let mut max_width = 0;
        for line in &text_lines {
            for wline in wrap(line, usize::MAX) {
                max_width = max_width.max(UnicodeWidthStr::width(wline.as_ref()));
            }
        }
        if show_translation {
            if let Some(t) = translation {
                for wline in wrap(t, usize::MAX) {
                    max_width = max_width.max(UnicodeWidthStr::width(wline.as_ref()));
                }
            }
        }
        if show_source {
            if let Some(s) = source {
                for wline in wrap(s, usize::MAX) {
                    max_width = max_width.max(UnicodeWidthStr::width(wline.as_ref()));
                }
            }
        }
        max_width
    } else {
        width
    };

    let box_width = inner_width + horizontal_padding * 2 + if border { 2 } else { 0 };
    let jap_style = if bold { Style::new().bold() } else { Style::new() };

    // Top border
    if border {
        let line = format!("┌{}┐", "─".repeat(inner_width + horizontal_padding * 2));
        println!("{}", pad_to_center(&line, box_width, centered));
    }

    // Vertical padding (top)
    for _ in 0..vertical_padding {
        println!("{}", pad_to_center(&blank_line(inner_width, horizontal_padding, border), box_width, centered));
    }

    // Japanese text
    print_block(&text_lines, jap_style, inner_width, horizontal_padding, border, box_width, centered);

    // Translation
    if show_translation {
        if let Some(t) = translation {
            println!("{}", pad_to_center(&blank_line(inner_width, horizontal_padding, border), box_width, centered));
            print_block(&[t.to_string()], translation_style, inner_width, horizontal_padding, border, box_width, centered);
        }
    }

    // Source
    if show_source {
        if let Some(s) = source {
            println!("{}", pad_to_center(&blank_line(inner_width, horizontal_padding, border), box_width, centered));
            let wrapped: Vec<String> = wrap(s, inner_width.saturating_sub(2))
                .into_iter()
                .enumerate()
                .map(|(i, wline)| {
                    if i == 0 {
                        format!("— {}", wline)
                    } else {
                        format!("  {}", wline)
                    }
                })
                .collect();
            print_block(&wrapped, source_style, inner_width, horizontal_padding, border, box_width, centered);
        }
    }

    // Vertical padding (bottom)
    for _ in 0..vertical_padding {
        println!("{}", pad_to_center(&blank_line(inner_width, horizontal_padding, border), box_width, centered));
    }

    // Bottom border
    if border {
        let line = format!("└{}┘", "─".repeat(inner_width + horizontal_padding * 2));
        println!("{}", pad_to_center(&line, box_width, centered));
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

    // collect quotes
    let mut pool = Vec::new();
    if runtime.modes.contains(&"proverb".to_string()) {
        if let Some(v) = &runtime.quotes.proverb {
            pool.extend(v.clone());
        }
    }
    if runtime.modes.contains(&"haiku".to_string()) {
        if let Some(v) = &runtime.quotes.haiku {
            pool.extend(v.clone());
        }
    }
    if runtime.modes.contains(&"anime".to_string()) {
        if let Some(v) = &runtime.quotes.anime {
            pool.extend(v.clone());
        }
    }

    if pool.is_empty() {
        pool.push(Quote {
            japanese: "(no quote found)".to_string(),
            translation: None,
            source: None,
        });
    }

    // pick quote
    let quote = if let Some(i) = cli.index {
        pool.get(i).cloned()
    } else {
        pool.choose(&mut rng).cloned()
    }
    .unwrap();

    // render
    let jap = simulate_font_size(&quote.japanese, &runtime.font_size);
    let jap_lines: Vec<String> = jap.lines().map(|s| s.to_string()).collect();
    let translation_style = color_from_hex_like(&runtime.translation_color);
    let show_source = runtime.source && quote.source.is_some();
    let source_style = Style::new().dim();

    print_boxed(
        jap_lines,
        runtime.bold,
        runtime.horizontal_padding,
        runtime.vertical_padding,
        runtime.width,
        runtime.border,
        quote.translation.as_deref(),
        runtime.show_translation,
        translation_style,
        quote.source.as_deref(),
        show_source,
        source_style,
        runtime.centered,
    );
}

use clap::builder::{Styles, styling::AnsiColor};
use console::Style;

pub fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Cyan.on_default().bold())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Yellow.on_default())
        .placeholder(AnsiColor::Blue.on_default())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Red.on_default())
}

pub fn title() -> Style {
    Style::new().bold().cyan()
}

pub fn label() -> Style {
    Style::new().dim()
}

pub fn path() -> Style {
    Style::new().yellow()
}

pub fn success() -> Style {
    Style::new().green()
}

pub fn error() -> Style {
    Style::new().bold().red()
}

pub fn separator(val: &str) -> String {
    let style = Style::new().dim();

    let sep = style.apply_to("=".repeat(50)).to_string();
    let title = title().apply_to(val).to_string();

    format!("{sep}\n{title}\n{sep}")
}

use clap::ValueEnum;
use ratatui::style::{Color as RatatuiColor, Modifier, Style as RatatuiStyle};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use serde::Serialize;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::highlighting::{Color, FontStyle, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::questions::CodeBlock;

#[derive(Clone, Copy, ValueEnum, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    #[default]
    EightiesDark,
    MochaDark,
    OceanDark,
    OceanLight,
    InspiredGitHub,
    SolarizedDark,
    SolarizedLight,
}

impl From<Theme> for &str {
    fn from(value: Theme) -> Self {
        match value {
            Theme::EightiesDark => "base16-eighties.dark",
            Theme::MochaDark => "base16-mocha.dark",
            Theme::OceanDark => "base16-ocean.dark",
            Theme::OceanLight => "base16-ocean.light",
            Theme::InspiredGitHub => "InspiredGitHub",
            Theme::SolarizedDark => "Solarized (dark)",
            Theme::SolarizedLight => "Solarized (light)",
        }
    }
}

// TODO store the tresult of this function
// in the state so it doesnt get called with every redraw
pub fn highlight_code_block(code_block: &CodeBlock, syntax_theme: Theme) -> Paragraph {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = match ss.find_syntax_by_extension(&code_block.language) {
        Some(syntax) => syntax,
        None => ss.find_syntax_plain_text(),
    };

    let theme = &ts.themes[syntax_theme.into()];
    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut lines: Vec<Line> = Vec::new();

    for line in LinesWithEndings::from(&code_block.code) {
        let Ok(ranges) = highlighter.highlight_line(line, &ss) else {
            return Paragraph::new("Unable to highlight code block");
        };

        let spans = ranges.into_iter().map(range_to_span).collect::<Vec<_>>();
        let line = Line::from(spans);
        lines.push(line);
    }

    let highlighted_paragraph = Paragraph::new(lines);
    let Some(translated_color) = translate_color(theme.settings.background) else {
        return highlighted_paragraph;
    };
    highlighted_paragraph.style(RatatuiStyle::default().bg(translated_color))
}

fn range_to_span((style, content): (Style, &str)) -> Span {
    Span::styled(
        content,
        RatatuiStyle {
            fg: translate_color(Some(style.foreground)),
            bg: None,
            underline_color: translate_color(Some(style.foreground)),
            add_modifier: translate_font_style(style.font_style),
            sub_modifier: Modifier::empty(),
        },
    )
}

fn translate_color(color: Option<Color>) -> Option<RatatuiColor> {
    match color {
        Some(Color { r, g, b, a }) if a > 0 => Some(RatatuiColor::Rgb(r, g, b)),
        _ => None,
    }
}

fn translate_font_style(font_style: FontStyle) -> Modifier {
    let mut modifier = Modifier::empty();
    if font_style.contains(FontStyle::BOLD) {
        modifier.insert(Modifier::BOLD);
    }
    if font_style.contains(FontStyle::ITALIC) {
        modifier.insert(Modifier::ITALIC);
    }
    if font_style.contains(FontStyle::UNDERLINE) {
        modifier.insert(Modifier::UNDERLINED);
    }
    modifier
}

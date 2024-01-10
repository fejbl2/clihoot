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

use crate::questions::{find_syntax, CodeBlock};

#[derive(Clone, Copy, ValueEnum, Serialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Theme {
    #[default]
    Default,
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
            Theme::Default => "base16-eighties.dark",
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

// TODO store the result of this function
// in the state so it doesnt get called with every redraw
#[must_use]
pub fn highlight_code_block(code_block: &CodeBlock, syntax_theme: Theme) -> Paragraph {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let use_bg_color = syntax_theme != Theme::Default;

    let syntax = match find_syntax(&code_block.language, Some(&code_block.code)) {
        Ok(syntax) => syntax,                          // should always happen
        Err(_) => ss.find_syntax_plain_text().clone(), // fallback if something does terribly wrong
    };

    let theme = &ts.themes[syntax_theme.into()];
    let mut highlighter = HighlightLines::new(&syntax, theme);

    let mut lines: Vec<Line> = Vec::new();

    for line in LinesWithEndings::from(&code_block.code) {
        let Ok(ranges) = highlighter.highlight_line(line, &ss) else {
            return Paragraph::new("Unable to highlight code block");
        };

        let spans = ranges
            .into_iter()
            .map(|range| range_to_span(range, use_bg_color))
            .collect::<Vec<_>>();
        let line = Line::from(spans);
        lines.push(line);
    }

    let highlighted_paragraph = Paragraph::new(lines);
    if !use_bg_color {
        return highlighted_paragraph;
    }

    let Some(translated_color) = translate_color(theme.settings.background) else {
        return highlighted_paragraph;
    };
    highlighted_paragraph.style(RatatuiStyle::default().bg(translated_color))
}

fn range_to_span((style, content): (Style, &str), use_bg_color: bool) -> Span {
    let bg = if use_bg_color {
        translate_color(Some(style.background))
    } else {
        None
    };

    Span::styled(
        content,
        RatatuiStyle {
            fg: translate_color(Some(style.foreground)),
            bg,
            underline_color: None,
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

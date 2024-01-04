use clap::ValueEnum;
use ratatui::style::Modifier;
use ratatui::text::{Line, Span, Text};
use serde::Serialize;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::highlighting::{FontStyle, Style};
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
    InspiredGithub,
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
            Theme::InspiredGithub => "InspiredGithub",
            Theme::SolarizedDark => "Solarized (dark)",
            Theme::SolarizedLight => "Solarized (light)",
        }
    }
}

// TODO store the tresult of this function
// in the state so it doesnt get called with every redraw
pub fn highlight_code_block(code_block: &CodeBlock, theme: Theme) -> Text {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let Some(syntax) = ss.find_syntax_by_extension(&code_block.language) else {
        return Text::from("Unable to highlight code block");
    };

    let mut highlighter = HighlightLines::new(syntax, &ts.themes[theme.into()]);

    let mut lines: Vec<Line> = Vec::new();

    for line in LinesWithEndings::from(&code_block.code) {
        let Ok(ranges) = highlighter.highlight_line(line, &ss) else {
            return Text::from("Unable to highlight code block");
        };

        let spans = ranges.into_iter().map(range_to_span).collect::<Vec<_>>();
        let line = Line::from(spans);
        lines.push(line);
    }
    Text::from(lines)
}

fn range_to_span((style, content): (Style, &str)) -> Span {
    Span::styled(
        content,
        ratatui::style::Style {
            fg: translate_color(style.foreground),
            bg: None,
            // bg: translate_color(style.background),
            underline_color: translate_color(style.foreground),
            add_modifier: translate_font_style(style.font_style),
            sub_modifier: Modifier::empty(),
        },
    )
}

fn translate_color(color: syntect::highlighting::Color) -> Option<ratatui::style::Color> {
    match color {
        syntect::highlighting::Color { r, g, b, a } if a > 0 => {
            Some(ratatui::style::Color::Rgb(r, g, b))
        }
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

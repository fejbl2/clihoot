use ratatui::style::Modifier;
use ratatui::text::{Line, Span, Text};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::highlighting::{FontStyle, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::questions::CodeBlock;

pub fn highlight_code_block(code_block: &CodeBlock) -> Text {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // TODO use rust if error
    let syntax = ss.find_syntax_by_extension(&code_block.language).unwrap();
    // TODO maybe change with some option when launching
    let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    let mut lines: Vec<Line> = Vec::new();

    //for line in LinesWithEndings::from(&code_block.code) {
    for line in LinesWithEndings::from(&code_block.code) {
        let ranges = highlighter.highlight_line(line, &ss).unwrap();
        let spans = ranges.into_iter().map(range_to_span).collect::<Vec<_>>();
        let line = Line::from(spans).to_owned();
        lines.push(line);
    }
    Text::from(lines)
}

fn range_to_span((style, content): (Style, &'_ str)) -> Span<'_> {
    Span::styled(
        content,
        ratatui::style::Style {
            fg: translate_color(style.foreground),
            bg: translate_color(style.background),
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

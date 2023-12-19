use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

use crate::questions::CodeBlock;

pub fn highlight_code_block(code_block: &CodeBlock) -> Paragraph<'static> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ss.find_syntax_by_extension(&code_block.language).unwrap();
    let mut highlighter = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    let mut lines: Vec<Line> = Vec::new();

    let string = "fn foo(a: usize) -> bool {}";
    // TODO take text from code block

    //for line in LinesWithEndings::from(&code_block.code) {
    for line in LinesWithEndings::from(string) {
        let ranges = highlighter.highlight_line(line, &ss).unwrap();
        let spans = ranges
            .into_iter()
            .map(|(style, content)| {
                Span::styled(
                    content,
                    ratatui::style::Style {
                        fg: translate_color(style.foreground),
                        bg: translate_color(style.background),
                        underline_color: translate_color(style.foreground),
                        // TODO font modifiers
                        add_modifier: ratatui::style::Modifier::empty(),
                        sub_modifier: ratatui::style::Modifier::empty(),
                    },
                )
            })
            .collect::<Vec<_>>();

        let line = Line::from(spans);
        lines.push(line);
    }
    Paragraph::new(lines)
}

fn translate_color(color: syntect::highlighting::Color) -> Option<ratatui::style::Color> {
    match color {
        syntect::highlighting::Color { r, g, b, a } if a > 0 => {
            Some(ratatui::style::Color::Rgb(r, g, b))
        }
        _ => None,
    }
}

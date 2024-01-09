use crate::terminal::constants::COLORS;

use common::terminal::render::{get_bordered_block, get_empty_block, welcome_results_layout};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState, Paragraph},
};

pub fn render_name_selection(
    frame: &mut Frame,
    name: &str,
    name_used: bool,
    quiz_name: &str,
) -> anyhow::Result<()> {
    let layout = welcome_results_layout(
        frame,
        vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Percentage(80),
        ],
        "Name: ".to_string(),
        " Welcome! ",
        quiz_name,
    );

    let paragraph_name = Paragraph::new(format!("{name}|")).block(get_bordered_block());
    let paragraph_used_name = Paragraph::new("Name already used")
        .fg(Color::Red)
        .block(get_empty_block());

    frame.render_widget(paragraph_name, layout[1]);
    if name_used {
        frame.render_widget(paragraph_used_name, layout[2]);
    }

    Ok(())
}

pub fn render_color_selection(
    frame: &mut Frame,
    _color: Color,
    list_state: &mut ListState,
    quiz_name: &str,
) -> anyhow::Result<()> {
    let layout = welcome_results_layout(
        frame,
        vec![Constraint::Length(1), Constraint::Percentage(90)],
        "Color: ".to_string(),
        " Welcome! ",
        quiz_name,
    );

    let items: Vec<_> = COLORS
        .iter()
        .map(|color| ListItem::new(format!("{color:?}")).style(style::Style::default().fg(*color)))
        .collect();

    let list = List::new(items)
        .block(get_bordered_block())
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, layout[1], list_state);

    Ok(())
}

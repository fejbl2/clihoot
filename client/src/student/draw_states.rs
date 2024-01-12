use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph},
};

use common::{
    constants::COLORS,
    terminal::render::{self, get_bordered_block, get_empty_block, welcome_results_layout},
};

use crate::student::state::{ColorSelectionState, NameSelectionState};

pub fn render_name_selection(frame: &mut Frame, state: &NameSelectionState, quiz_name: &str) {
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

    let paragraph_name = Paragraph::new(format!("{}|", state.name)).block(get_bordered_block());
    let paragraph_used_name = Paragraph::new("Name already used")
        .fg(Color::Red)
        .block(get_empty_block());

    frame.render_widget(paragraph_name, layout[1]);
    if state.name_already_used {
        frame.render_widget(paragraph_used_name, layout[2]);
    }
}

pub fn render_color_selection(frame: &mut Frame, state: &mut ColorSelectionState, quiz_name: &str) {
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

    frame.render_stateful_widget(list, layout[1], &mut state.list_state);
}

pub fn render_help(frame: &mut Frame) {
    let help_text = [
        ("ENTER", "Move to the next state"),
        ("CTRL C", "Exit the game"),
        ("SPACE", "Select an option"),
        ("h", "Show this help"),
        ("↑↓ | ws", "Move up and down"),
        ("←→ | ad", "Move left and right"),
    ];
    render::help(frame, &help_text);
}

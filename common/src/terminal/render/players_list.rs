use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{self},
    widgets::{Block, List, ListItem, ListState, Row, Table, TableState},
};
use uuid::Uuid;

use crate::messages::network::{PlayerData, ShowLeaderboard};

use super::{
    get_bordered_block, get_centered_paragraph, get_highlighted_style, get_inner_block,
    get_outer_block, get_player_style,
};

pub fn list_layout(
    frame: &mut Frame,
    constraints: Vec<Constraint>,
    text: &str,
    title: &str,
    quiz_name: &str,
) -> Rc<[Rect]> {
    let outer_block = get_outer_block(quiz_name);
    let inner_block = get_inner_block(title);
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(content_space);

    let paragraph = get_centered_paragraph(text, Block::default());

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, layout[0]);

    layout
}

pub fn waiting(
    frame: &mut Frame,
    players: &mut [PlayerData],
    list_state: &mut ListState,
    player_uuid: Option<Uuid>,
    quiz_name: &str,
) {
    let constraints = if player_uuid.is_none() {
        vec![
            Constraint::Length(1),
            Constraint::Percentage(90),
            Constraint::Length(1),
        ]
    } else {
        vec![Constraint::Length(1), Constraint::Percentage(90)]
    };
    let layout = list_layout(
        frame,
        constraints.clone(),
        "Players waiting for the game to start:",
        " Welcome! ",
        quiz_name,
    );

    let items: Vec<_> = players
        .iter()
        .map(|player| {
            let mut item = Line::raw(player.nickname.to_string());
            if player.uuid == player_uuid.unwrap_or(Uuid::nil()) {
                item.patch_style(get_player_style());
            }

            ListItem::new(item).fg(player.color)
        })
        .collect();

    let list = List::new(items)
        .block(get_bordered_block())
        .highlight_style(get_highlighted_style())
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, layout[1], list_state);

    if constraints.len() == 3 {
        let paragraph = get_centered_paragraph("Press Enter to start the game!", Block::default());
        frame.render_widget(paragraph, layout[2]);
    }
}

pub fn results(
    frame: &mut Frame,
    results: &ShowLeaderboard,
    table_state: &mut TableState,
    player_uuid: Option<Uuid>,
    quiz_name: &str,
) {
    let mut layout = list_layout(
        frame,
        vec![Constraint::Length(1), Constraint::Percentage(90)],
        "Leaderboard:",
        " Results! ",
        quiz_name,
    );

    if results.was_final_round {
        layout = list_layout(
            frame,
            vec![
                Constraint::Length(1),
                Constraint::Percentage(90),
                Constraint::Length(1),
            ],
            "Final Leaderboard:",
            " Final Results! ",
            quiz_name,
        );

        let paragraph = get_centered_paragraph("Great job everyone!", Block::default());
        frame.render_widget(paragraph, layout[2]);
    }

    let items: Vec<_> = results
        .players
        .iter()
        .map(|(player, score)| {
            let mut name_cell = Line::raw(player.nickname.to_string()).alignment(Alignment::Left);
            let mut score_cell = Line::raw(format!("{score}")).alignment(Alignment::Center);
            if player.uuid == player_uuid.unwrap_or(Uuid::nil()) {
                name_cell.patch_style(get_player_style());
                score_cell.patch_style(get_player_style());
            }
            let row = vec![name_cell, score_cell];

            Row::new(row).style(style::Style::default().fg(player.color))
        })
        .collect();

    let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];
    let cells = vec![
        Line::raw("Player").alignment(Alignment::Left),
        Line::raw("Score").alignment(Alignment::Center),
    ];

    let table = Table::new(items, widths)
        .header(Row::new(cells).underlined())
        .block(get_bordered_block())
        .highlight_style(get_highlighted_style())
        .highlight_symbol(">> ");

    frame.render_stateful_widget(table, layout[1], table_state);
}

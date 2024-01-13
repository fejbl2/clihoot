use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{self},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Clear, Padding, Row, Table,
    },
};

use super::{get_bordered_block, get_centered_paragraph};

fn popup_block<'a>(title: &'a str, bottom_title: &'a str) -> Block<'a> {
    let title = Title::from(title)
        .alignment(Alignment::Center)
        .position(Position::Top);
    let bottom_title = Title::from(bottom_title)
        .alignment(Alignment::Center)
        .position(Position::Bottom);
    let popup_block = get_bordered_block()
        .title(title)
        .title(bottom_title)
        .border_type(BorderType::Thick)
        .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().bg(Color::DarkGray));
    popup_block
}

pub fn help(frame: &mut Frame, help_text: &[(&str, &str)]) {
    let popup_block = popup_block(" Help ", " Press any key to close ");

    let area = centered_rect(frame.size(), 60, 60);

    let rows: Vec<_> = help_text
        .iter()
        .map(|(key, function)| {
            let row = vec![
                Line::styled((*key).to_string(), style::Style::default().bold())
                    .alignment(Alignment::Left),
                Line::raw((*function).to_string()).alignment(Alignment::Left),
            ];

            Row::new(row)
        })
        .collect();

    let widths = [Constraint::Percentage(30), Constraint::Percentage(70)];
    let table = Table::new(rows, widths).block(popup_block);

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

pub fn confirm(frame: &mut Frame, message: &str) {
    let popup_block = popup_block(" Confirm ", " Press y to confirm ");

    let area = centered_rect(frame.size(), 60, 30);

    let paragraph = get_centered_paragraph(message, popup_block);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

// source: https://ratatui.rs/how-to/layout/center-a-rect/
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

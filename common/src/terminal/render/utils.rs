use figlet_rs::FIGfont;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::constants::{MINIMAL_ASCII_HEIGHT, MINIMAL_ASCII_WIDTH};

use super::{get_centered_paragraph, get_outer_block, list_layout};

fn ascii_art(frame: &mut Frame, lines: &[&str], text: &str, quiz_name: &str) {
    let outer_block = get_outer_block(quiz_name);
    let inner = outer_block.inner(frame.size());

    let mut constraints = vec![];
    for _ in lines {
        let constraint = Constraint::Percentage((95 / lines.len()) as u16);
        constraints.push(constraint);
    }
    constraints.push(Constraint::Min(1));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    frame.render_widget(outer_block, frame.size());

    match FIGfont::standard() {
        Ok(standard_font)
            if frame.size().height > MINIMAL_ASCII_HEIGHT
                && frame.size().width > MINIMAL_ASCII_WIDTH =>
        {
            for i in 0..lines.len() {
                let Some(figure) = standard_font.convert(lines[i]) else {
                    // returns none when there is nothing to draw
                    continue;
                };

                let figure = figure.to_string();
                let paragraph = Paragraph::new(figure)
                    .block(Block::default())
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, layout[i]);
            }
        }
        _ => {
            for i in 0..lines.len() {
                let paragraph = get_centered_paragraph(lines[i], Block::default());
                frame.render_widget(paragraph, layout[i]);
            }
        }
    }

    let paragraph = get_centered_paragraph(text, Block::default());
    frame.render_widget(paragraph, layout[lines.len()]);
}

pub fn simple_message(frame: &mut Frame, title: &str, message: &str, quiz_name: &str) {
    list_layout(
        frame,
        vec![Constraint::Percentage(100)],
        message,
        title,
        quiz_name,
    );
}

pub fn welcome(frame: &mut Frame, quiz_name: &str) {
    let lines = ["Welcome", "to", "Clihoot!"];
    ascii_art(
        frame,
        &lines,
        "Press ENTER to start!\nPress h for help",
        quiz_name,
    );
}

pub fn end_game(frame: &mut Frame, quiz_name: &str) {
    let lines = ["Game", "Ended", "Thank You!"];
    ascii_art(frame, &lines, "Press CTRL C to close", quiz_name);
}

pub fn error(frame: &mut Frame, message: &str, quiz_name: &str) {
    simple_message(frame, "Error", message, quiz_name);
}

pub fn resize(frame: &mut Frame, quiz_name: &str, height: u16, width: u16) {
    frame.render_widget(Clear, frame.size());
    simple_message(
        frame,
        "Terminal is too small",
        &format!("Please resize your terminal to at least {height}x{width} size"),
        quiz_name,
    );
}

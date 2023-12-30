use crate::terminal::constants::COLORS;

use common::messages::network::PlayerData;
use figlet_rs::FIGfont;
use ratatui::{
    prelude::*,
    widgets::{block::Title, Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};
use std::{rc::Rc, str::FromStr};

fn get_outer_block() -> Block<'static> {
    let title = Title::from("Clihoot: Here will be name");
    let block = Block::default()
        .title(title)
        .title_style(style::Style::default().bold())
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));
    block
}

fn get_inner_block(title: String) -> Block<'static> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title(title)
        .title_style(style::Style::default().bold())
        .title_alignment(Alignment::Center)
        .padding(Padding::new(1, 1, 1, 1));
    block
}

fn get_empty_block() -> Block<'static> {
    let block = Block::default().borders(Borders::NONE);
    block
}

fn get_bordered_block() -> Block<'static> {
    let block = Block::default().borders(Borders::ALL);
    block
}

fn render_ascii_art(frame: &mut Frame, lines: &[&str]) {
    let outer_block = get_outer_block();
    let inner = outer_block.inner(frame.size());

    let mut constraints = vec![];
    for _ in lines {
        let constraint = Constraint::Percentage((100 / lines.len()).try_into().unwrap());
        constraints.push(constraint);
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    frame.render_widget(outer_block, frame.size());

    let standard_font = FIGfont::standard().unwrap();

    for i in 0..lines.len() {
        let figure = standard_font.convert(lines[i]).unwrap();
        let paragraph = Paragraph::new(figure.to_string())
            .block(get_empty_block())
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, layout[i]);
    }
}

fn render_welcome_layout(
    frame: &mut Frame,
    constraints: Vec<Constraint>,
    paragraph_name: String,
) -> Rc<[Rect]> {
    let outer_block = get_outer_block();
    let inner_block = get_inner_block("Welcome!".to_string());
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(content_space);

    let paragraph = Paragraph::new(paragraph_name).block(get_empty_block());

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, layout[0]);

    layout
}

fn render_simple_message(frame: &mut Frame, title: String, message: &str) {
    let outer_block = get_outer_block();
    let inner_block = get_inner_block(title);
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let paragraph = Paragraph::new(message)
        .block(get_empty_block())
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, content_space);
}

pub fn render_welcome(frame: &mut Frame) {
    let lines = ["Welcome", "to", "Clihoot!"];
    render_ascii_art(frame, &lines);
}

pub fn render_name_selection(frame: &mut Frame, name: &str) {
    let layout = render_welcome_layout(
        frame,
        vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Percentage(80),
        ],
        "Name: ".to_string(),
    );

    let paragraph_name = Paragraph::new(format!("{name}|")).block(get_bordered_block());
    let paragraph_used_name = Paragraph::new("Si jebnuty!!!")
        .fg(Color::Red)
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(paragraph_name, layout[1]);
    frame.render_widget(paragraph_used_name, layout[2]);
}

pub fn render_color_selection(frame: &mut Frame, _color: Color, list_state: &mut ListState) {
    let layout = render_welcome_layout(
        frame,
        vec![Constraint::Length(1), Constraint::Percentage(90)],
        "Color: ".to_string(),
    );

    // TOOD constant for this
    let items: Vec<_> = COLORS
        .iter()
        .map(|color| ListItem::new(format!("{color:?}")).style(style::Style::default().fg(*color)))
        .collect();

    let list = List::new(items)
        .block(get_bordered_block())
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, layout[1], list_state);
}

pub fn render_waiting(frame: &mut Frame, players: &mut [PlayerData]) {
    let layout = render_welcome_layout(
        frame,
        vec![Constraint::Length(1), Constraint::Percentage(90)],
        "Waiting for the game to start:".to_string(),
    );

    let items: Vec<_> = players
        .iter()
        .map(|player| {
            ListItem::new(player.nickname.to_string()).style(
                style::Style::default()
                    .fg(Color::from_str(player.color.as_str()).unwrap_or(Color::White)),
            )
        })
        .collect();

    let list = List::new(items).block(get_bordered_block());

    frame.render_widget(list, layout[1]);
}

pub fn render_question(frame: &mut Frame, question: &str, answers: &[String]) {
    let paragraph =
        Paragraph::new(format!("{}\n\n{}", question, answers.join("\n"))).block(get_outer_block());
    frame.render_widget(paragraph, frame.size());
}

pub fn render_question_waiting(frame: &mut Frame) {
    let paragraph = Paragraph::new("Waiting for others to answer...").block(get_outer_block());
    frame.render_widget(paragraph, frame.size());
}

pub fn render_question_answers(frame: &mut Frame, question: &str, results: &[String]) {
    let paragraph = Paragraph::new(format!("{}\n\n{}", question, results.join("\n")))
        .block(Block::default().title("Results").borders(Borders::ALL));
    frame.render_widget(paragraph, frame.size());
}

pub fn render_results(frame: &mut Frame, results: &[String]) {
    let paragraph = Paragraph::new(format!("{}\n\n{}", "Final results", results.join("\n")))
        .block(Block::default().title("Results").borders(Borders::ALL));
    frame.render_widget(paragraph, frame.size());
}

pub fn render_end_game(frame: &mut Frame) {
    let lines = ["Game", "Over", "Thank you!"];
    render_ascii_art(frame, &lines);
}

pub fn render_teacher_welcome(frame: &mut Frame) {
    render_simple_message(
        frame,
        "Welcome!".to_string(),
        "To start the game press ENTER",
    );
}

pub fn render_teacher_lobby(frame: &mut Frame, players: &mut [PlayerData]) {
    render_waiting(frame, players);
}

pub fn render_error(frame: &mut Frame, message: &str) {
    render_simple_message(frame, "Error!".to_string(), message);
}

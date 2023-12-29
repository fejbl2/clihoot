use crate::terminal::constants::COLORS;

use common::messages::network::PlayerData;
use figlet_rs::FIGfont;
use ratatui::{
    prelude::*,
    widgets::{
        block::Title, Block, Borders, List, ListItem, ListState, Padding, Paragraph,
    },
};
use std::str::FromStr;

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

pub fn render_welcome(frame: &mut Frame) {
    let standard_font = FIGfont::standard().unwrap();
    let figure_w1 = standard_font.convert("Welcome").unwrap();
    let figure_w2 = standard_font.convert("to").unwrap();
    let figure_w3 = standard_font.convert("Clihoot!").unwrap();

    let outer_block = get_outer_block();
    let _inner_block = Block::new().borders(Borders::NONE);
    let inner = outer_block.inner(frame.size());

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(inner);

    let paragraph = Paragraph::new(figure_w1.to_string())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);
    let paragraph2 = Paragraph::new(figure_w2.to_string())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);
    let paragraph3 = Paragraph::new(figure_w3.to_string())
        .block(Block::default().borders(Borders::NONE))
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(paragraph, layout[0]);
    frame.render_widget(paragraph2, layout[1]);
    frame.render_widget(paragraph3, layout[2]);
}

pub fn render_name_selection(frame: &mut Frame, name: &str) {
    let outer_block = get_outer_block();
    let inner_block = get_inner_block("Welcome!".to_string());
    let inner = outer_block.inner(frame.size());

    let name_space_block = Block::default().borders(Borders::ALL);
    let name_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Percentage(80),
        ])
        .split(name_space);

    let paragraph = Paragraph::new("Name: ").block(Block::default().borders(Borders::NONE));
    let paragraph_name = Paragraph::new(format!("{name}|")).block(name_space_block.clone());
    let paragraph_used_name = Paragraph::new("Si jebnuty!!!")
        .fg(Color::Red)
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, layout[0]);
    frame.render_widget(paragraph_name, layout[1]);
    frame.render_widget(paragraph_used_name, layout[2]);
}

pub fn render_color_selection(frame: &mut Frame, _color: Color, list_state: &mut ListState) {
    let outer_block = get_outer_block();
    let inner_block = get_inner_block("Welcome!".to_string());
    let inner = outer_block.inner(frame.size());

    let color_space_block = Block::default().borders(Borders::ALL);
    let color_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Percentage(90)])
        .split(color_space);

    let paragraph = Paragraph::new("Color: ").block(Block::default().borders(Borders::NONE));

    // TOOD constant for this
    let items: Vec<_> = COLORS
        .iter()
        .map(|color| ListItem::new(format!("{color:?}")).style(style::Style::default().fg(*color)))
        .collect();

    let list = List::new(items)
        .block(color_space_block)
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">> ");

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, layout[0]);
    frame.render_stateful_widget(list, layout[1], list_state);
}

pub fn render_waiting(frame: &mut Frame, players: &mut [PlayerData]) {
    let outer_block = get_outer_block();
    let inner_block = get_inner_block("Welcome!".to_string());
    let inner = outer_block.inner(frame.size());

    let content_space_block = Block::default().borders(Borders::ALL);
    let content_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Percentage(90)])
        .split(content_space);

    let paragraph = Paragraph::new("Waiting for teacher to start:")
        .block(Block::default().borders(Borders::NONE));

    let items: Vec<_> = players
        .iter()
        .map(|player| {
            ListItem::new(player.nickname.to_string()).style(
                style::Style::default()
                    .fg(Color::from_str(player.color.as_str()).unwrap_or(Color::White)),
            )
        })
        .collect();

    let list = List::new(items).block(content_space_block);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, layout[0]);
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

pub fn render_teacher_welcome(frame: &mut Frame) {
    let paragraph = Paragraph::new("Welcome teacher!")
        .block(Block::default().title("Welcome").borders(Borders::ALL));
    frame.render_widget(paragraph, frame.size());
}

pub fn render_teacher_lobby(frame: &mut Frame) {
    let paragraph = Paragraph::new("Waiting for others to join...")
        .block(Block::default().title("Waiting").borders(Borders::ALL));
    frame.render_widget(paragraph, frame.size());
}

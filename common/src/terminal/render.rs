use std::{rc::Rc, str::FromStr};

use anyhow::anyhow;
use figlet_rs::FIGfont;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{self, Color, Stylize},
    widgets::{block::Title, Block, Borders, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};

use crate::constants::MINIMAL_ASCII_HEIGHT;
use crate::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};

pub fn get_outer_block(name: &str) -> Block<'static> {
    let title = Title::from("Clihoot: ".to_owned() + name);
    let block = Block::default()
        .title(title)
        .title_style(style::Style::default().bold())
        .borders(Borders::ALL)
        .padding(Padding::new(2, 2, 1, 1));
    block
}

pub fn get_inner_block(title: String) -> Block<'static> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title(title)
        .title_style(style::Style::default().bold())
        .title_alignment(Alignment::Center)
        .padding(Padding::new(1, 1, 1, 1));
    block
}

pub fn get_empty_block() -> Block<'static> {
    let block = Block::default().borders(Borders::NONE);
    block
}

pub fn get_bordered_block() -> Block<'static> {
    let block = Block::default().borders(Borders::ALL);
    block
}

pub fn question(
    frame: &mut Frame,
    question: &NextQuestion,
    _players_answered_count: usize,
) -> anyhow::Result<()> {
    let paragraph = Paragraph::new(format!(
        "{}\n\n{}",
        question.text,
        question
            .choices
            .iter()
            .map(|ch| ch.text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    ))
    .block(get_outer_block("Quiz name"));

    frame.render_widget(paragraph, frame.size());

    Ok(())
}

pub fn question_answers(frame: &mut Frame, _question: &QuestionEnded) -> anyhow::Result<()> {
    let paragraph = Paragraph::new(format!("{}\n\n{}", "TODO", "TODO"))
        .block(Block::default().title("Results").borders(Borders::ALL));
    frame.render_widget(paragraph, frame.size());

    Ok(())
}

pub fn error(frame: &mut Frame, message: &str) -> anyhow::Result<()> {
    simple_message(frame, "Error!".to_string(), message)?;

    Ok(())
}

pub fn waiting(
    frame: &mut Frame,
    players: &mut [PlayerData],
    list_state: &mut ListState,
) -> anyhow::Result<()> {
    let layout = welcome_layout(
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

    let list = List::new(items)
        .block(get_bordered_block())
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, layout[1], list_state);

    Ok(())
}

pub fn question_waiting(frame: &mut Frame) -> anyhow::Result<()> {
    let paragraph =
        Paragraph::new("Waiting for others to answer...").block(get_outer_block("Quiz name"));
    frame.render_widget(paragraph, frame.size());

    Ok(())
}

pub fn welcome_layout(
    frame: &mut Frame,
    constraints: Vec<Constraint>,
    paragraph_name: String,
) -> Rc<[Rect]> {
    let outer_block = get_outer_block("Quiz name");
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

pub fn simple_message(frame: &mut Frame, title: String, message: &str) -> anyhow::Result<()> {
    let outer_block = get_outer_block("Quiz name");
    let inner_block = get_inner_block(title);
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let paragraph = Paragraph::new(message)
        .block(get_empty_block())
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, content_space);

    Ok(())
}

pub fn welcome(frame: &mut Frame) -> anyhow::Result<()> {
    let lines = ["Welcome", "to", "Clihoot!"];
    ascii_art(frame, &lines, "Press ENTER to start")?;

    Ok(())
}

pub fn end_game(frame: &mut Frame) -> anyhow::Result<()> {
    let lines = ["Game", "Over", "Thank you!"];
    ascii_art(frame, &lines, "Press ENTER to exit")?;

    Ok(())
}

pub fn results(frame: &mut Frame, results: &ShowLeaderboard) -> anyhow::Result<()> {
    let paragraph = Paragraph::new(format!(
        "{}\n\n{}",
        "Results are:",
        results
            .players
            .iter()
            .map(|(player, points)| format!("{}: {} points", player.nickname, points))
            .collect::<Vec<_>>()
            .join("\n")
    ))
    .block(Block::default().title("Results").borders(Borders::ALL));
    frame.render_widget(paragraph, frame.size());

    Ok(())
}

pub fn ascii_art(frame: &mut Frame, lines: &[&str], text: &str) -> anyhow::Result<()> {
    let outer_block = get_outer_block("Quiz name");
    let inner = outer_block.inner(frame.size());

    let mut constraints = vec![];
    for _ in lines {
        let constraint = Constraint::Percentage((95 / lines.len()).try_into()?);
        constraints.push(constraint);
    }
    constraints.push(Constraint::Min(1));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    frame.render_widget(outer_block, frame.size());

    if frame.size().height > MINIMAL_ASCII_HEIGHT {
        let standard_font = FIGfont::standard().map_err(|_| anyhow!("Couldn't get font"))?;

        for i in 0..lines.len() {
            let figure = standard_font
                .convert(lines[i])
                .ok_or(anyhow!("Couldn't convert text to figure"))?;
            let paragraph = Paragraph::new(figure.to_string())
                .block(get_empty_block())
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, layout[i]);
        }
    } else {
        for i in 0..lines.len() {
            let paragraph = Paragraph::new(lines[i])
                .block(get_empty_block())
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, layout[i]);
        }
    }

    let paragraph = Paragraph::new(text)
        .block(get_empty_block())
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, layout[lines.len()]);

    Ok(())
}

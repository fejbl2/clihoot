use std::{rc::Rc, str::FromStr};

use anyhow::anyhow;
use figlet_rs::FIGfont;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{self, Color, Stylize},
    text::Span,
    widgets::{
        block::Title, Block, BorderType, Borders, List, ListItem, ListState, Padding, Paragraph,
        Row, Table, TableState,
    },
    Frame,
};

use crate::constants::MINIMAL_ASCII_HEIGHT;
use crate::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};
use crate::terminal::highlight::{highlight_code_block, Theme};
use crate::terminal::widgets::choice::{ChoiceGrid, ChoiceSelector, ChoiceSelectorState};

#[must_use]
pub fn get_outer_block(name: &str) -> Block<'static> {
    let title = Title::from("Clihoot: ".to_owned() + name);
    let block = Block::default()
        .title(title)
        .title_style(style::Style::default().bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .padding(Padding::new(2, 2, 1, 1));
    block
}

#[must_use]
pub fn get_inner_block(title: String) -> Block<'static> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title(title)
        .title_style(style::Style::default().bold())
        .title_alignment(Alignment::Center)
        .padding(Padding::new(1, 1, 1, 1));
    block
}

#[must_use]
pub fn get_empty_block() -> Block<'static> {
    let block = Block::default().borders(Borders::NONE);
    block
}

#[must_use]
pub fn get_bordered_block() -> Block<'static> {
    let block = Block::default().borders(Borders::ALL);
    block
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

fn question_layout(
    frame: &mut Frame,
    question: &NextQuestion,
    players_answered_count: usize,
    time_from_start: usize,
    text: &str,
) -> Rc<[Rect]> {
    let outer_block = get_outer_block("Quiz name");
    let inner_block = get_inner_block(
        "Question ".to_string()
            + (question.question_index + 1).to_string().as_str()
            + "/"
            + question.questions_count.to_string().as_str(),
    );
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(content_space);

    let counts_block = get_bordered_block().padding(Padding::new(1, 1, 0, 0));
    let counts_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(counts_block.inner(layout[0]));

    let counts_paragraph_2 = Paragraph::new(format!(
        "Time left: {}",
        (question.show_choices_after + question.time_seconds).saturating_sub(time_from_start)
    ))
    .alignment(Alignment::Left)
    .block(get_empty_block());
    let counts_paragraph_1 = Paragraph::new(format!("Players answered: {players_answered_count}"))
        .alignment(Alignment::Right)
        .block(get_empty_block());

    let paragraph = Paragraph::new(text)
        .bold()
        .block(get_empty_block().padding(Padding::new(1, 1, 1, 1)))
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);

    frame.render_widget(counts_block, layout[0]);
    frame.render_widget(counts_paragraph_1, counts_layout[1]);
    frame.render_widget(counts_paragraph_2, counts_layout[0]);

    frame.render_widget(paragraph, layout[1]);

    layout
}

pub fn question(
    frame: &mut Frame,
    question: &NextQuestion,
    players_answered_count: usize,
    choice_grid: &mut ChoiceGrid,
    choice_selector_state: &mut ChoiceSelectorState,
    time_from_start: usize,
    answered: bool,
) -> anyhow::Result<()> {
    if answered {
        let _layout = question_layout(
            frame,
            question,
            players_answered_count,
            time_from_start,
            "Waiting for other players to answer...",
        );
    } else {
        let layout = question_layout(
            frame,
            question,
            players_answered_count,
            time_from_start,
            question.question.text.as_str(),
        );

        if question.code_block.is_some() {
            let code_paragraph =
                highlight_code_block(question.code_block.as_ref().unwrap(), Theme::OceanDark)
                    .block(get_bordered_block().padding(Padding::new(1, 1, 1, 1)));
            frame.render_widget(code_paragraph, layout[2]);
        }

        let mut items = choice_grid.clone().items();

        match &mut items[0][0] {
            Some(item) => {
                item.set_style_ref(style::Style::default().bg(Color::Red));
            }
            None => {}
        };

        match &mut items[0][1] {
            Some(item) => {
                item.set_style_ref(style::Style::default().bg(Color::Green));
            }
            None => {}
        };

        match &mut items[1][0] {
            Some(item) => {
                item.set_style_ref(style::Style::default().bg(Color::Blue));
            }
            None => {}
        };

        match &mut items[1][1] {
            Some(item) => {
                item.set_style_ref(style::Style::default().bg(Color::Yellow));
            }
            None => {}
        };

        *choice_grid = ChoiceGrid::new(items);

        let choice_selector = ChoiceSelector::new(choice_grid.clone());
        let choice_selector = choice_selector
            .vertical_gap(1)
            .horizontal_gap(3)
            .block(get_empty_block());

        if time_from_start >= question.show_choices_after {
            frame.render_stateful_widget(choice_selector, layout[3], choice_selector_state);
        }
    }

    Ok(())
}

pub fn question_answers(frame: &mut Frame, question: &QuestionEnded) -> anyhow::Result<()> {
    let outer_block = get_outer_block("Quiz name");
    let inner_block = get_inner_block(
        "Question ".to_string() + (question.question_index + 1).to_string().as_str(),
    );
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(content_space);

    let question_paragraph = Paragraph::new(question.question.text.to_string())
        .bold()
        .block(get_empty_block().padding(Padding::new(1, 1, 1, 1)))
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);

    frame.render_widget(question_paragraph, layout[1]);

    if question.question.code_block.is_some() {
        let code_paragraph = highlight_code_block(
            question.question.code_block.as_ref().unwrap(),
            Theme::SolarizedDark,
        )
        .block(get_bordered_block().padding(Padding::new(1, 1, 1, 1)));
        frame.render_widget(code_paragraph, layout[2]);
    }

    let choice_grid: ChoiceGrid = question.clone().question.into();
    // TODO set some fancy colors to the grid

    let choice_selector = ChoiceSelector::new(choice_grid);
    let choice_selector = choice_selector
        .vertical_gap(1)
        .horizontal_gap(3)
        .block(get_empty_block());

    frame.render_widget(choice_selector, layout[3]);

    Ok(())
}

pub fn results(
    frame: &mut Frame,
    results: &ShowLeaderboard,
    table_state: &mut TableState,
) -> anyhow::Result<()> {
    let layout = welcome_layout(
        frame,
        vec![Constraint::Length(1), Constraint::Percentage(90)],
        "Leaderboard:".to_string(),
    );

    let items: Vec<_> = results
        .players
        .iter()
        .map(|(player, score)| {
            let row = vec![
                Span::styled(player.nickname.to_string(), style::Style::default().bold()),
                Span::raw(format!("{}", score)),
            ];

            Row::new(row).style(
                style::Style::default()
                    .fg(Color::from_str(player.color.as_str()).unwrap_or(Color::White)),
            )
        })
        .collect();

    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
    let table = Table::new(items, widths)
        .block(get_bordered_block())
        .highlight_symbol(">> ");

    frame.render_stateful_widget(table, layout[1], table_state);

    Ok(())
}

pub fn end_game(frame: &mut Frame) -> anyhow::Result<()> {
    let lines = ["Game", "Ended", "Thank You!"];
    ascii_art(frame, &lines, "Press ENTER to close")?;

    Ok(())
}

pub fn error(frame: &mut Frame, message: &str) -> anyhow::Result<()> {
    simple_message(frame, "Error".to_string(), message)?;

    Ok(())
}

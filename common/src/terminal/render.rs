use std::rc::Rc;

use figlet_rs::FIGfont;
use log::{debug, trace};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{self, Color, Style, Stylize},
    text::Line,
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Row,
        Table, TableState, Wrap,
    },
    Frame,
};
use uuid::Uuid;

use crate::constants::{COLORS, MINIMAL_ASCII_HEIGHT, MINIMAL_ASCII_WIDTH};
use crate::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};
use crate::terminal::highlight::{highlight_code_block, Theme};
use crate::terminal::widgets::choice::{ChoiceGrid, ChoiceSelector, ChoiceSelectorState};

#[must_use]
pub fn get_outer_block(name: &str) -> Block<'_> {
    let title = Title::from(" Clihoot: ".to_owned() + name + " ");
    let block = Block::default()
        .title(title)
        .title_style(style::Style::default().bold())
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .padding(Padding::new(2, 2, 1, 1));
    block
}

#[must_use]
pub fn get_inner_block(title: &str) -> Block<'_> {
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

pub fn ascii_art(frame: &mut Frame, lines: &[&str], text: &str, quiz_name: &str) {
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
                let paragraph = Paragraph::new(figure.to_string())
                    .block(get_empty_block())
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, layout[i]);
            }
        }
        _ => {
            for i in 0..lines.len() {
                let paragraph = Paragraph::new(lines[i])
                    .wrap(Wrap { trim: true })
                    .block(get_empty_block())
                    .alignment(Alignment::Center);
                frame.render_widget(paragraph, layout[i]);
            }
        }
    }

    let paragraph = Paragraph::new(text)
        .wrap(Wrap { trim: true })
        .block(get_empty_block())
        .alignment(Alignment::Center);
    frame.render_widget(paragraph, layout[lines.len()]);
}

pub fn welcome_results_layout(
    frame: &mut Frame,
    constraints: Vec<Constraint>,
    paragraph_name: String,
    block_name: &str,
    quiz_name: &str,
) -> Rc<[Rect]> {
    let outer_block = get_outer_block(quiz_name);
    let inner_block = get_inner_block(block_name);
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(content_space);

    let paragraph = Paragraph::new(paragraph_name)
        .wrap(Wrap { trim: true })
        .block(get_empty_block());

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, layout[0]);

    layout
}

pub fn simple_message(frame: &mut Frame, title: &str, message: &str, quiz_name: &str) {
    let outer_block = get_outer_block(quiz_name);
    let inner_block = get_inner_block(title);
    let inner = outer_block.inner(frame.size());

    let content_space = inner_block.inner(inner);

    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .block(get_empty_block())
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);
    frame.render_widget(paragraph, content_space);
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
    let layout = welcome_results_layout(
        frame,
        constraints.clone(),
        "Players waiting for the game to start:".to_string(),
        " Welcome! ",
        quiz_name,
    );

    let items: Vec<_> = players
        .iter()
        .map(|player| {
            let item = ListItem::new(player.nickname.to_string())
                .style(style::Style::default().fg(player.color));
            if player.uuid == player_uuid.unwrap_or(Uuid::nil()) {
                item.underlined().bold()
            } else {
                item
            }
        })
        .collect();

    let list = List::new(items)
        .block(get_bordered_block())
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, layout[1], list_state);

    if constraints.len() == 3 {
        let paragraph = Paragraph::new("Press Enter to start the game!")
            .wrap(Wrap { trim: true })
            .block(get_empty_block())
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, layout[2]);
    }
}

fn question_time(
    frame: &mut Frame,
    question: &NextQuestion,
    players_answered_count: usize,
    time_from_start: usize,
    layout: &[Rect],
) {
    let counts_block = get_bordered_block().padding(Padding::new(1, 1, 0, 0));
    let counts_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(counts_block.inner(layout[0]));

    let time_paragraph = Paragraph::new(format!(
        "Time left: {}",
        (question.show_choices_after + question.time_seconds).saturating_sub(time_from_start)
    ))
    .wrap(Wrap { trim: true })
    .alignment(Alignment::Left)
    .block(get_empty_block());
    let asnwered_paragraph = Paragraph::new(format!("Players answered: {players_answered_count}"))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .block(get_empty_block());
    let type_paragraph = Paragraph::new(format!(
        "Type: {}",
        match question.is_multichoice {
            true => "Multi choice",
            false => "Single choice",
        }
    ))
    .wrap(Wrap { trim: true })
    .alignment(Alignment::Center)
    .block(get_empty_block());

    frame.render_widget(counts_block, layout[0]);
    frame.render_widget(time_paragraph, counts_layout[0]);
    frame.render_widget(type_paragraph, counts_layout[1]);
    frame.render_widget(asnwered_paragraph, counts_layout[2]);
}

fn question_layout(frame: &mut Frame, title: &str, text: &str, quiz_name: &str) -> Rc<[Rect]> {
    let outer_block = get_outer_block(quiz_name);

    let inner_block = get_inner_block(title);
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

    let paragraph = Paragraph::new(text)
        .bold()
        .wrap(Wrap { trim: true })
        .block(get_empty_block().padding(Padding::new(1, 1, 1, 1)))
        .alignment(Alignment::Center);

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);

    frame.render_widget(paragraph, layout[1]);

    layout
}

#[allow(clippy::too_many_arguments)]
pub fn question(
    frame: &mut Frame,
    question: &NextQuestion,
    players_answered_count: usize,
    choice_grid: &mut ChoiceGrid,
    choice_selector_state: &mut ChoiceSelectorState,
    time_from_start: usize,
    answered: bool,
    theme: Theme,
    quiz_name: &str,
) {
    let layout = question_layout(
        frame,
        &format!(
            " Question {}/{} ",
            question.question_index + 1,
            question.questions_count
        ),
        if answered {
            "Waiting for other players to answer..."
        } else {
            question.question.text.as_str()
        },
        quiz_name,
    );

    question_time(
        frame,
        question,
        players_answered_count,
        time_from_start,
        &layout,
    );

    if answered {
        return;
    }

    if question.code_block.is_some() {
        let code_paragraph = highlight_code_block(
            question
                .code_block
                .as_ref()
                .expect("Code block should be Some"),
            theme,
        )
        .block(get_bordered_block().padding(Padding::new(1, 1, 1, 1)));
        frame.render_widget(code_paragraph, layout[2]);
    }

    if time_from_start < question.show_choices_after {
        let time = question.show_choices_after.saturating_sub(time_from_start);
        let paragraph = Paragraph::new(format!(
            "Choices will be displayed in {} second{}!",
            time,
            if time == 1 { "" } else { "s" }
        ))
        .wrap(Wrap { trim: true })
        .block(Block::default().padding(Padding::new(0, 0, layout[3].height / 2, 0)))
        .alignment(Alignment::Center);

        frame.render_widget(paragraph, layout[3]);
        return;
    }

    let mut items = choice_grid.clone().items();

    let mut color_index = 0;
    for (row, items) in items.iter_mut().enumerate() {
        for (col, mut items) in items.iter_mut().enumerate() {
            trace!("row: {row}, col: {col}");
            match &mut items {
                Some(item) => {
                    color_index += 1;

                    item.set_style_ref(style::Style::default().fg(COLORS[color_index]));
                }
                None => {}
            }
        }
    }

    *choice_grid = ChoiceGrid::new(items);

    let choice_selector = ChoiceSelector::new(choice_grid.clone());
    let choice_selector = choice_selector
        .vertical_gap(1)
        .horizontal_gap(2)
        .current_item_style(Style::default().bg(Color::White))
        .selected_item_block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        )
        .block(get_empty_block());

    if choice_selector_state.is_none() {
        let choice_selector = choice_selector.current_item_style(Style::default());
        frame.render_widget(choice_selector, layout[3]);
    } else {
        frame.render_stateful_widget(choice_selector, layout[3], choice_selector_state);
    }
}

pub fn question_answers(
    frame: &mut Frame,
    question: &QuestionEnded,
    theme: Theme,
    quiz_name: &str,
) {
    let layout = question_layout(
        frame,
        &format!(" Question {} ", question.question_index + 1),
        &question.question.text,
        quiz_name,
    );

    if question.question.code_block.is_some() {
        let code_paragraph = highlight_code_block(
            question
                .question
                .code_block
                .as_ref()
                .expect("Code block should be Some"),
            theme,
        )
        .block(get_bordered_block().padding(Padding::new(1, 1, 1, 1)));
        frame.render_widget(code_paragraph, layout[2]);
    }

    let mut choice_grid: ChoiceGrid = question.clone().question.into();
    let mut items = choice_grid.clone().items();

    for (row, items) in items.iter_mut().enumerate() {
        for (col, mut items) in items.iter_mut().enumerate() {
            match &mut items {
                Some(item) => {
                    item.set_style_ref(Style::default());

                    let was_selected_by_user = question
                        .player_answer
                        .iter()
                        .any(|choice| choice.contains(&item.get_uuid()));

                    debug!("was_selected_by_user {row} {col}: {was_selected_by_user}");

                    let answers_count = match question.stats.get(&item.get_uuid()) {
                        Some(count) => count.players_answered_count,
                        None => 0,
                    };

                    let title = Title::from(answers_count.to_string())
                        .alignment(Alignment::Right)
                        .position(Position::Top);

                    if was_selected_by_user {
                        item.set_block_ref(
                            get_bordered_block()
                                .border_type(BorderType::Double)
                                .title(title),
                        );
                        item.set_style_ref(Style::default().bold());
                    } else {
                        item.set_block_ref(get_bordered_block().title(title));
                    }
                }
                None => {}
            }
        }
    }

    choice_grid = ChoiceGrid::new(items);

    let choice_selector = ChoiceSelector::new(choice_grid);
    let choice_selector = choice_selector
        .vertical_gap(1)
        .horizontal_gap(3)
        .current_item_style(Style::default())
        .correct_item_style(Style::default().bg(Color::Green))
        .block(get_empty_block());

    frame.render_widget(choice_selector, layout[3]);
}

pub fn results(
    frame: &mut Frame,
    results: &ShowLeaderboard,
    table_state: &mut TableState,
    player_name: &str,
    quiz_name: &str,
) {
    let mut layout = welcome_results_layout(
        frame,
        vec![Constraint::Length(1), Constraint::Percentage(90)],
        "Leaderboard:".to_string(),
        " Results! ",
        quiz_name,
    );

    if results.was_final_round {
        layout = welcome_results_layout(
            frame,
            vec![
                Constraint::Length(1),
                Constraint::Percentage(90),
                Constraint::Length(1),
            ],
            "Final Leaderboard:".to_string(),
            " Final Results! ",
            quiz_name,
        );

        let paragraph = Paragraph::new("Great job!")
            .wrap(Wrap { trim: true })
            .block(get_empty_block())
            .alignment(Alignment::Center);
        frame.render_widget(paragraph, layout[2]);
    }

    let items: Vec<_> = results
        .players
        .iter()
        .map(|(player, score)| {
            let mut name_cell = Line::styled(player.nickname.to_string(), style::Style::default())
                .alignment(Alignment::Left);
            let mut score_cell = Line::raw(format!("{score}")).alignment(Alignment::Center);
            if player.nickname == player_name {
                name_cell.patch_style(style::Style::default().underlined().bold());
                score_cell.patch_style(style::Style::default().underlined().bold());
            }
            let row = vec![name_cell, score_cell];

            Row::new(row).style(style::Style::default().fg(player.color))
        })
        .collect();

    let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

    let cells = vec![
        Line::styled("Player", style::Style::default()).alignment(Alignment::Left),
        Line::raw("Score").alignment(Alignment::Center),
    ];

    let table = Table::new(items, widths)
        .header(Row::new(cells).underlined())
        .block(get_bordered_block())
        .highlight_symbol(">> ");

    frame.render_stateful_widget(table, layout[1], table_state);
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

pub fn help(frame: &mut Frame, help_text: &[(&str, &str)]) {
    let title = Title::from(" Help ")
        .alignment(Alignment::Center)
        .position(Position::Top);
    let bottom_title = Title::from(" Press any key to close ")
        .alignment(Alignment::Center)
        .position(Position::Bottom);
    let popup_block = Block::default()
        .title(title)
        .title(bottom_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(frame.size(), 60, 60);

    let items: Vec<_> = help_text
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
    let table = Table::new(items, widths).block(popup_block);

    frame.render_widget(Clear, area);
    frame.render_widget(table, area);
}

pub fn yes_no_popup(frame: &mut Frame, message: &str) {
    let title = Title::from(" Confirm ")
        .alignment(Alignment::Center)
        .position(Position::Top);
    let bottom_title = Title::from(" Press y to confirm ")
        .alignment(Alignment::Center)
        .position(Position::Bottom);
    let popup_block = Block::default()
        .title(title)
        .title(bottom_title)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .padding(Padding::new(1, 1, 1, 1))
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(frame.size(), 60, 30);

    let paragraph = Paragraph::new(message)
        .wrap(Wrap { trim: true })
        .block(popup_block)
        .alignment(Alignment::Center);

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

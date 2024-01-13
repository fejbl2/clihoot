use crate::{constants::COLORS, messages::network::QuestionEnded, questions::CodeBlock};
use std::rc::Rc;

use crate::{
    messages::network::NextQuestion,
    terminal::{
        highlight::{highlight_code_block, Theme},
        widgets::choice::{ChoiceGrid, ChoiceSelector, ChoiceSelectorState},
    },
};
use log::{debug, trace};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{self},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Padding, Paragraph, Wrap,
    },
};

use super::{get_bordered_block, get_centered_paragraph, get_inner_block, get_outer_block};

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
    .block(Block::default());

    let asnwered_paragraph = Paragraph::new(format!("Players answered: {players_answered_count}"))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Right)
        .block(Block::default());

    let text = format!(
        "Type: {}",
        if question.is_multichoice {
            "Multi choice"
        } else {
            "Single choice"
        }
    );

    let type_paragraph = get_centered_paragraph(&text, Block::default());

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

    let paragraph =
        get_centered_paragraph(text, Block::default().padding(Padding::new(1, 1, 1, 1)));

    frame.render_widget(outer_block, frame.size());
    frame.render_widget(inner_block, inner);

    frame.render_widget(paragraph, layout[1]);

    layout
}

fn code(frame: &mut Frame, code_block: &CodeBlock, theme: Theme, layout: &[Rect]) {
    let code_paragraph = highlight_code_block(code_block, theme)
        .block(get_bordered_block().padding(Padding::new(1, 1, 1, 1)));
    frame.render_widget(code_paragraph, layout[2]);
}

#[allow(clippy::too_many_arguments)]
pub fn question(
    frame: &mut Frame,
    question: &NextQuestion,
    players_answered_count: usize,
    choice_grid: &mut ChoiceGrid,
    choice_selector_state: Option<&mut ChoiceSelectorState>,
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

    if let Some(code_block) = &question.question.code_block {
        code(frame, code_block, theme, &layout);
    }

    if time_from_start < question.show_choices_after {
        let time = question.show_choices_after.saturating_sub(time_from_start);

        let text = format!(
            "Choices will be displayed in {} second{}!",
            time,
            if time == 1 { "" } else { "s" }
        );
        let paragraph = get_centered_paragraph(
            &text,
            Block::default().padding(Padding::new(0, 0, layout[3].height / 2, 0)),
        );

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

                    item.set_style_ref(
                        style::Style::default().fg(COLORS[color_index % COLORS.len()]),
                    );
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
        .block(Block::default());

    match choice_selector_state {
        Some(state) => {
            frame.render_stateful_widget(choice_selector, layout[3], state);
        }
        None => {
            let choice_selector = choice_selector.current_item_style(Style::default());
            frame.render_widget(choice_selector, layout[3]);
        }
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

    if let Some(code_block) = &question.question.code_block {
        code(frame, code_block, theme, &layout);
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
        .block(Block::default());

    frame.render_widget(choice_selector, layout[3]);
}

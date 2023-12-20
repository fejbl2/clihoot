use anyhow::anyhow;
use common::{messages::network::AnswerSelected, questions::QuestionSet};
use uuid::Uuid;

use super::state::QuestionRecords;

pub fn calculate_points(
    _player: Uuid,
    answer_order: usize,
    total_players: usize,
    question: usize,
    answers: &AnswerSelected,
    questions: &QuestionSet,
    _results: &QuestionRecords,
) -> anyhow::Result<usize> {
    // find the question
    let question = questions
        .get(question)
        .ok_or(anyhow!("Question not found"))?;

    // find the correct answers
    let mut correct_answers = question.choices.iter().filter(|choice| choice.is_right);

    let num_correct = answers
        .iter()
        .filter(|answer| correct_answers.any(|choice| choice.id == **answer))
        .count();

    let num_wrong = answers.len() - num_correct;

    let modifier = total_players - answer_order + 10; // magic constant

    Ok(modifier * usize::saturating_sub(num_correct * 10, num_wrong * 5))
}

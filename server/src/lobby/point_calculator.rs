use anyhow::anyhow;
use common::{messages::network::AnswerSelected, questions::QuestionSet};
use log::debug;
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
    let mut correct_answers = question.choices.iter().filter(|choice| choice.is_correct);
    debug!(
        "Question has {} correct answers",
        correct_answers.clone().collect::<Vec<_>>().len()
    );

    // TODO: this is wrong
    let num_correct = answers
        .answers
        .iter()
        .filter(|answer| correct_answers.any(|correct| correct.id == **answer))
        .count();

    let num_wrong = answers.answers.len() - num_correct;
    //

    debug!("Total players: {total_players}, answer order: {answer_order}");

    // Correct modifier
    let modifier = total_players - answer_order + 10; // magic constant
    debug!("Modifier: {modifier}");

    debug!(
        "Final points: {}",
        modifier * usize::saturating_sub(num_correct * 10, num_wrong * 5)
    );

    Ok(modifier * usize::saturating_sub(num_correct * 10, num_wrong * 5))
}

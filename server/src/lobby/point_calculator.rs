use std::collections::HashSet;

use anyhow::anyhow;
use common::{messages::network::AnswerSelected, questions::QuestionSet};
use log::debug;
use uuid::Uuid;

use super::state::QuestionRecords;

pub fn calculate_points(
    player: Uuid,
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
    let correct_answers = question
        .choices
        .iter()
        .filter(|choice| choice.is_correct)
        .map(|choice| choice.id)
        .collect::<HashSet<_>>();
    debug!("Question has {} correct answers", correct_answers.len());

    let num_correct = answers.answers.intersection(&correct_answers).count();

    let num_wrong = answers.answers.len() - num_correct;

    debug!("Player {player} got {num_correct} correct and {num_wrong} wrong");

    debug!("Total players: {total_players}, answer order: {answer_order}");

    // Correct modifier
    let modifier = total_players - answer_order + 10; // magic constant
    debug!("Modifier: {modifier}");

    let final_points = modifier * usize::saturating_sub(num_correct * 10, num_wrong * 5);
    debug!("Final points: {final_points}");

    Ok(final_points)
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use common::constants::DEFAULT_QUIZ_NAME;
    use common::questions::QuestionSet;
    use common::questions::{Choice, Question};

    #[test]
    fn test_calculate_points() -> anyhow::Result<()> {
        let player_id = Uuid::new_v4();
        let answer_order = 1;
        let total_players = 4;
        let question_index = 0;

        let choice_1 = Choice {
            id: Uuid::new_v4(),
            text: "42".to_string(),
            is_correct: true,
        };

        let choice_2 = Choice {
            id: Uuid::new_v4(),
            text: "43".to_string(),
            is_correct: true,
        };

        let choice_3 = Choice {
            id: Uuid::new_v4(),
            text: "44".to_string(),
            is_correct: false,
        };

        let choice_4 = Choice {
            id: Uuid::new_v4(),
            text: "45".to_string(),
            is_correct: false,
        };

        let questions = QuestionSet {
            questions: vec![Question {
                text: "What is the answer to life, the universe, and everything?".to_string(),
                choices: vec![
                    choice_1.clone(),
                    choice_2.clone(),
                    choice_3.clone(),
                    choice_4.clone(),
                ],
                code_block: None,
                time_seconds: 10,
                is_multichoice: true,
            }],
            randomize_answers: false,
            randomize_questions: false,
            quiz_name: DEFAULT_QUIZ_NAME.to_owned(),
        };

        let answers = AnswerSelected {
            answers: HashSet::from([choice_2.id, choice_1.id]),
            player_uuid: player_id,
            question_index,
        };

        let results = HashMap::new();

        let points = calculate_points(
            player_id,
            answer_order,
            total_players,
            question_index,
            &answers,
            &questions,
            &results,
        )?;

        assert!(points > 200);

        let answers = AnswerSelected {
            answers: HashSet::from([choice_1.id, choice_3.id]),
            player_uuid: player_id,
            question_index,
        };

        let points = calculate_points(
            player_id,
            answer_order,
            total_players,
            question_index,
            &answers,
            &questions,
            &results,
        )?;

        assert!(points > 0);

        let answers = AnswerSelected {
            answers: HashSet::from([choice_4.id, choice_3.id]),
            player_uuid: player_id,
            question_index,
        };

        let points = calculate_points(
            player_id,
            answer_order,
            total_players,
            question_index,
            &answers,
            &questions,
            &results,
        )?;

        assert_eq!(points, 0);
        Ok(())
    }
}

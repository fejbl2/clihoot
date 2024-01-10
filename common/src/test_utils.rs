use crate::questions::{self, QuestionSet};

#[must_use]
pub fn compare_question_sets(left: &QuestionSet, right: &QuestionSet) -> bool {
    for (left_question, right_question) in left.questions.iter().zip(right.questions.iter()) {
        if !compare_questions(left_question, right_question) {
            return false;
        }
    }

    true
}

#[must_use]
pub fn compare_censored_questions(
    left: &questions::QuestionCensored,
    right: &questions::QuestionCensored,
) -> bool {
    if left.text != right.text {
        return false;
    }

    if left.time_seconds != right.time_seconds {
        return false;
    }

    if left.code_block != right.code_block {
        return false;
    }

    if left.choices.len() != right.choices.len() {
        return false;
    }

    if left.is_multichoice != right.is_multichoice {
        return false;
    }

    for (left_choice, right_choice) in left.choices.iter().zip(right.choices.iter()) {
        if left_choice.text != right_choice.text {
            return false;
        }
    }

    true
}

fn compare_questions(left: &questions::Question, right: &questions::Question) -> bool {
    if left.text != right.text {
        return false;
    }

    if left.time_seconds != right.time_seconds {
        return false;
    }

    if left.code_block != right.code_block {
        return false;
    }

    if left.choices.len() != right.choices.len() {
        return false;
    }

    if left.is_multichoice != right.is_multichoice {
        return false;
    }

    for (left_choice, right_choice) in left.choices.iter().zip(right.choices.iter()) {
        if left_choice.text != right_choice.text {
            return false;
        }

        if left_choice.is_correct != right_choice.is_correct {
            return false;
        }
    }

    true
}

#[must_use]
pub fn no_code_question_fixture() -> questions::Question {
    questions::Question {
        text: "What is the answer to the ultimate question of life, the Universe, and Everything?"
            .to_string(),
        code_block: None,
        time_seconds: 42,
        is_multichoice: false,
        choices: vec![
            questions::Choice {
                id: uuid::Uuid::nil(),
                text: "sleep".to_string(),
                is_correct: false,
            },
            questions::Choice {
                id: uuid::Uuid::nil(),
                text: "42".to_string(),
                is_correct: true,
            },
            questions::Choice {
                id: uuid::Uuid::nil(),
                text: "food".to_string(),
                is_correct: false,
            },
            questions::Choice {
                id: uuid::Uuid::nil(),
                text: "69".to_string(),
                is_correct: false,
            },
        ],
    }
}

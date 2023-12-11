use std::path::Path;

use common::questions;
use uuid::Uuid;

fn no_code_question_fixture(ids: Vec<Uuid>) -> questions::Question {
    questions::Question {
        text: "What is the answer to the ultimate question of life, the Universe, and Everything?"
            .to_string(),
        code_block: None,
        time_seconds: 42,
        choices: vec![
            questions::Choice {
                id: ids[0],
                text: "sleep".to_string(),
                is_right: false,
            },
            questions::Choice {
                id: ids[1],
                text: "42".to_string(),
                is_right: true,
            },
            questions::Choice {
                id: ids[2],
                text: "food".to_string(),
                is_right: false,
            },
            questions::Choice {
                id: ids[3],
                text: "69".to_string(),
                is_right: false,
            },
        ],
    }
}

#[test]
fn test_ok_minimal() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/ok_minimal.yaml")).unwrap();

    let wanted = questions::QuestionSet {
        quiz_name: "test quiz".to_owned(),
        randomize_answers: false,
        randomize_questions: false,
        questions: vec![no_code_question_fixture(
            result.questions[0].choices.iter().map(|c| c.id).collect(),
        )],
    };

    assert_eq!(result, wanted);
}

#[test]
fn test_ok_code() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/ok_code.yaml")).unwrap();

    let wanted = questions::QuestionSet {
        quiz_name: "test quiz".to_owned(),
        randomize_answers: false,
        randomize_questions: false,
        questions: vec![questions::Question {
            text: "What does this code do?".to_string(),
            code_block: Some(questions::CodeBlock {
                language: "rust".to_string(),
                code: "fn main() {\n    println!(\"42\");\n}\n".to_string(),
            }),
            time_seconds: 42,
            choices: vec![
                questions::Choice {
                    id: result.questions[0].choices[0].id,
                    text: "Nothing useful".to_string(),
                    is_right: false,
                },
                questions::Choice {
                    id: result.questions[0].choices[1].id,
                    text: "It prints 42".to_string(),
                    is_right: true,
                },
                questions::Choice {
                    id: result.questions[0].choices[2].id,
                    text: "It fails to compile and the compiler will scream at us".to_string(),
                    is_right: false,
                },
                questions::Choice {
                    id: result.questions[0].choices[3].id,
                    text:
                        "It answers to the ultimate question of life, the Universe, and Everything"
                            .to_string(),
                    is_right: true,
                },
            ],
        }],
    };

    assert_eq!(result, wanted);
}

#[test]
fn test_ok_multiple() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/ok_multiple.yaml")).unwrap();

    let wanted = questions::QuestionSet {
        quiz_name: "test quiz".to_owned(),
        randomize_answers: false,
        randomize_questions: false,
        questions: vec![
            no_code_question_fixture(result.questions[0].choices.iter().map(|c| c.id).collect()),
            no_code_question_fixture(result.questions[1].choices.iter().map(|c| c.id).collect()),
            no_code_question_fixture(result.questions[2].choices.iter().map(|c| c.id).collect()),
        ],
    };

    assert_eq!(result, wanted);
}

#[test]
fn test_missing_field() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_missing_field.yaml"));

    assert!(result.is_err());
}

#[test]
fn test_no_right_choice() {
    let result = questions::QuestionSet::from_file(Path::new("./tests/files/err_no_right.yaml"));

    assert!(result.is_err());
}

#[test]
fn test_too_much_choices() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_too_much_choices.yaml"));

    assert!(result.is_err());
}

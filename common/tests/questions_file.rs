use common::assert_questionset_eq;
use common::questions::{self};
use common::test_utils::compare_question_sets;
use common::test_utils::no_code_question_fixture;
use std::path::Path;
use uuid::Uuid; // Import the missing module `tests` from the `common` module..

#[test]
fn test_ok_minimal() {
    let result = questions::QuestionSet::from_file(Path::new("./tests/files/ok_minimal.yaml"))
        .expect("Question file should be OK");

    let wanted = questions::QuestionSet::new(vec![no_code_question_fixture()]);

    assert_questionset_eq!(result, wanted);
}

#[test]
fn test_ok_language_name() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/ok_language_name.yaml"));

    assert!(result.is_ok());
}

#[test]
fn test_ok_language_name_uppercase() {
    let result = questions::QuestionSet::from_file(Path::new(
        "./tests/files/ok_language_name_uppercase.yaml",
    ));

    assert!(result.is_ok());
}

#[test]
fn test_ok_code() {
    let result = questions::QuestionSet::from_file(Path::new("./tests/files/ok_code.yaml"))
        .expect("Question file should be OK");

    let wanted = questions::QuestionSet::new(vec![questions::Question {
        text: "What does this code do?".to_string(),
        code_block: Some(questions::CodeBlock {
            language: "rs".to_string(),
            code: "fn main() {\n    println!(\"42\");\n}\n".to_string(),
        }),
        time_seconds: 42,
        is_multichoice: true,
        choices: vec![
            questions::Choice {
                id: Uuid::nil(),
                text: "Nothing useful".to_string(),
                is_correct: false,
            },
            questions::Choice {
                id: Uuid::nil(),
                text: "It prints 42".to_string(),
                is_correct: true,
            },
            questions::Choice {
                id: Uuid::nil(),
                text: "It fails to compile and the compiler will scream at us".to_string(),
                is_correct: false,
            },
            questions::Choice {
                id: Uuid::nil(),
                text: "It answers to the ultimate question of life, the Universe, and Everything"
                    .to_string(),
                is_correct: true,
            },
        ],
    }]);

    assert_questionset_eq!(result, wanted);
}

#[test]
fn test_ok_multiple() {
    let result = questions::QuestionSet::from_file(Path::new("./tests/files/ok_multiple.yaml"))
        .expect("Question file should be OK");

    let wanted = questions::QuestionSet::new(vec![
        no_code_question_fixture(),
        no_code_question_fixture(),
        no_code_question_fixture(),
    ]);

    assert_questionset_eq!(result, wanted);
}

#[test]
fn test_missing_field() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_missing_field.yaml"));

    assert!(result.is_err());
}

#[test]
fn test_too_long_question() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_too_long_question.yaml"));

    assert!(result.is_err());
}

#[test]
fn test_too_long_code() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_code_too_long.yaml"));

    assert!(result.is_err());
}

#[test]
fn test_too_long_choice() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_choice_too_long.yaml"));

    assert!(result.is_err());
}

#[test]
fn test_unknown_language() {
    let result =
        questions::QuestionSet::from_file(Path::new("./tests/files/err_unknown_language.yaml"));

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

#[test]
fn test_quiz_name() {
    let result = questions::QuestionSet::from_file(Path::new("./tests/files/ok_quiz_name.yaml"))
        .expect("Question file should be OK");

    assert_eq!(result.quiz_name, "Very fancy quiz");
}

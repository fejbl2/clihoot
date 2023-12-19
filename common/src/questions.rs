use anyhow::Context;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use uuid::Uuid;

use crate::constants::DEFAULT_QUIZ_NAME;

fn falsy() -> bool {
    false
}

fn random_quiz_name() -> String {
    DEFAULT_QUIZ_NAME.to_owned()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionSet {
    pub questions: Vec<Question>,

    #[serde(default = "falsy", skip_deserializing, skip_serializing)]
    pub randomize_answers: bool,

    #[serde(default = "falsy", skip_deserializing, skip_serializing)]
    pub randomize_questions: bool,

    #[serde(default = "random_quiz_name", skip_deserializing, skip_serializing)]
    pub quiz_name: String,
}

/// We want to be able to iterate over the questions in the set directly
impl Deref for QuestionSet {
    type Target = Vec<Question>;

    fn deref(&self) -> &Self::Target {
        &self.questions
    }
}

impl DerefMut for QuestionSet {
    fn deref_mut(&mut self) -> &mut Vec<Question> {
        &mut self.questions
    }
}

fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Question {
    pub text: String,
    pub code_block: Option<CodeBlock>,
    pub time_seconds: usize,
    #[serde(deserialize_with = "deserialize_choices")]
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionCensored {
    pub text: String,
    pub code_block: Option<CodeBlock>,
    pub time_seconds: usize,
    pub choices: Vec<ChoiceCensored>,
}

impl From<Question> for QuestionCensored {
    fn from(question: Question) -> Self {
        Self {
            text: question.text,
            code_block: question.code_block,
            time_seconds: question.time_seconds,
            choices: question
                .choices
                .iter()
                .map(|choice| ChoiceCensored {
                    id: choice.id,
                    text: choice.text.clone(),
                })
                .collect(),
        }
    }
}

impl Question {
    #[must_use]
    pub fn get_reading_time_estimate(&self) -> usize {
        let words = self.text.split_whitespace().count()
            + self
                .code_block
                .as_ref()
                .map_or(0, |code| code.code.split_whitespace().count());

        // 200 words per minute
        let estimate_secs = words * 6 / 20;
        if estimate_secs == 0 {
            return 1;
        }

        estimate_secs
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Choice {
    // we want to be able to identify the choices even when the client shuffles them
    #[serde(default = "new_uuid")]
    pub id: Uuid,
    // by design, no syntax highlighting for the choices
    pub text: String,
    #[serde(default)]
    pub is_right: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChoiceCensored {
    pub id: Uuid,
    pub text: String,
}

fn deserialize_choices<'de, D>(deserializer: D) -> Result<Vec<Choice>, D::Error>
where
    D: Deserializer<'de>,
{
    let choices: Vec<Choice> = Deserialize::deserialize(deserializer)?;

    if choices.is_empty() || choices.len() > 4 {
        return Err(serde::de::Error::invalid_length(
            choices.len(),
            &"1 to 4 choices",
        ));
    }

    let right_answers = choices.iter().filter(|choice| choice.is_right).count();

    if right_answers == 0 {
        return Err(de::Error::custom("At least one choice must be right"));
    }

    Ok(choices)
}

impl QuestionSet {
    /// Loads a question set from a file
    /// # Errors
    /// If the file cannot be read or the YAML cannot be parsed
    pub fn from_file(path: &Path) -> anyhow::Result<QuestionSet> {
        let data = fs::read_to_string(path)?;
        let questions = serde_yaml::from_str(&data).context(format!(
            "Error while evaluating file \"{}\"",
            path.display()
        ))?;

        Ok(questions)
    }

    #[must_use]
    pub fn new(questions: Vec<Question>) -> Self {
        Self {
            questions,
            randomize_answers: false,
            randomize_questions: false,
            quiz_name: DEFAULT_QUIZ_NAME.to_owned(),
        }
    }
}

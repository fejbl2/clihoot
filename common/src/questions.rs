use anyhow::{bail, Context};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use uuid::Uuid;

use crate::constants::{
    DEFAULT_QUIZ_NAME, MAXIMAL_CHOICE_LENGTH, MAXIMAL_CODE_LENGTH, MAXIMAL_QUESTION_LENGTH,
};

fn falsy() -> bool {
    false
}

fn default_quiz_name() -> String {
    DEFAULT_QUIZ_NAME.to_owned()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionSet {
    pub questions: Vec<Question>,

    #[serde(default = "falsy", skip_deserializing, skip_serializing)]
    pub randomize_answers: bool,

    #[serde(default = "falsy", skip_deserializing, skip_serializing)]
    pub randomize_questions: bool,

    #[serde(default = "default_quiz_name")]
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
    #[serde(deserialize_with = "deserialize_question_text")]
    pub text: String,
    pub code_block: Option<CodeBlock>,
    pub time_seconds: usize,
    pub is_multichoice: bool,
    #[serde(deserialize_with = "deserialize_choices")]
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct QuestionCensored {
    #[serde(deserialize_with = "deserialize_question_text")]
    pub text: String,
    pub code_block: Option<CodeBlock>,
    pub time_seconds: usize,
    pub is_multichoice: bool,
    pub choices: Vec<ChoiceCensored>,
}

impl From<Question> for QuestionCensored {
    fn from(question: Question) -> Self {
        Self {
            text: question.text,
            code_block: question.code_block,
            time_seconds: question.time_seconds,
            is_multichoice: question.is_multichoice,
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
    #[serde(deserialize_with = "deserialize_language")]
    pub language: String,

    #[serde(deserialize_with = "deserialize_code_text")]
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
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChoiceCensored {
    pub id: Uuid,
    pub text: String,
}

pub fn find_syntax(language: &str, code: Option<&str>) -> anyhow::Result<SyntaxReference> {
    let ss = SyntaxSet::load_defaults_newlines();

    if let Some(syntax) = ss.find_syntax_by_token(language) {
        return Ok(syntax.clone());
    }

    if let Some(code) = code {
        if let Some(syntax) = ss.find_syntax_by_first_line(code) {
            return Ok(syntax.clone());
        }
    }

    bail!("Unknown syntax \"{}\"", language)
}

fn deserialize_language<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let language: String = Deserialize::deserialize(deserializer)?;

    find_syntax(&language, None)
        .map(|_| language)
        .map_err(|err| de::Error::custom(err.to_string()))
}

fn deserialize_code_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let text: String = Deserialize::deserialize(deserializer)?;

    if text.chars().count() > MAXIMAL_CODE_LENGTH {
        return Err(de::Error::custom(format!(
            "Code text must be at most {MAXIMAL_CODE_LENGTH} chars"
        )));
    }

    Ok(text)
}

fn deserialize_question_text<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let text: String = Deserialize::deserialize(deserializer)?;

    if text.chars().count() > MAXIMAL_QUESTION_LENGTH {
        return Err(de::Error::custom(format!(
            "Question text must be at most {MAXIMAL_QUESTION_LENGTH} chars"
        )));
    }

    Ok(text)
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

    let right_answers = choices.iter().filter(|choice| choice.is_correct).count();

    if right_answers == 0 {
        return Err(de::Error::custom("At least one choice must be right"));
    }

    // max length of text of a choice is 50 chars
    if choices
        .iter()
        .any(|choice| choice.text.chars().count() > MAXIMAL_CHOICE_LENGTH)
    {
        return Err(de::Error::custom(format!(
            "Choice text must be at most {MAXIMAL_CHOICE_LENGTH} chars"
        )));
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

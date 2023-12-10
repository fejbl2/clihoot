use anyhow::Context;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::ops::Deref;
use std::path::Path;
use uuid::Uuid;

fn falsy() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuestionSet {
    pub questions: Vec<Question>,
    #[serde(default = "falsy", skip_deserializing, skip_serializing)]
    pub randomize_answers: bool,
    #[serde(default = "falsy", skip_deserializing, skip_serializing)]
    pub randomize_questions: bool,
}

/// We want to be able to iterate over the questions in the set directly
impl Deref for QuestionSet {
    type Target = Vec<Question>;

    fn deref(&self) -> &Self::Target {
        &self.questions
    }
}

fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Question {
    pub text: String,
    pub code_block: Option<CodeBlock>,
    pub time_seconds: u32,
    #[serde(deserialize_with = "deserialize_choices")]
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Choice {
    // we want to be able to identify the choices even when the client shuffles them
    #[serde(default = "new_uuid", skip_deserializing, skip_serializing)]
    pub id: Uuid,
    // by design, no syntax highlighting for the choices
    pub text: String,
    #[serde(default)]
    pub is_right: bool,
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
    pub fn from_file(path: &Path) -> anyhow::Result<QuestionSet> {
        let data = fs::read_to_string(path)?;
        let questions = serde_yaml::from_str(&data).context(format!(
            "Error while evaluating file \"{}\"",
            path.display()
        ))?;

        Ok(questions)
    }
}

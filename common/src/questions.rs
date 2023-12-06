use anyhow::Context;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct QuestionSet {
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub text: String,
    pub code_block: Option<CodeBlock>,
    pub time_seconds: u32,
    #[serde(deserialize_with = "deserialize_choices")]
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Choice {
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

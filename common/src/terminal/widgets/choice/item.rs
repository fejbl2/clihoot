use ratatui::style::{Style, Styled};
use ratatui::widgets::{Block, Borders};
use std::cmp::PartialEq;
use uuid::Uuid;

use crate::questions::{Choice, ChoiceCensored};

#[derive(Debug, Clone, Default, PartialEq, Copy)]
pub struct ChoiceItem {
    pub(super) content: String,
    pub(super) is_correct: bool,
    pub(super) uuid: Uuid,
    pub(super) style: Style,
    pub(super) block: Block<'static>,
}

impl ChoiceItem {
    #[must_use]
    pub fn new(content: String, is_correct: bool, uuid: Uuid) -> Self {
        Self {
            content,
            is_correct,
            uuid,
            style: Style::default(),
            block: Block::default().borders(Borders::ALL),
        }
    }

    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    #[must_use]
    pub fn block(mut self, block: Block<'static>) -> Self {
        self.block = block;
        self
    }
}

impl From<ChoiceCensored> for ChoiceItem {
    fn from(value: ChoiceCensored) -> Self {
        Self::new(value.text, false, value.id)
    }
}

impl From<Choice> for ChoiceItem {
    fn from(value: Choice) -> Self {
        Self::new(value.text, value.is_correct, value.id)
    }
}

impl Styled for ChoiceItem {
    type Item = ChoiceItem;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self::Item {
        self.style = style;
        self
    }
}

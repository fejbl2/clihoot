use ratatui::style::{Style, Styled};
use ratatui::widgets::{Block, Borders};
use std::cmp::PartialEq;
use uuid::Uuid;

use crate::questions::{Choice, ChoiceCensored};

#[derive(Debug, Clone, Default, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct Item {
    pub(super) content: String,
    pub(super) is_correct: bool,
    pub(super) uuid: Uuid,
    pub(super) style: Style,
    pub(super) block: Block<'static>,
}

impl Item {
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

    pub fn set_style_ref(&mut self, style: Style) {
        self.style = style;
    }

    pub fn set_block_ref(&mut self, block: Block<'static>) {
        self.block = block;
    }

    #[must_use]
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }
}

impl From<ChoiceCensored> for Item {
    fn from(value: ChoiceCensored) -> Self {
        Self::new(value.text, false, value.id)
    }
}

impl From<Choice> for Item {
    fn from(value: Choice) -> Self {
        Self::new(value.text, value.is_correct, value.id)
    }
}

impl Styled for Item {
    type Item = Item;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(mut self, style: Style) -> Self::Item {
        self.style = style;
        self
    }
}

use std::cmp::PartialEq;

use crate::questions::{Question, QuestionCensored};

use crate::terminal::widgets::choice::ChoiceItem;

#[derive(Debug, PartialEq, Clone)]
pub struct ChoiceGrid {
    pub(super) items: Vec<Vec<Option<ChoiceItem>>>,
    is_empty: bool,
}

impl Default for ChoiceGrid {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            is_empty: true,
        }
    }
}

impl ChoiceGrid {
    pub fn new(items: Vec<Vec<Option<ChoiceItem>>>) -> Self {
        let is_empty = items.is_empty()
            || items.iter().any(std::vec::Vec::is_empty)
            || !items.iter().flatten().any(Option::is_some);

        Self { items, is_empty }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    // consume self and return the items inside
    // usefull when wanting to change the grid or items inside
    #[must_use]
    pub fn items(self) -> Vec<Vec<Option<ChoiceItem>>> {
        self.items
    }
}

fn create_grid(items: Vec<ChoiceItem>) -> Vec<Vec<Option<ChoiceItem>>> {
    let mut items: Vec<_> = items.into_iter().map(Some).collect();
    if items.len() % 2 != 0 {
        items.push(None);
    }

    items
        .chunks(2)
        .map(<[Option<ChoiceItem>]>::to_vec)
        .collect()
}

impl From<QuestionCensored> for ChoiceGrid {
    fn from(value: QuestionCensored) -> Self {
        let items: Vec<ChoiceItem> = value.choices.into_iter().map(From::from).collect();
        Self::new(create_grid(items))
    }
}

impl From<Question> for ChoiceGrid {
    fn from(value: Question) -> Self {
        let items: Vec<ChoiceItem> = value.choices.into_iter().map(From::from).collect();
        Self::new(create_grid(items))
    }
}

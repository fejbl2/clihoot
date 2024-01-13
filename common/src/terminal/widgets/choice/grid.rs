use std::cmp::PartialEq;

use crate::questions::{Question, QuestionCensored};

use crate::terminal::widgets::choice::Item;

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct Grid {
    pub(super) items: Vec<Vec<Option<Item>>>,
    is_empty: bool,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            is_empty: true,
        }
    }
}

impl Grid {
    pub fn new(items: Vec<Vec<Option<Item>>>) -> Self {
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
    // useful when wanting to change the grid or items inside
    #[must_use]
    pub fn items(self) -> Vec<Vec<Option<Item>>> {
        self.items
    }
}

fn create_grid(items: Vec<Item>) -> Vec<Vec<Option<Item>>> {
    let mut items: Vec<_> = items.into_iter().map(Some).collect();
    if items.len() % 2 != 0 {
        items.push(None);
    }

    items.chunks(2).map(<[Option<Item>]>::to_vec).collect()
}

impl From<QuestionCensored> for Grid {
    fn from(value: QuestionCensored) -> Self {
        let items: Vec<Item> = value.choices.into_iter().map(From::from).collect();
        Self::new(create_grid(items))
    }
}

impl From<Question> for Grid {
    fn from(value: Question) -> Self {
        let items: Vec<Item> = value.choices.into_iter().map(From::from).collect();
        Self::new(create_grid(items))
    }
}

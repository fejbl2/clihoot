use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap};
use std::collections::HashSet;
use uuid::Uuid;

use crate::questions::{Choice, ChoiceCensored, Question, QuestionCensored};

#[derive(Debug, Clone, Default)]
pub struct ChoiceItem {
    content: String,
    is_right: bool,
    uuid: Uuid,
    style: Style,
}

impl ChoiceItem {
    pub fn new(content: String, is_right: bool, uuid: Uuid) -> Self {
        Self {
            content,
            is_right,
            uuid,
            style: Style::default(),
        }
    }
}

impl From<ChoiceCensored> for ChoiceItem {
    fn from(value: ChoiceCensored) -> Self {
        Self::new(value.text, false, value.id)
    }
}

impl From<Choice> for ChoiceItem {
    fn from(value: Choice) -> Self {
        Self::new(value.text, value.is_right, value.id)
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

#[derive(Default, Debug)]
pub struct ChoiceGrid {
    pub items: Vec<Vec<ChoiceItem>>,
}

fn create_grid(items: Vec<ChoiceItem>) -> Vec<Vec<ChoiceItem>> {
    if items.len() <= 3 {
        return vec![items];
    }

    items.chunks(2).map(|chunk| chunk.to_vec()).collect()
}

impl From<QuestionCensored> for ChoiceGrid {
    fn from(value: QuestionCensored) -> Self {
        let items: Vec<ChoiceItem> = value.choices.into_iter().map(From::from).collect();
        Self {
            items: create_grid(items),
        }
    }
}

impl From<Question> for ChoiceGrid {
    fn from(value: Question) -> Self {
        let items: Vec<ChoiceItem> = value.choices.into_iter().map(From::from).collect();
        Self {
            items: create_grid(items),
        }
    }
}

#[derive(Debug, Default)]
pub struct ChoiceSelectorState {
    row: usize,
    col: usize,
    selected: HashSet<Uuid>,
    last_under_cursor: Option<Uuid>,
}

impl ChoiceSelectorState {
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn last_under_cursor(&self) -> Option<Uuid> {
        self.last_under_cursor
    }

    // update `row` and `col` in case the configuration of the grid has changed
    fn move_to_last_known_choice(&mut self, grid: &ChoiceGrid) {
        let Some(last_under_cursor) = self.last_under_cursor else {
            return;
        };

        if self.row < grid.items.len()
            && self.col < grid.items[self.row].len()
            && grid.items[self.row][self.col].uuid == last_under_cursor
        {
            return;
        }

        self.row = 0;
        self.col = 0;

        for (i, row) in grid.items.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if item.uuid == last_under_cursor {
                    self.row = i;
                    self.col = j;
                }
            }
        }
    }

    // place the cursor to the to the last item in the row if it is out of bounds
    // useful moving up/down and the rows dont have the same ammount of items
    fn normalize_cursor(&mut self, grid: &ChoiceGrid) {
        let row_len = grid.items[self.row].len();
        if self.col >= row_len {
            self.col = row_len - 1
        }
    }

    pub fn move_up(&mut self, grid: &ChoiceGrid) {
        self.move_to_last_known_choice(grid);
        if self.row == 0 {
            self.row = grid.items.len() - 1;
        } else {
            self.row -= 1;
        }

        self.normalize_cursor(grid);
        self.last_under_cursor = Some(grid.items[self.row][self.col].uuid);
    }

    pub fn move_down(&mut self, grid: &ChoiceGrid) {
        self.move_to_last_known_choice(grid);
        self.row = (self.row + 1) % grid.items.len();

        self.normalize_cursor(grid);
        self.last_under_cursor = Some(grid.items[self.row][self.col].uuid);
    }

    pub fn move_left(&mut self, grid: &ChoiceGrid) {
        self.move_to_last_known_choice(grid);
        let row_len = grid.items[self.row].len();
        if self.col == 0 {
            self.col = row_len - 1;
        } else {
            self.col -= 1;
        }
        self.last_under_cursor = Some(grid.items[self.row][self.col].uuid);
    }

    pub fn move_right(&mut self, grid: &ChoiceGrid) {
        self.move_to_last_known_choice(grid);
        let row_len = grid.items[self.row].len();
        self.col = (self.col + 1) % row_len;
        self.last_under_cursor = Some(grid.items[self.row][self.col].uuid);
    }

    // get selected answers as vector
    pub fn selected(&self) -> Vec<Uuid> {
        self.selected.clone().into_iter().collect()
    }

    pub fn toggle_selection(&mut self, grid: &ChoiceGrid) {
        self.move_to_last_known_choice(grid);

        let item = grid.items[self.row][self.col].uuid;

        if self.selected.contains(&item) {
            self.selected.remove(&item);
        } else {
            self.selected.insert(item);
        }
    }
}

#[derive(Default)]
pub struct ChoiceSelector<'a> {
    grid: ChoiceGrid,
    pub block: Option<Block<'a>>,
    pub current_item_style: Style,
    pub selected_item_style: Style,
    pub right_item_style: Style,
}

impl<'a> ChoiceSelector<'a> {
    pub fn new(grid: ChoiceGrid) -> Self {
        Self {
            grid,
            block: None,
            current_item_style: Style::default().italic(),
            selected_item_style: Style::default().bold(),
            right_item_style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn current_item_style(mut self, style: Style) -> Self {
        self.current_item_style = style;
        self
    }

    pub fn selected_item_style(mut self, style: Style) -> Self {
        self.selected_item_style = style;
        self
    }

    pub fn right_item_style(mut self, style: Style) -> Self {
        self.right_item_style = style;
        self
    }
}

impl<'a> StatefulWidget for ChoiceSelector<'a> {
    type State = ChoiceSelectorState;

    // make sure that the ChoiceSelectorState is used with the same grid as the
    // ChoiceSelector that is is used to draw
    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let choice_selector_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let items = &mut self.grid.items;

        let item_height = choice_selector_area.height / items.len() as u16;
        let (x, y) = (choice_selector_area.x, choice_selector_area.y);

        for (i, row) in items.iter_mut().enumerate() {
            let item_width = choice_selector_area.width / row.len() as u16;

            for (j, item) in row.iter_mut().enumerate() {
                let area = Rect::new(
                    x + j as u16 * item_width,
                    y + i as u16 * item_height,
                    item_width,
                    item_height,
                );

                let mut style = item.style;
                if state.row == i && state.col == j {
                    style = style.patch(self.current_item_style);
                }
                if state.selected.contains(&item.uuid) {
                    style = style.patch(self.selected_item_style);
                }
                if item.is_right {
                    style = style.patch(self.right_item_style);
                }

                Paragraph::new(item.content.clone())
                    .block(Block::default().borders(Borders::ALL))
                    .style(style)
                    .wrap(Wrap { trim: true })
                    .render(area, buf);
            }
        }
    }
}

impl<'a> Widget for ChoiceSelector<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ChoiceSelectorState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

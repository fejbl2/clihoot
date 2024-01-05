use ratatui::layout::Alignment;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap};
use std::cmp::PartialEq;
use std::collections::HashSet;
use uuid::Uuid;

use crate::questions::{Choice, ChoiceCensored, Question, QuestionCensored};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChoiceItem {
    content: String,
    is_correct: bool,
    uuid: Uuid,
    style: Style,
}

impl ChoiceItem {
    pub fn new(content: String, is_correct: bool, uuid: Uuid) -> Self {
        Self {
            content,
            is_correct,
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

impl ChoiceItem {
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct ChoiceGrid {
    items: Vec<Vec<ChoiceItem>>,
    is_empty: bool,
}

impl ChoiceGrid {
    pub fn new(items: Vec<Vec<ChoiceItem>>) -> Self {
        let is_empty = items.is_empty() || items.iter().any(|row| row.is_empty());

        Self { items, is_empty }
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    // consume self and return the items inside
    // usefull when wanting to change the grid or items inside
    pub fn items(self) -> Vec<Vec<ChoiceItem>> {
        self.items
    }
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
        Self::new(create_grid(items))
    }
}

impl From<Question> for ChoiceGrid {
    fn from(value: Question) -> Self {
        let items: Vec<ChoiceItem> = value.choices.into_iter().map(From::from).collect();
        Self::new(create_grid(items))
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
    // gets called before all other methods that mutate the state
    pub fn move_to_last_known_choice(&mut self, grid: &ChoiceGrid) {
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
                    return;
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
        if grid.is_empty() {
            return;
        }

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
        if grid.is_empty() {
            return;
        }

        self.move_to_last_known_choice(grid);
        self.row = (self.row + 1) % grid.items.len();

        self.normalize_cursor(grid);
        self.last_under_cursor = Some(grid.items[self.row][self.col].uuid);
    }

    pub fn move_left(&mut self, grid: &ChoiceGrid) {
        if grid.is_empty() {
            return;
        }

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
        if grid.is_empty() {
            return;
        }

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
        if grid.is_empty() {
            return;
        }

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
    pub horizontal_gap: u16,
    pub vertical_gap: u16,
}

impl<'a> ChoiceSelector<'a> {
    pub fn new(grid: ChoiceGrid) -> Self {
        Self {
            grid,
            block: None,
            current_item_style: Style::default().italic(),
            selected_item_style: Style::default().bold(),
            right_item_style: Style::default(),
            horizontal_gap: 0,
            vertical_gap: 0,
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

    pub fn horizontal_gap(mut self, gap: u16) -> Self {
        self.horizontal_gap = gap;
        self
    }

    pub fn vertical_gap(mut self, gap: u16) -> Self {
        self.vertical_gap = gap;
        self
    }

    pub fn gap(mut self, gap: u16) -> Self {
        self.horizontal_gap = gap;
        self.vertical_gap = gap;
        self
    }
}

impl<'a> StatefulWidget for ChoiceSelector<'a> {
    type State = ChoiceSelectorState;

    // when rendering the widget, make sure that the ChoiceSelectorState is used
    // with the same grid that is used for the rendering
    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let choice_selector_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        // don't try to draw anything if the grid is empty or has empty rows
        if self.grid.is_empty() {
            return;
        }

        let items = &mut self.grid.items;

        let mut ignore_vgaps = false;
        let total_vgap_size = self.vertical_gap * (items.len() as u16 - 1);
        let item_height = if total_vgap_size >= choice_selector_area.height {
            // ignore the gaps if they are too big
            ignore_vgaps = true;
            choice_selector_area.height / items.len() as u16
        } else {
            (choice_selector_area.height - total_vgap_size) / items.len() as u16
        };

        let (x, mut y) = (choice_selector_area.x, choice_selector_area.y);

        for (i, row) in items.iter_mut().enumerate() {
            let mut ignore_hgaps = false;
            let total_hgap_size = self.horizontal_gap * (row.len() as u16 - 1);
            let item_width = if total_hgap_size >= choice_selector_area.width {
                // ignore the gaps if they are too big
                ignore_hgaps = true;
                choice_selector_area.width / row.len() as u16
            } else {
                (choice_selector_area.width - total_hgap_size) / row.len() as u16
            };

            let mut row_x = x;
            for (j, item) in row.iter_mut().enumerate() {
                let area = Rect::new(
                    row_x + j as u16 * item_width,
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
                if item.is_correct {
                    style = style.patch(self.right_item_style);
                }

                let text = Text::from(item.content.clone());
                let text_height = text.height() as u16 + 2;
                // centering the text vertically
                let leftover_vertical_space = area.height.saturating_sub(text_height);
                Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).padding(
                        ratatui::widgets::Padding::new(0, 0, leftover_vertical_space / 2, 0),
                    ))
                    .style(style)
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center)
                    .render(area, buf);

                if !ignore_hgaps {
                    row_x += self.horizontal_gap;
                }
            }

            if !ignore_vgaps {
                y += self.vertical_gap;
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

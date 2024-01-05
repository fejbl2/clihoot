use std::collections::HashSet;
use uuid::Uuid;

use crate::terminal::widgets::choice::ChoiceGrid;

#[derive(Debug, Default)]
pub struct ChoiceSelectorState {
    row: usize,
    col: usize,
    pub(super) selected: HashSet<Uuid>,
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

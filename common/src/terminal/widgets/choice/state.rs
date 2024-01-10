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
    #[must_use]
    pub fn row(&self) -> usize {
        self.row
    }

    #[must_use]
    pub fn col(&self) -> usize {
        self.col
    }

    #[must_use]
    pub fn last_under_cursor(&self) -> Option<Uuid> {
        self.last_under_cursor
    }

    // update `row` and `col` in case the configuration of the grid has changed
    // gets called before all other methods that mutate the state
    pub fn move_to_last_known_choice(&mut self, grid: &ChoiceGrid) {
        let Some(last_under_cursor) = self.last_under_cursor else {
            return;
        };

        if self.row < grid.items.len() && self.col < grid.items[self.row].len() {
            if let Some(item) = &grid.items[self.row][self.col] {
                if item.uuid == last_under_cursor {
                    return;
                }
            }
        }

        self.row = 0;
        self.col = 0;

        for (i, row) in grid.items.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                let Some(item) = item else {
                    continue;
                };

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
            self.col = row_len - 1;
        }
    }

    fn find_next_some_right(&mut self, grid: &ChoiceGrid) -> bool {
        let row_len = grid.items[self.row].len();
        for _ in 0..row_len {
            self.col = (self.col + 1) % row_len;

            if let Some(item) = &grid.items[self.row][self.col] {
                self.last_under_cursor = Some(item.uuid);
                return true;
            }
        }
        false
    }

    fn find_next_some_left(&mut self, grid: &ChoiceGrid) -> bool {
        let row_len = grid.items[self.row].len();
        for _ in 0..row_len {
            if self.col == 0 {
                self.col = row_len - 1;
            } else {
                self.col -= 1;
            }

            if let Some(item) = &grid.items[self.row][self.col] {
                self.last_under_cursor = Some(item.uuid);
                return true;
            }
        }
        false
    }

    // move up a row in the grid. If the item above is None, find closest Some to the left
    // of the original or continue moving up, until any Some is found
    // if on the first line and move_up is used, move to the last line.
    pub fn move_up(&mut self, grid: &ChoiceGrid) {
        if grid.is_empty() {
            return;
        }

        self.move_to_last_known_choice(grid);
        for _ in 0..grid.items.len() {
            if self.row == 0 {
                self.row = grid.items.len() - 1;
            } else {
                self.row -= 1;
            }

            self.normalize_cursor(grid);
            if let Some(item) = &grid.items[self.row][self.col] {
                self.last_under_cursor = Some(item.uuid);
                return;
            }

            if self.find_next_some_left(grid) {
                return;
            }
        }
    }

    pub fn move_down(&mut self, grid: &ChoiceGrid) {
        if grid.is_empty() {
            return;
        }

        self.move_to_last_known_choice(grid);
        for _ in 0..grid.items.len() {
            self.row = (self.row + 1) % grid.items.len();

            self.normalize_cursor(grid);
            if let Some(item) = &grid.items[self.row][self.col] {
                self.last_under_cursor = Some(item.uuid);
                return;
            }
            if self.find_next_some_right(grid) {
                return;
            }
        }
    }

    // move left. If the next item is None, find the closest Some to the left.
    // If at the start at the line and move_left is used, move to the end of the line.
    pub fn move_left(&mut self, grid: &ChoiceGrid) {
        if grid.is_empty() {
            return;
        }

        self.move_to_last_known_choice(grid);

        if !self.find_next_some_left(grid) {
            // if only None values are on the line, move up
            self.move_up(grid);
        }
    }

    pub fn move_right(&mut self, grid: &ChoiceGrid) {
        if grid.is_empty() {
            return;
        }

        self.move_to_last_known_choice(grid);

        if !self.find_next_some_right(grid) {
            // if only None values are on the line, move down
            self.move_down(grid);
        }
    }

    // get selected answers as vector
    #[must_use]
    pub fn selected(&self) -> Vec<Uuid> {
        self.selected.clone().into_iter().collect()
    }

    pub fn toggle_selection(&mut self, grid: &ChoiceGrid, is_multichoice: bool) {
        if grid.is_empty() {
            return;
        }

        self.move_to_last_known_choice(grid);

        let Some(item) = &grid.items[self.row][self.col] else {
            return;
        };

        let id = item.uuid;
        if self.selected.contains(&id) {
            self.selected.remove(&id);
        } else {
            if !is_multichoice {
                self.selected.clear();
            }
            self.selected.insert(id);
        }
    }
}

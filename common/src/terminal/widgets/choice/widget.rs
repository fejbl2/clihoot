use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Padding, Paragraph, StatefulWidget, Widget, Wrap};

use crate::terminal::widgets::choice::{Grid, SelectorState};

#[derive(Default, Clone)]
pub struct Selector<'a> {
    grid: Grid,
    block: Option<Block<'a>>,
    current_item_style: Style,
    current_item_block: Option<Block<'a>>,
    selected_item_style: Style,
    selected_item_block: Option<Block<'a>>,
    correct_item_style: Style,
    correct_item_block: Option<Block<'a>>,
    horizontal_gap: u16,
    vertical_gap: u16,
    max_width_percentage: u8,
}

impl<'a> Selector<'a> {
    #[must_use]
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            block: None,
            current_item_style: Style::default().italic(),
            current_item_block: None,
            selected_item_style: Style::default().bold(),
            selected_item_block: None,
            correct_item_style: Style::default(),
            correct_item_block: None,
            horizontal_gap: 0,
            vertical_gap: 0,
            max_width_percentage: 100,
        }
    }

    #[must_use]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    #[must_use]
    pub fn current_item_style(mut self, style: Style) -> Self {
        self.current_item_style = style;
        self
    }

    #[must_use]
    pub fn current_item_block(mut self, block: Block<'a>) -> Self {
        self.current_item_block = Some(block);
        self
    }

    #[must_use]
    pub fn selected_item_style(mut self, style: Style) -> Self {
        self.selected_item_style = style;
        self
    }

    #[must_use]
    pub fn selected_item_block(mut self, block: Block<'a>) -> Self {
        self.selected_item_block = Some(block);
        self
    }

    #[must_use]
    pub fn correct_item_style(mut self, style: Style) -> Self {
        self.correct_item_style = style;
        self
    }

    #[must_use]
    pub fn correct_item_block(mut self, block: Block<'a>) -> Self {
        self.correct_item_block = Some(block);
        self
    }

    #[must_use]
    pub fn horizontal_gap(mut self, gap: u16) -> Self {
        self.horizontal_gap = gap;
        self
    }

    #[must_use]
    pub fn vertical_gap(mut self, gap: u16) -> Self {
        self.vertical_gap = gap;
        self
    }

    #[must_use]
    pub fn gap(mut self, gap: u16) -> Self {
        self.horizontal_gap = gap;
        self.vertical_gap = gap;
        self
    }

    // maximum percentage of the row width, that one choice item can take
    #[must_use]
    pub fn max_width_percentage(mut self, max_width_percentage: u8) -> Self {
        self.max_width_percentage = max_width_percentage;
        self
    }
}

fn calculate_size(available_size: u16, item_count: u16, total_gap_size: u16, max_size: u16) -> u16 {
    let size = (available_size - total_gap_size) / item_count;

    if size > max_size {
        max_size
    } else {
        size
    }
}

impl<'a> StatefulWidget for Selector<'a> {
    type State = SelectorState;

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

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let max_width = (f32::from(choice_selector_area.width)
            * (f32::from(self.max_width_percentage) / 100.0))
            .round() as u16;

        let items = &mut self.grid.items;

        let mut total_vertical_gap_size =
            self.vertical_gap * (u16::try_from(items.len()).unwrap_or(u16::MAX) - 1);
        if total_vertical_gap_size >= choice_selector_area.height {
            total_vertical_gap_size = 0;
        }
        let item_height = calculate_size(
            choice_selector_area.height,
            u16::try_from(items.len()).unwrap_or(u16::MAX),
            total_vertical_gap_size,
            choice_selector_area.height,
        );

        let (x, mut y) = (choice_selector_area.x, choice_selector_area.y);

        for (i, row) in items.iter_mut().enumerate() {
            let mut total_horizontal_gap_size =
                self.horizontal_gap * (u16::try_from(row.len()).unwrap_or(u16::MAX) - 1);
            if total_horizontal_gap_size > choice_selector_area.width {
                total_horizontal_gap_size = 0;
            }

            let item_width = calculate_size(
                choice_selector_area.width,
                u16::try_from(row.len()).unwrap_or(u16::MAX),
                total_horizontal_gap_size,
                max_width,
            );

            let leftover_space = choice_selector_area.width
                - (item_width * u16::try_from(row.len()).unwrap_or(u16::MAX))
                - total_horizontal_gap_size;

            let mut row_x = x + leftover_space / 2;
            for (j, item) in row.iter_mut().enumerate() {
                let area = Rect::new(
                    row_x + u16::try_from(j).unwrap_or_default() * item_width,
                    y + u16::try_from(i).unwrap_or_default() * item_height,
                    item_width,
                    item_height,
                );

                if total_horizontal_gap_size > 0 {
                    row_x += self.horizontal_gap;
                }

                let Some(item) = item else {
                    continue;
                };

                let selected = state.selected.contains(&item.uuid);
                let current = state.row() == i && state.col() == j;
                let correct = item.is_correct;

                let mut style = item.style;
                if selected {
                    style = style.patch(self.selected_item_style);
                }
                if current {
                    style = style.patch(self.current_item_style);
                }

                if correct {
                    style = style.patch(self.correct_item_style);
                }

                let block = if self.current_item_block.is_some() && current {
                    self.current_item_block.clone().take()
                } else if self.selected_item_block.is_some() && selected {
                    self.selected_item_block.clone().take()
                } else if self.correct_item_block.is_some() && correct {
                    self.correct_item_block.clone().take()
                } else {
                    None
                };

                let block = block.unwrap_or(item.block.clone());

                let text = Text::from(item.content.clone());
                let text_height = u16::try_from(text.height()).unwrap_or_default() + 2;
                // centering the text vertically
                let padding = Padding::new(0, 0, area.height.saturating_sub(text_height) / 2, 0);

                Paragraph::new(text)
                    .block(block.padding(padding))
                    .style(style)
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center)
                    .render(area, buf);
            }

            if total_vertical_gap_size > 0 {
                y += self.vertical_gap;
            }
        }
    }
}

impl<'a> Widget for Selector<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = SelectorState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

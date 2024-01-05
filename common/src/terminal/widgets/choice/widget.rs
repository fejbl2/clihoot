use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Padding, Paragraph, StatefulWidget, Widget, Wrap};

use crate::terminal::widgets::choice::{ChoiceGrid, ChoiceSelectorState};

#[derive(Default)]
pub struct ChoiceSelector<'a> {
    grid: ChoiceGrid,
    block: Option<Block<'a>>,
    current_item_style: Style,
    selected_item_style: Style,
    right_item_style: Style,
    horizontal_gap: u16,
    vertical_gap: u16,
    max_width_percentage: u8,
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
            max_width_percentage: 100,
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

    // maximum percentage of the row width, that one choice item can take
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

        let max_width = (choice_selector_area.width as f32
            * (self.max_width_percentage as f32 / 100.0))
            .round() as u16;

        let items = &mut self.grid.items;

        let mut total_vgap_size = self.vertical_gap * (items.len() as u16 - 1);
        if total_vgap_size >= choice_selector_area.height {
            total_vgap_size = 0;
        }
        let item_height = calculate_size(
            choice_selector_area.height,
            items.len() as u16,
            total_vgap_size,
            choice_selector_area.height,
        );

        let (x, mut y) = (choice_selector_area.x, choice_selector_area.y);

        for (i, row) in items.iter_mut().enumerate() {
            let mut total_hgap_size = self.horizontal_gap * (row.len() as u16 - 1);
            if total_hgap_size > choice_selector_area.width {
                total_hgap_size = 0;
            }

            let item_width = calculate_size(
                choice_selector_area.width,
                row.len() as u16,
                total_hgap_size,
                max_width,
            );

            let leftover_space =
                choice_selector_area.width - (item_width * row.len() as u16) - total_hgap_size;

            let mut row_x = x + leftover_space / 2;
            for (j, item) in row.iter_mut().enumerate() {
                let area = Rect::new(
                    row_x + j as u16 * item_width,
                    y + i as u16 * item_height,
                    item_width,
                    item_height,
                );

                if total_hgap_size > 0 {
                    row_x += self.horizontal_gap;
                }

                let Some(item) = item else {
                    continue;
                };

                let mut style = item.style;
                if state.row() == i && state.col() == j {
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
                let padding = Padding::new(0, 0, area.height.saturating_sub(text_height) / 2, 0);

                Paragraph::new(text)
                    .block(item.block.clone().padding(padding))
                    .style(style)
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center)
                    .render(area, buf);
            }

            if total_vgap_size > 0 {
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

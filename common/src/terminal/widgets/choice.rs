use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, StatefulWidget, Widget, Wrap};

pub struct ChoiceItem {
    string: String,
    style: Style,
}

impl ChoiceItem {
    pub fn new(string: String) -> Self {
        Self {
            string,
            style: Style::default(),
        }
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

#[derive(Default)]
pub struct ChoiceSelectorState {
    pub row: usize,
    pub col: usize,
}

impl ChoiceSelectorState {
    fn normalize_state(&mut self, items: &Vec<Vec<ChoiceItem>>) {
        if self.row >= items.len() {
            self.row = 0;
        }

        let row_len = items[self.row].len();
        if self.col >= row_len {
            self.col = 0
        }
    }
}

pub struct ChoiceSelector<'a> {
    pub block: Option<Block<'a>>,
    pub items: Vec<Vec<ChoiceItem>>,
}

impl<'a> ChoiceSelector<'a> {
    pub fn new(items: Vec<Vec<ChoiceItem>>) -> Self {
        Self { block: None, items }
    }
}

impl<'a> ChoiceSelector<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
}

impl<'a> StatefulWidget for ChoiceSelector<'a> {
    type State = ChoiceSelectorState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // normalize the state before drawing the widget
        state.normalize_state(&self.items);

        let choice_selector_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let item_height = choice_selector_area.height / self.items.len() as u16;
        let (x, y) = (choice_selector_area.x, choice_selector_area.y);

        for (i, row) in self.items.iter_mut().enumerate() {
            let item_width = choice_selector_area.width / row.len() as u16;

            for (j, item) in row.iter_mut().enumerate() {
                let area = Rect::new(
                    x + j as u16 * item_width,
                    y + i as u16 * item_height,
                    item_width,
                    item_height,
                );

                let paragraph = Paragraph::new(item.string.clone())
                    .block(Block::default().borders(Borders::ALL))
                    .style(item.style)
                    .wrap(Wrap { trim: true });

                if state.row == i && state.col == j {
                    paragraph.add_modifier(Modifier::ITALIC).render(area, buf);
                } else {
                    paragraph.render(area, buf);
                }
            }
        }
    }
}

impl<'a> Widget for ChoiceSelector<'a> {
    // just draw the choices in the grid without changing the state
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ChoiceSelectorState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

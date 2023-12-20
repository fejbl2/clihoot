use crate::terminal::student::{Color, StudentTerminal, StudentTerminalState};
use common::terminal::terminal_actor::TerminalDraw;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

const COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

impl TerminalDraw for StudentTerminal {
    fn redraw(
        &mut self,
        term: &mut ratatui::prelude::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
    ) -> anyhow::Result<()> {
        // TODO define function that would do the drawing
        match &mut self.state {
            StudentTerminalState::NameSelection { name } => {
                term.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new(format!("Name: {name}|")).block(
                            Block::default()
                                .title("Write your name")
                                .borders(Borders::ALL),
                        ),
                        frame.size(),
                    );
                })?;
            }
            StudentTerminalState::ColorSelection { list_state } => {
                term.draw(|frame| {
                    let default_block = Block::default()
                        .title("Select your color")
                        .borders(Borders::ALL);

                    // TOOD constant for this
                    let items: Vec<_> = COLORS
                        .iter()
                        .map(|color| ListItem::new(format!("{color:?}")))
                        .collect();

                    frame.render_stateful_widget(
                        List::new(items)
                            .block(default_block)
                            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                            .highlight_symbol(">> "),
                        frame.size(),
                        list_state,
                    );
                })?;
            }
            StudentTerminalState::Todo => {
                term.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new(format!(
                            "Your name is \"{}\" and your color is \"{:?}\".",
                            self.name, self.color
                        ))
                        .block(Block::default().title("Greeting").borders(Borders::ALL)),
                        frame.size(),
                    );
                })?;
            }
        }
        Ok(())
    }
}

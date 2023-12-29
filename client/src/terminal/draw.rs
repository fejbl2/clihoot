use crate::terminal::draw_states::{
    render_color_selection, render_name_selection, render_waiting, render_welcome,
};
use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::terminal::terminal_actor::TerminalDraw;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

impl TerminalDraw for StudentTerminal {
    fn redraw(
        &mut self,
        term: &mut ratatui::prelude::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,
    ) -> anyhow::Result<()> {
        // TODO define function that would do the drawing
        match &mut self.state {
            StudentTerminalState::StartGame => {
                term.draw(|frame| {
                    render_welcome(frame);
                })?;
                Ok(())
            }
            StudentTerminalState::NameSelection { name } => {
                term.draw(|frame| {
                    render_name_selection(frame, name);
                })?;
                Ok(())
            }
            StudentTerminalState::ColorSelection { list_state } => {
                term.draw(|frame| {
                    render_color_selection(frame, self.color, list_state);
                })?;
                Ok(())
            }
            StudentTerminalState::WaitingForGame { players } => {
                term.draw(|frame| {
                    render_waiting(frame, players);
                })?;
                Ok(())
            }
            _ => {
                term.draw(|frame| {
                    frame.render_widget(
                        Paragraph::new("Not implemented yet")
                            .block(Block::default().title("Error").borders(Borders::ALL)),
                        frame.size(),
                    );
                })?;
                Ok(())
            }
        }
    }
}

use actix::prelude::*;

use common::{messages::network::PlayerData, terminal::highlight::Theme};

use crate::{teacher::state::TeacherTerminalState, Lobby};

pub struct TeacherTerminal {
    pub quiz_name: String,
    pub lobby: Addr<Lobby>,
    pub players: Vec<PlayerData>,
    pub help_visible: bool,
    pub state: TeacherTerminalState,
    pub syntax_theme: Theme,
}

impl TeacherTerminal {
    #[must_use]
    pub fn new(quiz_name: String, lobby: Addr<Lobby>, syntax_theme: Theme) -> Self {
        Self {
            quiz_name,
            lobby,
            players: Vec::new(),
            help_visible: false,
            state: TeacherTerminalState::StartGame,
            syntax_theme,
        }
    }
}

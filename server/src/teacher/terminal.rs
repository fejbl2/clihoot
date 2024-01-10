use actix::prelude::*;

use ratatui::widgets::{ListState, TableState};

use common::{
    messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard},
    terminal::highlight::Theme,
};

use crate::Lobby;

#[derive(Debug)]
pub enum TeacherTerminalState {
    StartGame,
    WaitingForGame {
        list_state: ListState,
        kick_popup_visible: bool,
    },
    Question {
        question: NextQuestion,
        players_answered_count: usize,
        start_time: chrono::DateTime<chrono::Utc>,
        duration_from_start: chrono::Duration,
    },
    Answers {
        answers: QuestionEnded,
    },
    Results {
        results: ShowLeaderboard,
        table_state: TableState,
        kick_popup_visible: bool,
    },
    EndGame,
    Error {
        message: String,
    },
}

pub struct TeacherTerminal {
    pub quiz_name: String,
    pub lobby: Addr<Lobby>,
    pub players: Vec<PlayerData>,
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
            state: TeacherTerminalState::StartGame,
            syntax_theme,
        }
    }
}

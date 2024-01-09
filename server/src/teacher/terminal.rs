use actix::prelude::*;

use ratatui::widgets::{ListState, TableState};

use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};

use crate::Lobby;

#[derive(Debug)]
pub enum TeacherTerminalState {
    StartGame,
    WaitingForGame {
        list_state: ListState,
    },
    Question {
        question: NextQuestion,
        players_answered_count: usize,
        start_time: chrono::DateTime<chrono::Utc>,
        duration_from_start: chrono::Duration,
    },
    Answers {
        answers: QuestionEnded,
        list_state: ListState,
    },
    Results {
        results: ShowLeaderboard,
        table_state: TableState,
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
}

impl TeacherTerminal {
    #[must_use]
    pub fn new(quiz_name: String, lobby: Addr<Lobby>) -> Self {
        Self {
            quiz_name,
            lobby,
            players: Vec::new(),
            state: TeacherTerminalState::StartGame,
        }
    }
}

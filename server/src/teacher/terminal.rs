use actix::prelude::*;

use ratatui::widgets::ListState;

use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};

use crate::lobby::state::Lobby;

#[derive(Debug)]
pub enum TeacherTerminalState {
    StartGame,
    WaitingForGame {
        list_state: ListState,
        players: Vec<PlayerData>,
    },
    Question {
        question: NextQuestion,
        players_answered_count: usize,
        answered: bool,
        players: Vec<PlayerData>,
    },
    Answers {
        answers: QuestionEnded,
        players: Vec<PlayerData>,
    },
    Results {
        results: ShowLeaderboard,
        players: Vec<PlayerData>,
    },
    EndGame,
    Error {
        message: String,
    },
}

pub struct TeacherTerminal {
    pub quiz_name: String,
    pub lobby: Addr<Lobby>,
    pub state: TeacherTerminalState,
}

impl TeacherTerminal {
    #[must_use]
    pub fn new(quiz_name: String, lobby: Addr<Lobby>) -> Self {
        Self {
            quiz_name,
            lobby,
            state: TeacherTerminalState::StartGame,
        }
    }
}

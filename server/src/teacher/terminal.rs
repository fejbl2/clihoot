use actix::prelude::*;

use ratatui::widgets::ListState;

use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};

use crate::Lobby;

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
        players: Vec<PlayerData>,
    },
    Answers {
        answers: QuestionEnded,
        players: Vec<PlayerData>,
        list_state: ListState,
    },
    Results {
        results: ShowLeaderboard,
        list_state: ListState,
    },
    EndGame {
        results: ShowLeaderboard,
        list_state: ListState,
    },
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

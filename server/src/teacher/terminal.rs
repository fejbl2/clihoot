use actix::prelude::*;

use ratatui::widgets::ListState;

use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};
use common::terminal::widgets::choice::ChoiceGrid;

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
    },
    Answers {
        answers: QuestionEnded,
        list_state: ListState,
        choice_grid: ChoiceGrid,
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

impl common::terminal::terminal_actor::TerminalHandleTick for TeacherTerminal {
    fn handle_tick(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

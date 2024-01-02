use actix::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::ListState;
use uuid::Uuid;

use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};

#[derive(Debug)]
pub enum TeacherTerminalState {
    NameSelection {
        name: String,
    },
    ColorSelection {
        list_state: ListState,
    },
    WaitingForGame {
        players: Vec<PlayerData>,
    },
    Question {
        question: NextQuestion,
        players_answered_count: usize,
        answered: bool,
        /*answer_checker_state: Todo*/
    },
    Answers {
        answers: QuestionEnded,
    },
    Results {
        results: ShowLeaderboard,
    },
    EndGame, // show some screen saying that the game ended and the student should just pres ctrl + c to close the app
    Error {
        message: String,
    },
}

pub struct TeacherTerminal {
    pub uuid: Uuid,
    pub name: String,
    pub color: Color,
    pub quiz_name: String,
    pub ws_actor_address: Addr<WebsocketActor>,
    pub state: StudentTerminalState,
    pub music_address: Addr<MusicActor>,
}

impl TeacherTerminal {
    #[must_use]
    pub fn new(
        uuid: Uuid,
        quiz_name: String,
        ws_addr: Addr<WebsocketActor>,
        music_address: Addr<MusicActor>,
    ) -> Self {
        Self {
            uuid,
            name: String::new(),
            color: Color::default(),
            quiz_name,
            ws_actor_address: ws_addr,
            state: StudentTerminalState::NameSelection {
                name: String::new(),
            },
            music_address,
        }
    }
}

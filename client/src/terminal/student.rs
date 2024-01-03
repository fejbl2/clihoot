use actix::prelude::*;
use log::debug;
use ratatui::style::Color;
use ratatui::widgets::ListState;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::music_actor::{MusicActor, MusicMessage};
use crate::websocket::WebsocketActor;
use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};
use common::terminal::handle_terminal_events::handle_events;
use common::terminal::messages::Initialize;
use common::terminal::terminal_actor::{TerminalActor, TerminalStop};

#[derive(Debug)]
pub enum StudentTerminalState {
    StartGame,
    NameSelection {
        name: String,
        name_already_used: bool,
    },
    ColorSelection {
        list_state: ListState,
    },
    WaitingForGame {
        list_state: ListState,
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

pub struct StudentTerminal {
    pub uuid: Uuid,
    pub name: String,
    pub color: Color,
    pub quiz_name: String,
    pub players: Vec<PlayerData>,
    pub ws_actor_address: Addr<WebsocketActor>,
    pub state: StudentTerminalState,
    pub music_address: Addr<MusicActor>,
}

impl StudentTerminal {
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
            players: Vec::new(),
            ws_actor_address: ws_addr,
            state: StudentTerminalState::StartGame,
            music_address,
        }
    }
}

impl TerminalStop for StudentTerminal {
    fn stop(&mut self) -> anyhow::Result<()> {
        debug!("Stopping terminal actor for student");
        Ok(())
    }
}

pub async fn run_student(
    uuid: Uuid,
    quiz_name: String,
    ws_actor_addr: Addr<WebsocketActor>,
    music_actor_addr: Addr<MusicActor>,
) -> anyhow::Result<(
    Addr<TerminalActor<StudentTerminal>>,
    JoinHandle<anyhow::Result<()>>,
)> {
    let term = TerminalActor::new(StudentTerminal::new(
        uuid,
        quiz_name,
        ws_actor_addr,
        music_actor_addr.clone(),
    ))
    .start();

    term.send(Initialize).await??;

    music_actor_addr.do_send(MusicMessage::Lobby);

    let task = tokio::spawn(handle_events(term.clone()));
    Ok((term, task))
}

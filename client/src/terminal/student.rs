use actix::prelude::*;
use ratatui::style::Color;
use ratatui::widgets::ListState;
use tokio::task::JoinHandle;
use uuid::Uuid;

use crate::websocket::WebsocketActor;
use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};
use common::terminal::handle_terminal_events::handle_events;
use common::terminal::messages::Initialize;
use common::terminal::terminal_actor::TerminalActor;

#[derive(Debug)]
pub enum StudentTerminalState {
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

pub struct StudentTerminal {
    pub uuid: Uuid,
    pub name: String,
    pub color: Color,
    pub quiz_name: String,
    pub ws_actor_address: Addr<WebsocketActor>,
    pub state: StudentTerminalState,
}

impl StudentTerminal {
    #[must_use]
    pub fn new(uuid: Uuid, quiz_name: String, ws_addr: Addr<WebsocketActor>) -> Self {
        Self {
            uuid,
            name: String::new(),
            color: Color::default(),
            quiz_name,
            ws_actor_address: ws_addr,
            state: StudentTerminalState::NameSelection {
                name: String::new(),
            },
        }
    }
}

pub async fn run_student(
    uuid: Uuid,
    quiz_name: String,
    ws_actor_addr: Addr<WebsocketActor>,
) -> anyhow::Result<(
    Addr<TerminalActor<StudentTerminal>>,
    JoinHandle<anyhow::Result<()>>,
)> {
    let term = TerminalActor::new(StudentTerminal::new(uuid, quiz_name, ws_actor_addr)).start();

    term.send(Initialize).await??;

    let task = tokio::spawn(handle_events(term.clone()));
    Ok((term, task))
}

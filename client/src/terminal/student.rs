use actix::prelude::*;
use tokio::task::JoinHandle;
use uuid::Uuid;

use common::messages::network::{NextQuestion, PlayerData, QuestionEnded, ShowLeaderboard};
use common::terminal::handle_terminal_events::handle_events;
use common::terminal::messages::Initialize;
use common::terminal::terminal_actor::TerminalActor;
use ratatui::style::Color;
use ratatui::widgets::ListState;

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
        answered: bool, /*state: Todo*/
    },
    Answers {
        answer: QuestionEnded,
    },
    Results {
        results: ShowLeaderboard,
    },
    EndGame, // ????
    Error {
        message: String,
    },
}

pub struct StudentTerminal {
    pub name: String,
    pub color: Color,
    pub quiz_name: String,
    pub state: StudentTerminalState,
}

impl Default for StudentTerminal {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: Color::default(),
            quiz_name: String::new(),
            state: StudentTerminalState::NameSelection {
                name: String::new(),
            },
        }
    }
}

impl StudentTerminal {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

pub async fn run_student(
    _uuid: Uuid,
    _quiz_name: String,
) -> anyhow::Result<(
    Addr<TerminalActor<StudentTerminal>>,
    JoinHandle<anyhow::Result<()>>,
)> {
    let term = TerminalActor::new(StudentTerminal::new()).start();

    term.send(Initialize).await??;

    let task = tokio::spawn(handle_events(term.clone()));
    Ok((term, task))
}

// we can implement handlers for student specific messages:
//
// impl Handler<Foo> for TerminalActor<TerminalActorData> {
//     type Result = anyhow::Result<()>;
//
//     fn handle(&mut self, msg: Foo, ctx: &mut Self::Context) -> Self::Result {
//         ...
//     }
// }

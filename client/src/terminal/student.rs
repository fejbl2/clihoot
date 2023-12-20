use actix::prelude::*;
use tokio::task::JoinHandle;

use common::terminal::handle_terminal_events::handle_events;
use common::terminal::messages::Initialize;
use common::terminal::terminal_actor::TerminalActor;
use ratatui::widgets::ListState;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub enum StudentTerminalState {
    NameSelection { name: String },
    ColorSelection { list_state: ListState },
    Todo,
}

pub struct StudentTerminal {
    pub name: String,
    pub color: Color,
    pub state: StudentTerminalState,
}

impl Default for StudentTerminal {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: Color::Red,
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

pub async fn run_student() -> anyhow::Result<(
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

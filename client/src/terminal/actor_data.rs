use ratatui::widgets::ListState;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

pub enum TerminalActorState {
    NameSelection { name: String },
    ColorSelection { list_state: ListState },
    Todo,
}

pub struct TerminalActorData {
    pub name: String,
    pub color: Color,
    pub state: TerminalActorState,
}

impl Default for TerminalActorData {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: Color::Red,
            state: TerminalActorState::NameSelection {
                name: String::new(),
            },
        }
    }
}

impl TerminalActorData {
    pub fn new() -> Self {
        Self::default()
    }
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

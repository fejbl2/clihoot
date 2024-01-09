use std::sync::mpsc::Sender;

use actix::{prelude::Actor, Addr};

use common::terminal::terminal_actor::TerminalActor;

use crate::{messages::lobby::RegisterTeacher, Lobby};

use super::terminal::TeacherTerminal;

pub type Teacher = TerminalActor<TeacherTerminal>;

pub fn run_teacher(
    lobby: Addr<Lobby>,
    tx: Sender<Addr<Teacher>>,
    quiz_name: &str,
) -> anyhow::Result<()> {
    let system = actix::System::new();

    system.block_on(init(lobby, tx, quiz_name))?;

    system.run()?;

    Ok(())
}

#[allow(clippy::unused_async)]
async fn init(
    lobby: Addr<Lobby>,
    tx: Sender<Addr<Teacher>>,
    quiz_name: &str,
) -> anyhow::Result<()> {
    let teacher = TerminalActor::new(TeacherTerminal::new(
        quiz_name.to_string(),
        lobby.clone(),
        common::terminal::highlight::Theme::Default,
    ))
    .start();

    tx.send(teacher.clone())
        .expect("Failed to send teacher address");

    lobby.do_send(RegisterTeacher {
        teacher: teacher.clone(),
    });

    Ok(())
}

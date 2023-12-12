use actix::{prelude::Message, Addr};

use crate::{server::state::Lobby, teacher::init::Teacher};

/// The teacher sends this to the lobby to set the locked state
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct SetLockMessage {
    pub locked: bool,
}

/// The teacher sends this to the lobby to register itself
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct RegisterTeacherMessage {
    pub teacher: Addr<Teacher>,
}

/// The teacher sends this to the lobby to register itself
#[derive(Message, Debug)]
#[rtype(result = "anyhow::Result<()>")]
pub struct StartQuestionMessage {}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ServerHardStop {}

#[derive(Message, Debug)]
#[rtype(result = "Lobby")]
pub struct GetServerState {}

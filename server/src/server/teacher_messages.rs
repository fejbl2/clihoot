use actix::{prelude::Message, Addr};

use crate::teacher::init::Teacher;

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

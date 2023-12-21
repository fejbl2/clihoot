use actix::{prelude::Message, Addr};
use uuid::Uuid;

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
pub struct RegisterTeacher {
    pub teacher: Addr<Teacher>,
}

#[derive(Message, Debug)]
#[rtype(result = "anyhow::Result<()>")]
pub struct StartQuestion;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct HardStop;

#[derive(Message, Debug)]
#[rtype(result = "anyhow::Result<()>")]
pub struct EndQuestion {
    pub index: usize,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct KickPlayer {
    pub player_uuid: Uuid,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct SwitchToLeaderboard;

use std::ops::Deref;

use crate::questions::Question;
use actix::{
    dev::{MessageResponse, OneshotSender},
    prelude::Message,
    Actor,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// helper structs:

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkPlayerData {
    pub uuid: Uuid,
    pub nickname: String,
    pub color: String, // TODO enum?
}

// these models (structs) describe messages used in network communication between client - server - teacher

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "TryJoinResponse")]
pub struct TryJoinRequest {
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CanJoin {
    Yes,
    No(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TryJoinResponse {
    pub uuid: Uuid,
    pub can_join: CanJoin,
    pub quiz_name: String,
}

impl<A, M> MessageResponse<A, M> for TryJoinResponse
where
    A: Actor,
    M: Message<Result = TryJoinResponse>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinResponse {
    pub uuid: Uuid,
    pub can_join: CanJoin,
    pub quiz_name: String,
    pub players: Vec<NetworkPlayerData>,
}

impl<A, M> MessageResponse<A, M> for JoinResponse
where
    A: Actor,
    M: Message<Result = JoinResponse>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRequest {
    pub player_data: NetworkPlayerData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayersUpdate {
    pub players: Vec<NetworkPlayerData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextQuestion {
    pub question_index: u64,
    pub questions_count: u64,
    pub question: Question, // make sure to set right answer to 'false' before sending
    pub show_choices_after: u64,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct AnswerSelected {
    pub player_uuid: Uuid,
    pub question_index: usize,
    pub answers: Vec<Uuid>, // player can choose multiple answers
}

impl Deref for AnswerSelected {
    type Target = Vec<Uuid>;

    fn deref(&self) -> &Self::Target {
        &self.answers
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionUpdate {
    pub players_answered_count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionEnded {
    // TODO also uuid of question? or add uuid to the question struct?
    pub question: Question, // here we want also right choices unlike in NextQuestion
    pub player_answer: Vec<Uuid>,
    pub stats: Vec<(Uuid, u64)>, // how many answers has the question with given uuid
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowLeaderboard {
    pub players: Vec<(NetworkPlayerData, u64)>, // players with score
    pub was_final_round: bool,
}

#[derive(Debug, Serialize, Deserialize, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct KickedOutNotice {
    pub kick_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDisconnected {
    // no data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherDisconnected {
    // no data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartQuestion {
    // no data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KickPlayer {
    pub player_uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarlyEndQuestion {
    // no data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchToLeaderboard {
    // no data
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockGame {
    pub lock: bool, // if true -> lock the game, if false -> unlock
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReconnectRequest {
    pub player_uuid: Uuid,
}

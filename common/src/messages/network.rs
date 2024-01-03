use std::{collections::HashMap, ops::Deref};

use crate::questions::{Question, QuestionCensored};
use actix::{
    dev::{MessageResponse, OneshotSender},
    prelude::Message,
    Actor,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// helper structs:

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayerData {
    pub uuid: Uuid,
    pub nickname: String,
    pub color: String, // TODO enum?
}

// these models (structs) describe messages used in network communication between client - server - teacher

#[derive(Message, Debug, Serialize, Deserialize, Clone)]
#[rtype(result = "TryJoinResponse")]
pub struct TryJoinRequest {
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CanJoin {
    Yes,
    No(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JoinResponse {
    pub uuid: Uuid,
    pub can_join: CanJoin,
    pub quiz_name: String,
    pub players: Vec<PlayerData>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JoinRequest {
    pub player_data: PlayerData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PlayersUpdate {
    pub players: Vec<PlayerData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Message, PartialEq)]
#[rtype(result = "anyhow::Result<()>")]
pub struct NextQuestion {
    pub question_index: usize,
    pub questions_count: usize,
    pub question: QuestionCensored,
    pub show_choices_after: usize,
}

impl Deref for NextQuestion {
    type Target = QuestionCensored;

    fn deref(&self) -> &Self::Target {
        &self.question
    }
}

#[derive(Debug, Serialize, Deserialize, Message, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Message, PartialEq)]
#[rtype(result = "anyhow::Result<()>")]
pub struct QuestionUpdate {
    pub players_answered_count: usize,
    pub question_index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChoiceStats {
    pub players_answered_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct QuestionEnded {
    pub question_index: usize,
    pub question: Question, // here we want also right choices unlike in NextQuestion, so no censoring
    pub player_answer: Option<Vec<Uuid>>, // optional -- if player did not answer, this is None
    pub stats: HashMap<Uuid, ChoiceStats>, // how many answers has the option with given uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, Message, PartialEq)]
#[rtype(result = "anyhow::Result<()>")]
pub struct ShowLeaderboard {
    pub players: Vec<(PlayerData, usize)>, // players with score
    pub was_final_round: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientDisconnected {
    // no data
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeacherDisconnected {
    // no data
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReconnectRequest {
    pub player_uuid: Uuid,
}

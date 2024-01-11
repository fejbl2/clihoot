use crate::websocket::Websocket;
use crate::Teacher;
use actix::Actor;
use actix::Addr;

use actix::dev::MessageResponse;
use actix::dev::OneshotSender;
use actix::Message;
use chrono::DateTime;
use chrono::Utc;
use common::questions::QuestionSet;
use ratatui::style::Color;

use std::collections::HashMap;
use std::ops::Deref;
use uuid::Uuid;

#[derive(Default, PartialEq, Clone, Debug)]

pub enum Phase {
    #[default]
    WaitingForPlayers,
    ActiveQuestion(usize),
    AfterQuestion(usize),
    ShowingLeaderboard(usize),
    GameEnded,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerQuestionRecord {
    pub answer_order: usize,
    pub timestamp: DateTime<Utc>,
    pub selected_answers: Vec<Uuid>,
    pub points_awarded: usize,
}

/// Uuid of the player -> record of a single question
type PlayerRecords = HashMap<Uuid, PlayerQuestionRecord>;

/// index of the question -> results of all players
pub type QuestionRecords = HashMap<usize, PlayerRecords>;

#[derive(PartialEq, Clone, Debug)]
pub struct JoinedPlayer {
    pub uuid: Uuid,
    pub nickname: String,
    pub color: Color,
    pub addr: Addr<Websocket>,
    pub joined_at: DateTime<Utc>,
}

impl Deref for JoinedPlayer {
    type Target = Addr<Websocket>;

    fn deref(&self) -> &Self::Target {
        &self.addr
    }
}

#[derive(Debug, Clone)]
pub struct Lobby {
    /// An address to the teacher actor
    pub teacher: Option<Addr<Teacher>>,

    /// Phase of the game  
    pub phase: Phase,

    /// Whether new players can join
    pub locked: bool,

    /// References to all the connected clients
    pub joined_players: HashMap<Uuid, JoinedPlayer>,

    /// Incremental results of the game
    /// * `results[question_index][player_uuid] = PlayerQuestionRecord`
    pub results: QuestionRecords,

    /// All questions to be asked
    pub questions: QuestionSet,

    /// Players who have sent a TryJoinRequest, but have not joined yet
    pub waiting_players: Vec<Uuid>,
}

impl<A, M> MessageResponse<A, M> for Lobby
where
    A: Actor,
    M: Message<Result = Lobby>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

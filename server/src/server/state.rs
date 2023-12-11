use crate::teacher::init::Teacher;
use actix::{
    Addr,
};

use chrono::DateTime;
use chrono::Utc;
use common::{
    questions::QuestionSet,
};


use std::collections::HashMap;
use uuid::Uuid;

use super::{
    websocket::WsConn,
};

#[derive(Default, PartialEq)]
#[allow(dead_code)]
pub enum Phase {
    #[default]
    WaitingForPlayers,
    ActiveQuestion(usize),
    AfterQuestion(usize),
    ShowingLeaderboard(usize),
    GameEnded,
}

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

pub struct Lobby {
    /// An address to the teacher actor
    pub teacher: Option<Addr<Teacher>>,

    /// Phase of the game  
    pub phase: Phase,

    /// Whether new players can join
    pub locked: bool,

    /// References to all the connected clients
    pub joined_players: HashMap<Uuid, Addr<WsConn>>,

    /// Incremental results of the game
    /// * `results[question_index][player_uuid] = PlayerQuestionRecord`
    pub results: QuestionRecords,

    /// All questions to be asked
    pub questions: QuestionSet,

    /// Players who have sent a TryJoinRequest, but have not joined yet
    pub waiting_players: Vec<Uuid>,
}

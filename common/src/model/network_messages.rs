use crate::questions::Question;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// helper structs:

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkPlayerData {
    pub uuid: Uuid,
    pub nickname: String,
    pub colour: String, // TODO enum?
}

// these models (structs) describe messages used in network communication between client - server - teacher

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TryJoinRequest {
    pub uuid: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinRequest {
    pub player_data: NetworkPlayerData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayersUpdate {
    pub players: Vec<NetworkPlayerData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NextQuestion {
    pub uuid: Uuid,
    pub question_number: u64,
    pub questions_count: u64,
    pub question: Question, // make sure to set right answer to 'false' before sending
    pub show_choices_after: u64,
    pub time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnswerSelected {
    pub player_uuid: Uuid,
    pub question_uuid: Uuid,
    pub answer: Vec<Uuid>, // player can choose multiple answers
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionUpdate {
    pub players_answered_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionEnded {
    // TODO also uuid of question? or add uuid to the question struct?
    pub question: Question, // here we want also right choices unlike in NextQuestion
    pub player_answer: Vec<Uuid>,
    pub stats: Vec<(Uuid, u64)>, // how many answers has the question with given uuid
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShowLeaderboard {
    pub players: Vec<(NetworkPlayerData, u64)>, // players with score
    pub was_final_round: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KickedOutNotice {
    pub kick_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientDisconnected {
    // no data
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TeacherDisconnected {
    // no data
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StartQuestion {
    // no data
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KickPlayer {
    pub player_uuid: Uuid,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EarlyEndQuestion {
    // no data
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchToLeaderboard {
    // no data
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LockGame {
    pub lock: bool, // if true -> lock the game, if false -> unlock
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReconnectRequest {
    pub player_uuid: Uuid,
}

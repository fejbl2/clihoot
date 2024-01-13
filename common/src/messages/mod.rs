use serde::{Deserialize, Serialize};

use self::network::{
    AnswerSelected, JoinRequest, JoinResponse, NextQuestion, PlayersUpdate, QuestionEnded,
    QuestionUpdate, ShowLeaderboard, TeacherDisconnected, TryJoinRequest, TryJoinResponse,
};
use actix::Message;

pub mod network;
pub mod status;

/// The messages that can be sent over the websocket FROM the client TO server
#[derive(Debug, Serialize, Deserialize, Message, Clone)]
#[rtype(result = "()")]
pub enum ClientNetworkMessage {
    TryJoinRequest(TryJoinRequest),
    JoinRequest(JoinRequest),
    AnswerSelected(AnswerSelected),
}

/// The messages that can be sent over the websocket FROM the server TO the client
#[derive(Debug, Serialize, Deserialize, Message, Clone)]
#[rtype(result = "anyhow::Result<()>")]
pub enum ServerNetworkMessage {
    PlayersUpdate(PlayersUpdate),
    NextQuestion(NextQuestion),
    QuestionUpdate(QuestionUpdate),
    QuestionEnded(QuestionEnded),
    ShowLeaderboard(ShowLeaderboard),
    TeacherDisconnected(TeacherDisconnected),
    JoinResponse(JoinResponse),
    TryJoinResponse(TryJoinResponse),
}

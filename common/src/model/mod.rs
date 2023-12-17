use serde::{Deserialize, Serialize};

use self::network_messages::{
    AnswerSelected, ClientDisconnected, JoinRequest, KickedOutNotice, NextQuestion, PlayersUpdate,
    QuestionEnded, QuestionUpdate, ShowLeaderboard, TeacherDisconnected, TryJoinRequest,
};
use actix::Message;

pub mod network_messages;

/// The messages that can be sent over the websocket FROM the client TO server
#[derive(Debug, Serialize, Deserialize, Message, Clone)]
#[rtype(result = "()")]
pub enum ClientNetworkMessage {
    TryJoinRequest(TryJoinRequest),
    JoinRequest(JoinRequest),
    AnswerSelected(AnswerSelected),
    ClientDisconnected(ClientDisconnected),
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
    KickedOutNotice(KickedOutNotice),
    TeacherDisconnected(TeacherDisconnected),
}

use actix::Message;
use serde::{Deserialize, Serialize};

use self::network_messages::{
    AnswerSelected, ClientDisconnected, JoinRequest, KickedOutNotice, NextQuestion, PlayersUpdate,
    QuestionEnded, QuestionUpdate, ShowLeaderboard, TeacherDisconnected, TryJoinRequest,
};

pub mod network_messages;

/// The messages that can be sent over the websocket between the client and the server
#[derive(Clone, Debug, Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub enum NetworkMessage {
    TryJoinRequest(TryJoinRequest),
    JoinRequest(JoinRequest),
    PlayersUpdate(PlayersUpdate),
    NextQuestion(NextQuestion),
    AnswerSelected(AnswerSelected),
    QuestionUpdate(QuestionUpdate),
    QuestionEnded(QuestionEnded),
    ShowLeaderboard(ShowLeaderboard),
    KickedOutNotice(KickedOutNotice),
    ClientDisconnected(ClientDisconnected),
    TeacherDisconnected(TeacherDisconnected),
}

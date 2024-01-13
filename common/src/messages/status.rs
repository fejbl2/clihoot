use actix::Message;

#[derive(Debug, Message, Clone)]
#[rtype(result = "anyhow::Result<()>")]
#[allow(clippy::module_name_repetitions)]
pub enum ClientWebsocketStatus {
    ListeningFail,
    CantSendMessage,
    SocketClosed,
    CloseFrameReceived(String),
}

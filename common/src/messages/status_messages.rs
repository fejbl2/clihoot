use actix::Message;

#[derive(Debug, Message, Clone)]
#[rtype(result = "anyhow::Result<()>")]
pub enum ClientWebsocketStatus {
    ListeningFail,
    CantSendMessage,
    SocketClosed,
    CloseFrameReceived(String),
}

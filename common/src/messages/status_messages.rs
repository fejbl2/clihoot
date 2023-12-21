use actix::Message;

#[derive(Debug, Message, Clone)]
#[rtype(result = "()")]
pub enum ClientWebsocketStatus {
    ListeningFail,
    CantSendMessage,
    SocketClosed,
    CloseFrameReceived(String),
}

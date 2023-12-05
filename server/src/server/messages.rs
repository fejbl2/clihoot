use actix::prelude::{Message, Recipient};
use uuid::Uuid;

//WsConn responds to this to pipe it through to the actual client
#[derive(Message)]
#[rtype(result = "()")]
pub struct RelayMessageToLobby(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsGracefulCloseConnection;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsHardCloseConnection;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RelayMessageToClient(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ConnectToLobby {
    pub addr: Recipient<RelayMessageToClient>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DisconnectFromLobby {
    pub room_id: Uuid,
    pub id: Uuid,
}

//client sends this to the lobby for the lobby to echo out.
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}

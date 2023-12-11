use actix::{prelude::Message, Addr};
use uuid::Uuid;

use crate::websocket::websocket::Websocket;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsGracefulCloseConnection;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsHardCloseConnection;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LobbyOutputMessage(pub String);

//WsConn sends this to the lobby to say "put me in please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct ConnectToLobby {
    pub addr: Addr<Websocket>,
    pub player_id: Uuid,
}

//WsConn sends this to a lobby to say "take me out please"
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DisconnectFromLobby {
    pub player_id: Uuid,
}

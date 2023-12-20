use actix::{Addr, Message};
use common::messages::network::PlayerData;

use crate::websocket::Websocket;

#[derive(Debug, Message)]
#[rtype(result = "common::messages::network::JoinResponse")]
pub struct JoinRequest {
    pub player_data: PlayerData,
    pub addr: Addr<Websocket>,
}

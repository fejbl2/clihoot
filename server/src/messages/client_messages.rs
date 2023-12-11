use actix::{Addr, Message};
use common::model::network_messages::NetworkPlayerData;

use crate::websocket::Websocket;

#[derive(Debug, Message)]
#[rtype(result = "common::model::network_messages::JoinResponse")]
pub struct JoinRequest {
    pub player_data: NetworkPlayerData,
    pub addr: Addr<Websocket>,
}

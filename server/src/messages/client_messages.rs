use actix::{Addr, Message};
use common::model::network_messages::NetworkPlayerData;

use crate::websocket::websocket::Websocket;

#[derive(Debug, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct JoinRequest {
    pub player_data: NetworkPlayerData,
    pub ws_conn: Addr<Websocket>,
}

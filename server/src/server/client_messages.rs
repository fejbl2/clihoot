use actix::{Addr, Message};
use common::model::network_messages::NetworkPlayerData;

use super::websocket::WsConn;

#[derive(Debug, Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct JoinRequest {
    pub player_data: NetworkPlayerData,
    pub ws_conn: Addr<WsConn>,
}

use crate::messages::{
    ClientActorMessage, ConnectToLobby, DisconnectFromLobby, RelayMessageToClient,
};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type Socket = Recipient<RelayMessageToClient>;

#[derive(Default)]
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,     //self id to self
    rooms: HashMap<Uuid, HashSet<Uuid>>, //room id  to list of users id
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            socket_recipient.do_send(RelayMessageToClient(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Lobby started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        println!("Lobby stopping");
        actix::prelude::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Lobby stopped");
    }
}

/// Handler for Disconnect message.
impl Handler<DisconnectFromLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: DisconnectFromLobby, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected.", &msg.id), user_id);
                });
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<ConnectToLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ConnectToLobby, _: &mut Context<Self>) -> Self::Result {
        // create a room if necessary, and then add the id to it
        self.rooms
            .entry(msg.lobby_id)
            .or_default()
            .insert(msg.self_id);

        // send to everyone in the room that new uuid just joined
        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| {
                self.send_message(&format!("{} just joined!", msg.self_id), conn_id);
            });

        // store the address
        self.sessions.insert(msg.self_id, msg.addr);

        // send self your new uuid
        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}

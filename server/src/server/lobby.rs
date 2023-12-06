use crate::{
    server::messages::{
        ClientActorMessage, ConnectToLobby, DisconnectFromLobby, RelayMessageToClient,
    },
    teacher::init::Teacher,
};
use actix::{
    prelude::{Actor, Context, Handler},
    Addr,
};
use rand::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    teacher_messages::{RegisterTeacherMessage, SetLockMessage, StartQuestionMessage},
    websocket::WsConn,
};

#[derive(Default, PartialEq)]
enum Phase {
    #[default]
    WaitingForPlayers,
    ActiveQuestion(Uuid),
    AfterQuestion(Uuid),
    ShowingLeaderboard(Uuid),
    GameEnded,
}

impl Phase {
    fn next_phase(&self, next_question: Uuid) -> Phase {
        match self {
            Phase::WaitingForPlayers => Phase::ActiveQuestion(next_question),
            Phase::ActiveQuestion(guid) => Phase::AfterQuestion(*guid),
            Phase::AfterQuestion(guid) => Phase::ActiveQuestion(*guid),
            Phase::ShowingLeaderboard(_guid) => Phase::ActiveQuestion(next_question),
            Phase::GameEnded => Phase::GameEnded,
        }
    }
}

// TODO: remove
#[derive(Debug, Clone, Deserialize)]
pub struct Question {
    id: Uuid,
    question: String,
    answers: Vec<String>,
    correct_answer: usize,
}

pub struct Lobby {
    /// An address to the teacher actor
    teacher: Option<Addr<Teacher>>,

    /// Phase of the game  
    phase: Phase,

    /// Whether to randomize answers order
    randomize_answers: bool,

    /// Whether new players can join
    locked: bool,

    /// References to all the connected clients
    joined_players: HashMap<Uuid, Addr<WsConn>>,

    questions: Vec<Question>,
}

impl Default for Lobby {
    fn default() -> Self {
        Lobby {
            teacher: None,
            phase: Phase::default(),
            randomize_answers: false,
            locked: true,
            joined_players: HashMap::new(),
            questions: Vec::new(),
        }
    }
}

impl Lobby {
    pub fn new(
        mut questions: Vec<Question>,
        randomize_questions: bool,
        randomize_answers: bool,
    ) -> Self {
        if randomize_questions {
            let mut rng = rand::thread_rng();
            questions.shuffle(&mut rng);
        }

        Lobby {
            teacher: None,
            phase: Phase::default(),
            randomize_answers,
            locked: true,
            joined_players: HashMap::new(),
            questions,
        }
    }

    fn next_question(&self) -> &Question {
        let prev_question = match self.phase {
            Phase::WaitingForPlayers => None,
            Phase::ActiveQuestion(guid) => Some(guid),
            Phase::AfterQuestion(guid) => Some(guid),
            Phase::ShowingLeaderboard(guid) => Some(guid),
            Phase::GameEnded => panic!("Cannot call next_question when game has ended"),
        };

        // get the next question after the previous one
        let next_question = if let Some(prev_question) = prev_question {
            // find the index of the previous question
            let index = self
                .questions
                .iter()
                .position(|q| q.id == prev_question)
                .unwrap();

            // should not happen
            assert!(index != self.questions.len() - 1, "Received StartQuestionMessage in Lobby, but can't show next question, because there is no next question");

            // get the next question
            &self.questions[index + 1]
        } else {
            // no previous question, so get the first one
            self.questions.first().unwrap()
        };

        next_question
    }

    fn can_show_next_question(&self) -> bool {
        match self.phase {
            Phase::WaitingForPlayers => true,
            Phase::AfterQuestion(guid) => {
                // find if this question is the last
                self.questions.last().unwrap().id != guid
            }
            _ => false,
        }
    }

    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.joined_players.get(id_to) {
            socket_recipient.do_send(RelayMessageToClient(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

    fn send_to_all(&self, message: &str, include_teacher: bool) {
        for (_id, socket_recipient) in &self.joined_players {
            socket_recipient.do_send(RelayMessageToClient(message.to_owned()));
        }

        if include_teacher {
            if let Some(_teacher) = &self.teacher {
                // teacher.do_send(RelayMessageToClient(message.to_owned()));
            }
        }
    }

    fn send_to_other(&self, message: &str, id_from: &Uuid, include_teacher: bool) {
        for (id, socket_recipient) in &self.joined_players {
            if id != id_from {
                socket_recipient.do_send(RelayMessageToClient(message.to_owned()));
            }
        }

        if include_teacher {
            if let Some(_teacher) = &self.teacher {
                // teacher.do_send(RelayMessageToClient(message.to_owned()));
            }
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
        if self.joined_players.remove(&msg.player_id).is_some() {
            println!("{} disconnected", msg.player_id);
            // TODO: send `PlayersUpdate` message
        }
    }
}

impl Handler<ConnectToLobby> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ConnectToLobby, _: &mut Context<Self>) -> Self::Result {
        // save info that new client joined
        self.joined_players.insert(msg.player_id, msg.addr);
        println!("{} joined", msg.player_id);

        // TODO: remove - just for testing
        self.send_message(&format!("your id is {}", msg.player_id), &msg.player_id);

        // send to all other clients that new client joined
        self.send_to_other(&format!("{} joined", msg.player_id), &msg.player_id, false);
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _: &mut Context<Self>) -> Self::Result {
        // TODO - remove - send the message to all clients
        self.send_to_all(msg.msg.as_str(), false);
    }
}

impl Handler<RegisterTeacherMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RegisterTeacherMessage, _: &mut Context<Self>) -> Self::Result {
        println!("Received RegisterTeacherMessage in Lobby; unlocking lobby");
        self.teacher = Some(msg.teacher);

        // only now actually start the server (i.e. allow players to join)
        self.locked = false;
    }
}

impl Handler<SetLockMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: SetLockMessage, _: &mut Context<Self>) -> Self::Result {
        println!(
            "Received SetLockMessage in Lobby; setting `locked` to `{}`",
            msg.locked
        );
        self.locked = msg.locked;
    }
}

impl Handler<StartQuestionMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, _msg: StartQuestionMessage, _: &mut Context<Self>) -> Self::Result {
        // 1. check that we can show the next question
        // 2. find which question it is
        // 3. set the phase to `ActiveQuestion`
        // 4. send the question to all clients as well as the teacher

        if !self.can_show_next_question() {
            println!("Received StartQuestionMessage in Lobby, but can't show next question");
            return;
        }

        let next_question = self.next_question();
        self.phase = Phase::ActiveQuestion(next_question);
    }
}

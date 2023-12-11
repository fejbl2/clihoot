use actix::prelude::{Actor, Context};
use anyhow::Ok;
use common::{
    model::{network_messages::NextQuestion, NetworkMessage},
    questions::QuestionSet,
};
use rand::prelude::*;

use std::collections::HashMap;
use uuid::Uuid;

use crate::messages::websocket_messages::LobbyOutputMessage;

use super::state::{Lobby, Phase};

impl Lobby {
    pub fn new(mut questions: QuestionSet) -> Self {
        if questions.randomize_questions {
            let mut rng = rand::thread_rng();
            questions.questions.shuffle(&mut rng);
        }

        Lobby {
            teacher: None,
            phase: Phase::default(),
            locked: true,
            joined_players: HashMap::new(),
            questions,
            waiting_players: Vec::new(),
            results: HashMap::new(),
        }
    }

    pub fn next_question(&self) -> anyhow::Result<usize> {
        if !self.can_show_next_question() {
            return Err(anyhow::anyhow!("Can't show next question"));
        }

        match self.phase {
            Phase::WaitingForPlayers => Ok(0),
            Phase::ActiveQuestion(index)
            | Phase::AfterQuestion(index)
            | Phase::ShowingLeaderboard(index) => Ok(index + 1),
            Phase::GameEnded => Err(anyhow::anyhow!(
                "Cannot call next_question when game has ended"
            )),
        }
    }

    pub fn can_show_next_question(&self) -> bool {
        match self.phase {
            Phase::WaitingForPlayers => true,
            Phase::ShowingLeaderboard(index) => {
                // only show next question if we are not at the last question
                index < self.questions.len() - 1
            }
            _ => false,
        }
    }

    pub fn send_question(&self, index: usize) -> anyhow::Result<()> {
        let mut question = self.questions[index].clone();

        // censor the right answers
        question.choices.iter_mut().for_each(|choice| {
            choice.is_right = false;
        });

        // construct a message object
        let message = NetworkMessage::NextQuestion(NextQuestion {
            question_index: index as u64,
            questions_count: self.questions.len() as u64,
            show_choices_after: question.get_reading_time_estimate() as u64,
            question,
        });

        // send it to everyone
        self.send_to_all(&serde_json::to_string(&message)?, true);

        Ok(())
    }

    pub fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.joined_players.get(id_to) {
            socket_recipient.do_send(LobbyOutputMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

    pub fn send_to_all(&self, message: &str, include_teacher: bool) {
        for socket_recipient in self.joined_players.values() {
            socket_recipient.do_send(LobbyOutputMessage(message.to_owned()));
        }

        if include_teacher {
            if let Some(teacher) = &self.teacher {
                teacher.do_send(LobbyOutputMessage(message.to_owned()));
            }
        }
    }

    pub fn send_to_other(&self, message: &str, id_from: &Uuid, include_teacher: bool) {
        for (id, socket_recipient) in &self.joined_players {
            if id != id_from {
                socket_recipient.do_send(LobbyOutputMessage(message.to_owned()));
            }
        }

        if include_teacher {
            if let Some(_teacher) = &self.teacher {}
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

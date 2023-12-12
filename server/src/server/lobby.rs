use actix::prelude::{Actor, Context};
use anyhow::Ok;
use common::{
    model::{
        network_messages::{NetworkPlayerData, NextQuestion},
        ServerNetworkMessage,
    },
    questions::QuestionSet,
};
use rand::prelude::*;

use std::collections::HashMap;
use uuid::Uuid;

use super::state::{Lobby, Phase};

impl Lobby {
    #[must_use]
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

    #[must_use]
    pub fn get_players(&self) -> Vec<NetworkPlayerData> {
        self.joined_players
            .values()
            .map(|val| NetworkPlayerData {
                color: val.color.clone(),
                nickname: val.nickname.clone(),
                uuid: val.uuid,
            })
            .collect()
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

    #[must_use]
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
        let message = ServerNetworkMessage::NextQuestion(NextQuestion {
            question_index: index as u64,
            questions_count: self.questions.len() as u64,
            show_choices_after: question.get_reading_time_estimate() as u64,
            question,
        });

        // send it to everyone
        self.send_to_all(&serde_json::to_string(&message)?, true);

        Ok(())
    }

    #[allow(dead_code)]
    pub fn send_message(&self, _message: &str, id_to: &Uuid) {
        if let Some(_socket_recipient) = self.joined_players.get(id_to) {
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }

    pub fn send_to_all(&self, _message: &str, include_teacher: bool) {
        // TODO: send the message to all players
        // TODO: the message should not be str, but ServerNetworkMessage (to have the ws enforce the types)
        //  -> probably, the teacher cannot be included like so, because he will not implement All Handler<ServerNetworkMessage>
        for _socket_recipient in self.joined_players.values() {}

        if include_teacher {
            if let Some(_teacher) = &self.teacher {}
        }
    }

    #[allow(dead_code)]
    pub fn send_to_other(&self, _message: &str, _id_from: &Uuid, _include_teacher: bool) {
        // for (id, _socket_recipient) in &self.joined_players {
        //     if id != id_from {}
        // }

        // if include_teacher {
        //     if let Some(_teacher) = &self.teacher {}
        // }
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

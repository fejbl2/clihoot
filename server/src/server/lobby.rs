use actix::prelude::{Actor, Context};
use anyhow::Ok;
use common::{
    model::{
        network_messages::{NetworkPlayerData, NextQuestion},
        ServerNetworkMessage,
    },
    questions::{Question, QuestionSet},
};
use rand::prelude::*;

use std::collections::HashMap;
use uuid::Uuid;

use super::state::{Lobby, Phase};

pub fn censor_question(question: &mut Question) -> &mut Question {
    question.choices.iter_mut().for_each(|choice| {
        choice.is_right = false;
    });

    question
}

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
        censor_question(&mut question);

        // construct a message object
        let message = NextQuestion {
            question_index: index as u64,
            questions_count: self.questions.len() as u64,
            show_choices_after: question.get_reading_time_estimate() as u64,
            question,
        };

        // send it to all students
        self.send_to_all(&ServerNetworkMessage::NextQuestion(message.clone()));

        // and also to the teacher
        let Some(ref teacher) = self.teacher else {
            anyhow::bail!("Cannot send to teacher, Teacher is null");
        };

        teacher.do_send(message);

        Ok(())
    }

    #[allow(dead_code)]
    /// Sends a message to a specific player
    /// # Errors
    /// - when the `id_to` does not exist in the `joined_players` hashmap
    pub fn send_message(&self, _message: &str, id_to: &Uuid) -> anyhow::Result<()> {
        let Some(_socket_recipient) = self.joined_players.get(id_to) else {
            anyhow::bail!("attempting to send message but couldn't find user id.");
        };

        // TODO
        Ok(())
    }

    pub fn send_to_all(&self, message: &ServerNetworkMessage) {
        for socket_recipient in self.joined_players.values() {
            socket_recipient.do_send(message.clone());
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

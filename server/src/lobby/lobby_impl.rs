use actix::prelude::{Actor, Context};

use anyhow::Ok;
use common::{
    messages::{
        network::{
            ChoiceStats, NextQuestion, PlayerData, PlayersUpdate, QuestionEnded, QuestionUpdate,
            ShowLeaderboard,
        },
        ServerNetworkMessage,
    },
    questions::{QuestionCensored, QuestionSet},
};

use log::debug;
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
    pub fn get_players(&self) -> Vec<PlayerData> {
        let mut players: Vec<_> = self.joined_players.values().collect();

        players.sort_by_key(|x| x.joined_at);

        players
            .into_iter()
            .map(|val| PlayerData {
                color: val.color,
                nickname: val.nickname.clone(),
                uuid: val.uuid,
            })
            .collect()
    }

    /// Returns the index of the next question, or an error if there is no next question
    /// # Errors
    /// - when the game has ended
    /// - when the current phase is not `WaitingForPlayers` or `ShowingLeaderboard`
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

    fn get_question_stats(&self, index: usize) -> anyhow::Result<HashMap<Uuid, ChoiceStats>> {
        let mut stats: HashMap<Uuid, ChoiceStats> =
            HashMap::with_capacity(self.questions[index].choices.len());

        // fill stats with zeros for each choice
        for choice in &self.questions[index].choices {
            stats.insert(
                choice.id,
                ChoiceStats {
                    players_answered_count: 0,
                },
            );
        }

        // calculate how many players (usize) chose the given option (uuid)

        let Some(results) = self.results.get(&index) else {
            // no results for this question -- means no one answered
            return Ok(stats);
        };

        for result in results {
            for answer in &result.1.selected_answers {
                if let Some(choice_stats) = stats.get_mut(answer) {
                    choice_stats.players_answered_count += 1;
                } else {
                    // never happens
                    panic!("Answer {answer} not found in stats");
                }
            }
        }

        Ok(stats)
    }

    fn get_player_answer(&self, index: usize, player_id: &Uuid) -> Option<Vec<Uuid>> {
        self.results
            .get(&index)
            .and_then(|results| results.get(player_id))
            .map(|record| record.selected_answers.clone())
    }

    pub fn send_question_ended(&self, index: usize) -> anyhow::Result<()> {
        let stats = self.get_question_stats(index)?;

        for (player_id, socket_recipient) in &self.joined_players {
            debug!("Sending QuestionEnded to player {player_id}");
            socket_recipient.do_send(ServerNetworkMessage::QuestionEnded(QuestionEnded {
                stats: stats.clone(),
                player_answer: self.get_player_answer(index, player_id),
                question_index: index,
                question: self.questions[index].clone(),
            }));
        }

        // and also to the teacher
        let Some(ref teacher) = self.teacher else {
            anyhow::bail!("Cannot send to teacher, Teacher is null");
        };

        teacher.do_send(QuestionEnded {
            stats: stats.clone(),
            player_answer: None,
            question_index: index,
            question: self.questions[index].clone(),
        });
        Ok(())
    }

    pub fn send_leaderboard(&self, index: usize) -> anyhow::Result<bool> {
        let is_final = index == self.questions.len() - 1;

        let message = ShowLeaderboard {
            was_final_round: is_final,
            players: self
                .get_players()
                .into_iter()
                .map(|player| {
                    // here, sum up the scores for each question so far

                    let score = (0..=index)
                        .map(|question_index| {
                            self.results
                                .get(&question_index)
                                .and_then(|results| results.get(&player.uuid))
                                .map_or(0, |record| record.points_awarded)
                        })
                        .sum();

                    (player, score)
                })
                .collect(),
        };

        // send it to all students
        self.send_to_all(&ServerNetworkMessage::ShowLeaderboard(message.clone()));

        // and also to the teacher
        let Some(ref teacher) = self.teacher else {
            anyhow::bail!("Cannot send to teacher, Teacher is null");
        };

        teacher.do_send(message);

        Ok(is_final)
    }

    pub fn send_question_update(&self, index: usize) -> anyhow::Result<()> {
        let answered_count = self
            .results
            .get(&index)
            .map_or(0, std::collections::HashMap::len);

        // construct a message object
        let message = QuestionUpdate {
            players_answered_count: answered_count,
            question_index: index,
        };

        // send it to all students
        self.send_to_all(&ServerNetworkMessage::QuestionUpdate(message.clone()));

        // and also to the teacher
        let Some(ref teacher) = self.teacher else {
            anyhow::bail!("Cannot send to teacher, Teacher is null");
        };

        teacher.do_send(message);
        Ok(())
    }

    pub fn send_question(&self, index: usize) -> anyhow::Result<usize> {
        let question = self.questions[index].clone();

        let answer_time = question.time_seconds;
        let reading_time = question.get_reading_time_estimate();

        // construct a message object
        let message = NextQuestion {
            question_index: index,
            questions_count: self.questions.len(),
            show_choices_after: question.get_reading_time_estimate(),
            question: QuestionCensored::from(question),
        };

        // send it to all students
        self.send_to_all(&ServerNetworkMessage::NextQuestion(message.clone()));

        // and also to the teacher
        let Some(ref teacher) = self.teacher else {
            anyhow::bail!("Cannot send to teacher, Teacher is null");
        };

        teacher.do_send(message);

        Ok(reading_time + answer_time)
    }

    /// Sends the `message` to all joined players
    pub fn send_to_all(&self, message: &ServerNetworkMessage) {
        for socket_recipient in self.joined_players.values() {
            socket_recipient.do_send(message.clone());
        }
    }

    /// Sends the `message` to all joined players except the one with `id_from`
    pub fn send_to_others(&self, message: &ServerNetworkMessage, id_from: &Uuid) {
        for (id, socket_recipient) in &self.joined_players {
            if id != id_from {
                socket_recipient.do_send(message.clone());
            }
        }
    }

    /// Sends the `PlayersUpdate` to all currently joined players. Should be invoked
    /// whenever the list of players changes.
    /// If `except` is not None, the message will not be sent to the player with this id.
    pub fn send_players_update(&self, except: Option<&Uuid>) -> anyhow::Result<()> {
        let message = PlayersUpdate {
            players: self.get_players(),
        };

        // (maybe) NICE TO HAVE: delay sending of the update by 100 ms, and
        //   if another `send_players_update` is called, discard the previous one and send the new one
        if let Some(uuid) = except {
            self.send_to_others(&ServerNetworkMessage::PlayersUpdate(message.clone()), uuid);
        } else {
            self.send_to_all(&ServerNetworkMessage::PlayersUpdate(message.clone()));
        }

        // also send to teacher
        let Some(ref teacher) = self.teacher else {
            anyhow::bail!("Cannot send to teacher, Teacher is null");
        };

        teacher.do_send(message);
        Ok(())
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("Lobby started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        debug!("Lobby stopping");
        actix::prelude::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("Lobby stopped");
    }
}

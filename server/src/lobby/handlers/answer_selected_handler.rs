use crate::{
    lobby::{
        point_calculator::calculate_points,
        state::{Lobby, Phase, PlayerQuestionRecord},
    },
    messages::lobby::EndQuestion,
};
use actix::{prelude::Handler, AsyncContext};
use anyhow::{bail, Ok};
use chrono::Utc;
use common::messages::network::AnswerSelected;
use log::debug;

impl Handler<AnswerSelected> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: AnswerSelected, ctx: &mut Self::Context) -> Self::Result {
        debug!(
            "Received AnswerSelected message: {:?} from {:?}",
            msg, msg.player_uuid
        );

        let id = msg.player_uuid;

        if !self.joined_players.contains_key(&id) {
            bail!("Player {id} not in joined list");
        }

        if self.phase != Phase::ActiveQuestion(msg.question_index)
            && self.phase != Phase::AfterQuestion(msg.question_index)
        {
            bail!("Not the right phase");
        }

        if self.phase != Phase::ActiveQuestion(msg.question_index) {
            debug!(
                "Player {id} answered after the question ended, but is not cheating -- ignoring."
            );
            return Ok(());
        }

        if self
            .results
            .entry(msg.question_index)
            .or_default()
            .contains_key(&id)
        {
            bail!("Player {id} already answered this question");
        }

        // If player selected more than one answer, but the question is not multichoice, return error
        if msg.answers.len() > 1 && !self.questions[msg.question_index].is_multichoice {
            bail!("Player {id} selected more than one answer, but the question is not multichoice");
        }

        let answer_order = self.results.entry(msg.question_index).or_default().len();
        debug!("Player {id} answered as {answer_order}th player");

        let points = calculate_points(
            id,
            answer_order,
            self.joined_players.len(),
            msg.question_index,
            &msg,
            &self.questions,
            &self.results,
        )?;
        debug!("Player {id} got {points} points");

        self.results.entry(msg.question_index).or_default().insert(
            id,
            PlayerQuestionRecord {
                answer_order: answer_order + 1,
                timestamp: Utc::now(),
                selected_answers: msg.answers,
                points_awarded: points,
            },
        );

        // if the last player answered, notify self of the end of the question
        if self.results.entry(msg.question_index).or_default().len() == self.joined_players.len() {
            ctx.notify(EndQuestion {
                index: msg.question_index,
            });

            return Ok(());
        }

        // if not last player, send update to everybody about the count of answers
        self.send_question_update(msg.question_index)?;

        Ok(())
    }
}

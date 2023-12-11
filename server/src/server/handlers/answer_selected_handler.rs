use crate::server::{
    point_calculator::calculate_points,
    state::{Lobby, Phase, PlayerQuestionRecord},
};
use actix::prelude::Handler;
use anyhow::Ok;
use chrono::Utc;
use common::model::network_messages::AnswerSelected;

use std::collections::HashMap;

impl Handler<AnswerSelected> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: AnswerSelected, _ctx: &mut Self::Context) -> Self::Result {
        let id = msg.player_uuid;

        if !self.joined_players.contains_key(&id) {
            return Err(anyhow::anyhow!("Player {id} not in joined list"));
        }

        if self.phase != Phase::ActiveQuestion(msg.question_index) {
            return Err(anyhow::anyhow!("Not the right phase"));
        }

        if self
            .results
            .entry(msg.question_index)
            .or_insert_with(HashMap::new)
            .contains_key(&id)
        {
            return Err(anyhow::anyhow!(
                "Player {id} already answered this question"
            ));
        }

        let answer_order = self
            .results
            .entry(msg.question_index)
            .or_insert_with(HashMap::new)
            .len();

        let points =
            calculate_points(id, msg.question_index, &msg, &self.questions, &self.results)?;

        self.results
            .entry(msg.question_index)
            .or_insert_with(HashMap::new)
            .insert(
                id,
                PlayerQuestionRecord {
                    answer_order,
                    timestamp: Utc::now(),
                    selected_answers: msg.answers,
                    points_awarded: points,
                },
            );

        // TODO: here, send update to everybody about the count of answers

        Ok(())
    }
}

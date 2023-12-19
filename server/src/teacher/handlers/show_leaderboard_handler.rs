use actix::{Context, Handler};
use common::model::network_messages::ShowLeaderboard;

use crate::teacher::init::Teacher;

impl Handler<ShowLeaderboard> for Teacher {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: ShowLeaderboard, _: &mut Context<Self>) -> Self::Result {
        // TODO: show to the teacher
        Ok(())
    }
}
use actix::{Context, Handler};
use anyhow::bail;

use crate::{
    lobby::state::{Lobby, Phase},
    messages::lobby::EndQuestion,
};

impl Handler<EndQuestion> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: EndQuestion, _: &mut Context<Self>) -> Self::Result {
        // check that it's the correct phase
        let Phase::ActiveQuestion(question) = self.phase else {
            bail!("Received EarlyEndQuestion in Lobby, but the lobby is not in the ActiveQuestion phase; ignoring");
        };

        if question != msg.index {
            bail!("Received EarlyEndQuestion in Lobby, but the question index does not match the current question; ignoring");
        }

        // set the phase
        self.phase = Phase::AfterQuestion(question);

        // send everybody the QuestionEnded message
        self.send_question_ended(question)?;

        Ok(())
    }
}

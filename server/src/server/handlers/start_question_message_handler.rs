use actix::{Context, Handler};

use crate::{
    messages::teacher_messages::StartQuestionMessage,
    server::state::{Lobby, Phase},
};

impl Handler<StartQuestionMessage> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: StartQuestionMessage, _: &mut Context<Self>) -> Self::Result {
        // * find the next question
        // * set the phase to `ActiveQuestion`
        // * send the question to all clients as well as the teacher

        let next_question = self.next_question()?;
        self.phase = Phase::ActiveQuestion(next_question);

        self.send_question(next_question)?;

        Ok(())
    }
}

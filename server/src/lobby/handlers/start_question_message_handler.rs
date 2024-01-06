use std::time::Duration;

use actix::{Addr, AsyncContext, Context, Handler};

use crate::{
    lobby::{Lobby, Phase},
    messages::lobby::{EndQuestion, StartQuestion},
};

use log::debug;

async fn notify_end_question_after(duration: Duration, index: usize, addr: Addr<Lobby>) {
    debug!("Waiting for {:?} seconds", duration.as_secs());
    tokio::time::sleep(duration).await;
    debug!("Sending EndQuestion");
    addr.do_send(EndQuestion { index });
}

impl Handler<StartQuestion> for Lobby {
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: StartQuestion, ctx: &mut Context<Self>) -> Self::Result {
        debug!("Received StartQuestion message in Lobby; starting question");

        // * find the next question
        // * set the phase to `ActiveQuestion`
        // * send the question to all clients as well as the teacher
        // * start the timer

        let next_question = self.next_question()?;
        self.phase = Phase::ActiveQuestion(next_question);

        let end_time = self.send_question(next_question)?;

        // spawn a task which will notify self after the timer is done
        tokio::spawn(notify_end_question_after(
            Duration::from_secs(end_time.try_into()?),
            next_question,
            ctx.address(),
        ));

        Ok(())
    }
}

use actix::{Context, Handler};

use crate::{messages::teacher_messages::RegisterTeacherMessage, server::state::Lobby};

impl Handler<RegisterTeacherMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RegisterTeacherMessage, _: &mut Context<Self>) -> Self::Result {
        println!("Received RegisterTeacherMessage in Lobby; unlocking lobby");
        self.teacher = Some(msg.teacher);

        // only now actually start the server (i.e. allow players to join)
        self.locked = false;
    }
}

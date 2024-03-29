use actix::{Context, Handler};

use crate::{messages::lobby::RegisterTeacher, Lobby};

use log::debug;

impl Handler<RegisterTeacher> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: RegisterTeacher, _: &mut Context<Self>) -> Self::Result {
        debug!("Received RegisterTeacherMessage in Lobby; unlocking lobby");
        self.teacher = Some(msg.teacher);

        // only now actually start the server (i.e. allow players to join)
        self.locked = false;
    }
}

use actix_rt::System;
use common::terminal::terminal_actor::TerminalStop;

use crate::{messages::lobby, teacher::terminal::TeacherTerminal};

impl TerminalStop for TeacherTerminal {
    fn stop(&mut self) -> anyhow::Result<()> {
        // lobby is on a different thread
        self.lobby.do_send(lobby::HardStop);
        System::current().stop();

        Ok(())
    }
}

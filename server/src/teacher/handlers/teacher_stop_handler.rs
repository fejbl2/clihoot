use actix_rt::System;
use common::terminal::terminal_actor::TerminalStop;

use crate::teacher::terminal::TeacherTerminal;

impl TerminalStop for TeacherTerminal {
    fn stop(&mut self) -> anyhow::Result<()> {
        System::current().stop();
        Ok(())
    }
}

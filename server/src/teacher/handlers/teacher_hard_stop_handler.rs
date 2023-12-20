use actix::{Context, Handler};
use actix_rt::System;

use crate::{messages::teacher_messages::TeacherHardStop, teacher::init::Teacher};

impl Handler<TeacherHardStop> for Teacher {
    type Result = ();

    fn handle(&mut self, _msg: TeacherHardStop, _: &mut Context<Self>) {
        System::current().stop();
    }
}

use actix::{Context, Handler};
use common::model::network_messages::QuestionUpdate;

use crate::teacher::init::Teacher;

impl Handler<QuestionUpdate> for Teacher {
    type Result = ();

    fn handle(&mut self, _msg: QuestionUpdate, _: &mut Context<Self>) {
        // TODO: show to the teacher
    }
}

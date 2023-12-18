use actix::{Context, Handler};
use common::model::network_messages::QuestionEnded;

use crate::teacher::init::Teacher;

impl Handler<QuestionEnded> for Teacher {
    type Result = ();

    fn handle(&mut self, _msg: QuestionEnded, _: &mut Context<Self>) {
        // TODO: show to the teacher
    }
}

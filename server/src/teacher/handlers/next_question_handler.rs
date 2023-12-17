use actix::{Context, Handler};
use common::model::network_messages::NextQuestion;

use crate::teacher::init::Teacher;

impl Handler<NextQuestion> for Teacher {
    type Result = ();

    fn handle(&mut self, _msg: NextQuestion, _: &mut Context<Self>) {
        // todo!("Show UI to the teacher");
    }
}

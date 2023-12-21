use actix::prelude::Message;

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct HardStop;

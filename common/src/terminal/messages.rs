use actix::prelude::*;
use crossterm::event::KeyCode;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Initialize;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Stop;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Redraw;

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct KeyPress {
    pub key_code: KeyCode,
}

#[derive(Message)]
#[rtype(result = "anyhow::Result<()>")]
pub struct Tick;

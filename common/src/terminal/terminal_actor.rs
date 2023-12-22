use actix::prelude::*;
use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::stdout;
use std::io::Stdout;
use std::marker::Unpin;

use crate::messages::status_messages::ClientWebsocketStatus;
use crate::messages::ServerNetworkMessage;
use crate::terminal::messages::{Initialize, KeyPress, Redraw, Stop};

pub trait TerminalDraw {
    fn redraw(&mut self, term: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()>;
}

pub trait TerminalHandleInput {
    fn handle_input(&mut self, key_code: KeyCode) -> anyhow::Result<()>;
}

pub trait TerminalHandleServerNetworkMessage {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()>;
}

pub struct TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    // base terminal actor, instantiated with struct that represents
    // its inner state
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub inner: T,
}

impl<T> TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    pub fn new(inner: T) -> Self {
        let term = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        Self {
            terminal: term,
            inner,
        }
    }
}

impl<T> Actor for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    type Context = Context<Self>;
}

impl<T> Handler<Initialize> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Initialize, _ctx: &mut Self::Context) -> Self::Result {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<Stop> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Stop, ctx: &mut Self::Context) -> Self::Result {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        ctx.stop();
        Ok(())
    }
}

impl<T> Handler<Redraw> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Redraw, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<KeyPress> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: KeyPress, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_input(msg.key_code)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<ServerNetworkMessage> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalHandleServerNetworkMessage,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ServerNetworkMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_network_message(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<ClientWebsocketStatus> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput,
{
    type Result = ();

    fn handle(&mut self, _msg: ClientWebsocketStatus, _ctx: &mut Self::Context) {
        println!("terminal actor get error message from websocket actor");
        todo!()
    }
}

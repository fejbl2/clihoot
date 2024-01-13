use actix::fut::wrap_future;
use actix::prelude::*;
use crossterm::event::KeyCode;
use log::debug;
use ratatui::backend::Backend;

use ratatui::Terminal;

use std::marker::Unpin;

use crate::constants::TICK_PERIOD_MS;
use crate::messages::network::PlayersUpdate;
use crate::messages::ServerNetworkMessage;
use crate::messages::{
    network::{NextQuestion, QuestionEnded, QuestionUpdate, ShowLeaderboard},
    status::ClientWebsocketStatus,
};
use crate::terminal::messages::{Initialize, KeyPress, Redraw, Stop, Tick};

use super::handle_terminal_events::handle_events;

pub trait TerminalDraw {
    fn redraw<B: Backend>(&mut self, term: &mut Terminal<B>) -> anyhow::Result<()>;
}

pub trait TerminalHandleInput {
    fn handle_input(&mut self, key_code: KeyCode);
}

pub trait TerminalHandleServerNetworkMessage {
    fn handle_network_message(
        &mut self,
        network_message: ServerNetworkMessage,
    ) -> anyhow::Result<()>;
}

pub trait TerminalHandleClientWebsocketStatus {
    fn handle_client_ws_status(&mut self, ws_status: ClientWebsocketStatus) -> anyhow::Result<()>;
}

pub trait TerminalHandleNextQuestion {
    fn handle_next_question(&mut self, question: NextQuestion) -> anyhow::Result<()>;
}

pub trait TerminalHandleQuestionEnded {
    fn handle_question_ended(&mut self, ended: QuestionEnded) -> anyhow::Result<()>;
}

pub trait TerminalHandleQuestionUpdate {
    fn handle_question_update(&mut self, update: QuestionUpdate) -> anyhow::Result<()>;
}

pub trait TerminalHandleShowLeaderboard {
    fn handle_show_leaderboard(&mut self, show: ShowLeaderboard) -> anyhow::Result<()>;
}

pub trait TerminalHandlePlayersUpdate {
    fn handle_players_update(&mut self, update: PlayersUpdate) -> anyhow::Result<()>;
}

pub trait TerminalHandleTick {
    fn handle_tick(&mut self) -> anyhow::Result<()>;
}

pub trait TerminalStop {
    fn stop(&mut self) -> anyhow::Result<()>;
}

#[allow(clippy::module_name_repetitions)]
pub struct TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop,
{
    // base terminal actor, instantiated with struct that represents
    // its inner state
    #[cfg(not(feature = "test"))]
    pub terminal: Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>,

    // When integration tests are run, we do not want to use the main
    // terminal to draw into; instead, provide a fake backend.
    #[cfg(feature = "test")]
    pub terminal: Terminal<ratatui::backend::TestBackend>,

    pub inner: T,
}

impl<T> TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop,
{
    #[cfg(not(feature = "test"))]
    /// Create a new terminal actor with the crossterm backend.
    ///
    /// # Panics
    /// - If the terminal cannot be initialized.
    pub fn new(inner: T) -> anyhow::Result<Self> {
        debug!("Initializing terminal actor with crossterm backend");

        let term = Terminal::new(ratatui::prelude::CrosstermBackend::new(std::io::stdout()))?;
        Ok(Self {
            terminal: term,
            inner,
        })
    }

    #[cfg(feature = "test")]
    pub fn new(inner: T) -> anyhow::Result<Self> {
        debug!("Initializing terminal actor with test backend");

        let term = Terminal::new(ratatui::backend::TestBackend::new(64, 32))?;
        Ok(Self {
            terminal: term,
            inner,
        })
    }
}

impl<T> Actor for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop + TerminalHandleTick,
{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(
            std::time::Duration::from_millis(TICK_PERIOD_MS),
            |_, ctx| {
                ctx.address().do_send(Tick);
            },
        );

        let addr = ctx.address();
        let fut = async move {
            let _ = addr.send(Initialize).await;
            let _task = tokio::spawn(handle_events(addr));
        };

        let fut = wrap_future(fut);

        ctx.spawn(fut);
    }
}

impl<T> Handler<Initialize> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop + TerminalHandleTick,
{
    type Result = anyhow::Result<()>;

    #[cfg(not(feature = "test"))]
    fn handle(&mut self, _msg: Initialize, _ctx: &mut Self::Context) -> Self::Result {
        use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
        use crossterm::ExecutableCommand;

        enable_raw_mode()?;
        std::io::stdout().execute(EnterAlternateScreen)?;
        self.inner.redraw(&mut self.terminal)
    }

    #[cfg(feature = "test")]
    fn handle(&mut self, _msg: Initialize, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<Stop> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop + TerminalHandleTick,
{
    type Result = anyhow::Result<()>;

    #[cfg(not(test))]
    fn handle(&mut self, _msg: Stop, ctx: &mut Self::Context) -> Self::Result {
        use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
        use crossterm::ExecutableCommand;

        disable_raw_mode()?;
        std::io::stdout().execute(LeaveAlternateScreen)?;
        ctx.stop();
        self.inner.stop()?;
        Ok(())
    }

    #[cfg(test)]
    fn handle(&mut self, _msg: Stop, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
        self.inner.stop()?;
        Ok(())
    }
}

impl<T> Handler<Redraw> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop + TerminalHandleTick,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Redraw, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<KeyPress> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop + TerminalHandleTick,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: KeyPress, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_input(msg.key_code);
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<Tick> for TerminalActor<T>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop + TerminalHandleTick,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, _msg: Tick, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_tick()?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<ServerNetworkMessage> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandleServerNetworkMessage,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ServerNetworkMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_network_message(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<ClientWebsocketStatus> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandleClientWebsocketStatus,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ClientWebsocketStatus, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_client_ws_status(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<NextQuestion> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandleNextQuestion,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: NextQuestion, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_next_question(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<QuestionEnded> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandleQuestionEnded,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: QuestionEnded, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_question_ended(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<QuestionUpdate> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandleQuestionUpdate,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: QuestionUpdate, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_question_update(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<ShowLeaderboard> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandleShowLeaderboard,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: ShowLeaderboard, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_show_leaderboard(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

impl<T> Handler<PlayersUpdate> for TerminalActor<T>
where
    T: 'static
        + Unpin
        + TerminalDraw
        + TerminalHandleInput
        + TerminalStop
        + TerminalHandleTick
        + TerminalHandlePlayersUpdate,
{
    type Result = anyhow::Result<()>;

    fn handle(&mut self, msg: PlayersUpdate, _ctx: &mut Self::Context) -> Self::Result {
        self.inner.handle_players_update(msg)?;
        self.inner.redraw(&mut self.terminal)
    }
}

use actix::Addr;
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers};
use futures::{FutureExt, StreamExt};
use std::marker::Unpin;

use crate::terminal::messages::{KeyPress, Redraw, Stop};
use crate::terminal::terminal_actor::{TerminalActor, TerminalDraw, TerminalHandleInput};

use super::terminal_actor::TerminalStop;

// poll terminal events and send appropriate messages to the terminal actor
pub async fn handle_events<T>(term: Addr<TerminalActor<T>>) -> anyhow::Result<()>
where
    T: 'static + Unpin + TerminalDraw + TerminalHandleInput + TerminalStop,
{
    let mut reader = EventStream::new();

    loop {
        let crossterm_event = reader.next().fuse();

        tokio::select! {
            maybe_event = crossterm_event => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => {
                        if key.kind == KeyEventKind::Press {
                            // we are in raw mode, so we need to handle this ourselves
                            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                                term.send(Stop).await??;
                                return Ok(())
                            }

                            term.send(KeyPress { key_code: key.code}).await??;
                        }
                    }
                    Some(Ok(Event::Resize(_,_))) => {
                        term.send(Redraw).await??;
                    }
                    Some(Err(e)) => return Err(e.into()),
                    None => {}
                    _ => todo!()
                }
            }
        }
    }
}

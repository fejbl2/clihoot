use crate::terminal::student::{StudentTerminal, StudentTerminalState};
use common::messages::status_messages::ClientWebsocketStatus;
use common::terminal::terminal_actor::TerminalHandleClientWebsocketStatus;

impl TerminalHandleClientWebsocketStatus for StudentTerminal {
    fn handle_client_ws_status(&mut self, ws_status: ClientWebsocketStatus) -> anyhow::Result<()> {
        match ws_status {
            ClientWebsocketStatus::ListeningFail => {
                self.state = StudentTerminalState::Error {
                    message: "Listening on websocket failed".to_string(),
                }
            }
            ClientWebsocketStatus::CantSendMessage => {
                self.state = StudentTerminalState::Error {
                    message: "Message cannot be send over websocket".to_string(),
                }
            }
            // TODO is this error as well ?
            ClientWebsocketStatus::SocketClosed => self.state = StudentTerminalState::EndGame,
            ClientWebsocketStatus::CloseFrameReceived(message) => {
                self.state = StudentTerminalState::Error { message }
            }
        }
        Ok(())
    }
}

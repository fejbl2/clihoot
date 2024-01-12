use common::{
    messages::status_messages::ClientWebsocketStatus,
    terminal::terminal_actor::TerminalHandleClientWebsocketStatus,
};

use crate::{
    music_actor::MusicMessage,
    student::{
        state::{ErrorState, StudentTerminalState},
        terminal::StudentTerminal,
    },
};

impl TerminalHandleClientWebsocketStatus for StudentTerminal {
    fn handle_client_ws_status(&mut self, ws_status: ClientWebsocketStatus) -> anyhow::Result<()> {
        match ws_status {
            ClientWebsocketStatus::ListeningFail => {
                self.music_address.do_send(MusicMessage::NoMusic);
                self.state = StudentTerminalState::Error(ErrorState {
                    message: "Listening on websocket failed".to_string(),
                })
            }
            ClientWebsocketStatus::CantSendMessage => {
                self.music_address.do_send(MusicMessage::NoMusic);
                self.state = StudentTerminalState::Error(ErrorState {
                    message: "Message cannot be send over websocket".to_string(),
                })
            }
            ClientWebsocketStatus::SocketClosed => {
                self.music_address.do_send(MusicMessage::NoMusic);
                self.state = StudentTerminalState::EndGame
            }
            ClientWebsocketStatus::CloseFrameReceived(message) => {
                self.music_address.do_send(MusicMessage::NoMusic);
                self.state = StudentTerminalState::Error(ErrorState { message })
            }
        }
        Ok(())
    }
}

use actix::prelude::*;
use log::debug;
use ratatui::style::Color;

use uuid::Uuid;

use common::{
    messages::network::PlayerData,
    terminal::{
        highlight::Theme,
        terminal_actor::{TerminalActor, TerminalStop},
    },
};

use crate::{
    music_actor::{MusicActor, MusicMessage},
    student::state::StudentTerminalState,
    websocket::WebsocketActor,
};

pub struct StudentTerminal {
    pub uuid: Uuid,
    pub name: String,
    pub color: Color,
    pub quiz_name: String,
    pub syntax_theme: Theme,
    pub help_visible: bool,
    pub players: Vec<PlayerData>,
    pub ws_actor_address: Addr<WebsocketActor>,
    pub state: StudentTerminalState,
    pub music_address: Addr<MusicActor>,
}

impl StudentTerminal {
    #[must_use]
    pub fn new(
        uuid: Uuid,
        quiz_name: String,
        ws_addr: Addr<WebsocketActor>,
        music_address: Addr<MusicActor>,
        syntax_theme: Theme,
    ) -> Self {
        Self {
            uuid,
            name: String::new(),
            color: Color::default(),
            quiz_name,
            help_visible: false,
            players: Vec::new(),
            ws_actor_address: ws_addr,
            state: StudentTerminalState::StartGame,
            music_address,
            syntax_theme,
        }
    }
}

impl TerminalStop for StudentTerminal {
    fn stop(&mut self) -> anyhow::Result<()> {
        debug!("Stopping terminal actor for student");
        System::current().stop(); // we don't have to save or clean anything on the client side
        Ok(())
    }
}

pub async fn run_student(
    uuid: Uuid,
    quiz_name: String,
    ws_actor_addr: Addr<WebsocketActor>,
    music_actor_addr: Addr<MusicActor>,
    syntax_theme: Theme,
) -> anyhow::Result<Addr<TerminalActor<StudentTerminal>>> {
    let term = TerminalActor::new(StudentTerminal::new(
        uuid,
        quiz_name,
        ws_actor_addr,
        music_actor_addr.clone(),
        syntax_theme,
    ))
    .start();

    music_actor_addr.do_send(MusicMessage::Lobby);

    Ok(term)
}

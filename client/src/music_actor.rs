use actix::prelude::*;
use actix::Message;
use rodio::{OutputStream, OutputStreamHandle};
use rodio::{Sink, Source};
use std::io::{BufReader, Cursor};

use log::error;

const LOBBY_MUSIC: &[u8] = include_bytes!("../assets/lobby.mp3");
const COUNTDOWN_MUSIC: &[u8] = include_bytes!("../assets/countdown.mp3");

const CHANGED_SELECTION_SOUND: &[u8] = include_bytes!("../assets/tap_sound.mp3");
const ENTER_PRESSED_SOUND: &[u8] = include_bytes!("../assets/enter_pressed_sound.wav");
const CORRECT_ANSWER_SOUND: &[u8] = include_bytes!("../assets/correct_answer_sound.mp3");
const WRONG_ANSWER_SOUND: &[u8] = include_bytes!("../assets/wrong_answer_sound.mp3");
const GONG_SOUND: &[u8] = include_bytes!("../assets/gong_sound.mp3");

#[derive(Message)]
#[rtype(result = "()")]
pub enum MusicMessage {
    Lobby,
    Countdown,
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum SoundEffectMessage {
    Tap,
    EnterPressed,
    CorrectAnswer,
    WrongAnswer,
    Gong,
}

impl MusicMessage {
    fn get_content(&self) -> &'static [u8] {
        match self {
            MusicMessage::Lobby => LOBBY_MUSIC,
            MusicMessage::Countdown => COUNTDOWN_MUSIC,
        }
    }
}

impl SoundEffectMessage {
    fn get_content(&self) -> &'static [u8] {
        match self {
            SoundEffectMessage::Tap => CHANGED_SELECTION_SOUND,
            SoundEffectMessage::EnterPressed => ENTER_PRESSED_SOUND,
            SoundEffectMessage::CorrectAnswer => CORRECT_ANSWER_SOUND,
            SoundEffectMessage::WrongAnswer => WRONG_ANSWER_SOUND,
            SoundEffectMessage::Gong => GONG_SOUND,
        }
    }
}

pub struct MusicActor {
    sink: Option<Sink>,
    _stream: Option<OutputStream>, // we must hold this to not drop it - like a cake above luxury carpet
    stream_handle: Option<OutputStreamHandle>,
}

impl MusicActor {
    #[must_use]
    pub fn new(silent: bool) -> Self {
        if !silent {
            if let Ok((stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(sink) = Sink::try_new(&stream_handle) {
                    sink.set_volume(0.5); // music should have less volume then sound effects
                    return MusicActor {
                        sink: Some(sink),
                        _stream: Some(stream),
                        stream_handle: Some(stream_handle),
                    };
                }
            }
            error!("Failed to open stream to music device, no music will be played during game.");
        }

        MusicActor {
            sink: None,
            _stream: None,
            stream_handle: None,
        }
    }
}

impl Actor for MusicActor {
    type Context = Context<Self>;
}

impl Handler<MusicMessage> for MusicActor {
    type Result = ();

    fn handle(&mut self, msg: MusicMessage, _: &mut Context<Self>) {
        let Some(sink) = &self.sink else {
            return; // just ignore the message if music device was not initialized
        };

        sink.stop(); // stop currently playing music

        let reader = BufReader::new(Cursor::new(msg.get_content()));

        if let Ok(source) = rodio::Decoder::new(reader) {
            sink.append(source);
        } else {
            error!("Failed to decode the music.");
        }
    }
}

impl Handler<SoundEffectMessage> for MusicActor {
    type Result = ();

    fn handle(&mut self, msg: SoundEffectMessage, _: &mut Context<Self>) {
        let Some(stream_handle) = &self.stream_handle else {
            return; // just ignore the message if music device was not initialized
        };

        let reader = BufReader::new(Cursor::new(msg.get_content()));

        if let Ok(source) = rodio::Decoder::new(reader) {
            stream_handle.play_raw(source.convert_samples()).unwrap();
        } else {
            error!("Failed to decode the sound effect.");
        }
    }
}

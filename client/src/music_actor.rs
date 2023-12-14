use actix::prelude::*;
use actix::Message;
use rodio::OutputStream;
use rodio::Sink;
use std::io::{BufReader, Cursor};

const HAPPY_MUSIC: &[u8] = include_bytes!("../assets/happy.wav");
const SAD_MUSIC: &[u8] = include_bytes!("../assets/sad.wav");
const ANGRY_MUSIC: &[u8] = include_bytes!("../assets/angry.wav");

#[derive(Message)]
#[rtype(result = "()")]
pub enum MusicMessage {
    Happy,
    Sad,
    Angry,
}

impl MusicMessage {
    fn get_content(&self) -> &'static [u8] {
        match self {
            MusicMessage::Happy => HAPPY_MUSIC,
            MusicMessage::Sad => SAD_MUSIC,
            MusicMessage::Angry => ANGRY_MUSIC,
        }
    }
}

pub struct MusicActor {
    sink: Option<Sink>,
    _stream: Option<OutputStream>, // we must hold this to not drop it - like a cake above luxury carpet
}

impl MusicActor {
    pub fn new() -> Self {
        if let Ok((stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                return MusicActor {
                    sink: Some(sink),
                    _stream: Some(stream),
                };
            }
        }
        eprintln!("Failed to open stream to music device, no music will be played during game.");
        MusicActor {
            sink: None,
            _stream: None,
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
            eprintln!("Failed to decode the music.");
        }
    }
}

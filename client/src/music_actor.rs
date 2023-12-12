use actix::prelude::*;
use actix::Message;
use rodio::OutputStream;
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

#[derive(Message)]
#[rtype(result = "()")]
pub enum MusicMessage {
    Happy,
    Sad,
    Angry,
}

impl MusicMessage {
    fn get_file_path(&self) -> &'static str {
        match self {
            MusicMessage::Happy => "assets/happy.wav",
            MusicMessage::Sad => "assets/sad.wav",
            MusicMessage::Angry => "assets/angry.wav",
        }
    }
}

pub struct MusicActor {
    sink: Sink,
    stream: OutputStream, // we must hold this to not drop it - like a cake above luxury carpet
}

impl MusicActor {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        MusicActor { sink, stream}
    }
}

impl Actor for MusicActor {
    type Context = Context<Self>;
}

impl Handler<MusicMessage> for MusicActor {
    type Result = ();

    fn handle(&mut self, msg: MusicMessage, _: &mut Context<Self>) {
        self.sink.stop(); // stop currently playing music

        let file_path = msg.get_file_path();

        if let Ok(file) = File::open(file_path) {
            if let Ok(source) = rodio::Decoder::new(BufReader::new(file)) {
                self.sink.append(source);
            }else {
                eprintln!("Failed to decode the music file: {file_path}");
            }
        } else {
            eprintln!("Failed to open the music file: {file_path}");
        }
    }
}

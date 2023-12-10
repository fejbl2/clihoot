use actix::prelude::*;
use actix::Message;
use rodio::OutputStream;
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub enum MusicMessage {
    Happy(Duration),
    Sad(Duration),
    Angry(Duration),
}

impl MusicMessage {
    fn duration(&self) -> Duration {
        match self {
            MusicMessage::Happy(duration)
            | MusicMessage::Sad(duration)
            | MusicMessage::Angry(duration) => *duration,
        }
    }
}

pub struct MusicActor {
    sink: Sink,
}

impl MusicActor {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        MusicActor { sink }
    }
}

impl Actor for MusicActor {
    type Context = Context<Self>;
}

impl Handler<MusicMessage> for MusicActor {
    type Result = ();

    fn handle(&mut self, msg: MusicMessage, _: &mut Context<Self>) {
        let file_path = match msg {
            MusicMessage::Happy(_) => "assets/happy.wav",
            MusicMessage::Sad(_) => "assets/sad.wav",
            MusicMessage::Angry(_) => "assets/angry.wav",
        };

        if let Ok(file) = File::open(file_path) {
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            self.sink.append(source);

            self.sink.sleep_until_end();
            self.sink.stop();
        } else {
            eprintln!("Failed to open the music file: {file_path}");
        }
    }
}

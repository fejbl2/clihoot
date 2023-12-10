mod music_actor;

use actix::prelude::*;
use music_actor::MusicActor;
use music_actor::MusicMessage;
use std::time::Duration;

#[actix_rt::main]
async fn main() {
    let music_actor = MusicActor::new().start();
    println!("Music actor is created.");

    music_actor
        .send(MusicMessage::Happy(Duration::from_secs(10)))
        .await
        .unwrap();
    println!("Happy music is playing.");
    music_actor
        .send(MusicMessage::Sad(Duration::from_secs(8)))
        .await
        .unwrap();
    println!("Sad music is playing.");
    music_actor
        .send(MusicMessage::Angry(Duration::from_secs(15)))
        .await
        .unwrap();
    println!("Angry music is playing.");
}

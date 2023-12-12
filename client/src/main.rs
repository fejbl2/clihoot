mod music_actor;

use actix::prelude::*;
use music_actor::MusicActor;
use music_actor::MusicMessage;
use std::thread;
use std::time::Duration;

#[actix_rt::main]
async fn main() {
    let music_actor = MusicActor::new().start();
    println!("Music actor is created.");

    music_actor.send(MusicMessage::Happy).await.unwrap();
    println!("Happy music is playing.");

    thread::sleep(Duration::from_millis(5000));

    music_actor.send(MusicMessage::Sad).await.unwrap();
    println!("Sad music is playing.");

    thread::sleep(Duration::from_millis(5000));

    music_actor.send(MusicMessage::Angry).await.unwrap();
    println!("Angry music is playing.");

    thread::sleep(Duration::from_millis(10000));
}

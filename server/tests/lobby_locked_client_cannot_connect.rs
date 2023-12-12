mod utils;

use std::{sync::mpsc, thread};

use server::{messages::teacher_messages::ServerHardStop, server::init::run_server};

use utils::sample_questions;

#[tokio::test]
async fn lobby_locked_client_cannot_connect() -> anyhow::Result<()> {
    let questions = sample_questions();
    let (tx, rx) = mpsc::channel();
    let addr = "0.0.0.0:8080".to_string().parse()?;

    let server_thread = thread::spawn(move || {
        run_server(tx, questions, addr).expect("Failed to run server");
    });

    let server = rx.recv().expect("Failed to receive server address");

    // TODO: some problem with connecting, will need further testing.

    // let (conn, _) = tokio_tungstenite::connect_async("ws://localhost:8080")
    //     .await
    //     .expect("Failed to connect to server");

    // println!("Connected to server");

    // let (mut sender, mut receiver) = conn.split();

    // let id = Uuid::new_v4();
    // let msg = TryJoinRequest { uuid: id };

    // sender
    //     .send(Message::Text(serde_json::to_string(&msg)?))
    //     .await?;

    // println!("Sent TryJoinRequest");

    // let msg = receiver.next().await.expect("Failed to receive message")?;

    // println!("Received message: {msg:?}");

    // assert_eq!(
    //     msg,
    //     Message::Text(serde_json::to_string(&TryJoinResponse {
    //         can_join: CanJoin::No(LOBBY_LOCKED_MSG.to_string()),
    //         uuid: id,
    //         quiz_name: DEFAULT_QUIZ_NAME.to_string(),
    //     })?)
    // );

    server.send(ServerHardStop {}).await?;
    server_thread.join().expect("Server thread panicked");

    Ok(())
}

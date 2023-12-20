use std::{path::Path, thread, time::Duration};

use anyhow::{bail, Ok};
use common::messages::network::{
    AnswerSelected, CanJoin, JoinRequest, JoinResponse, NextQuestion, PlayerData, PlayersUpdate,
    QuestionEnded, QuestionUpdate, ShowLeaderboard, TryJoinRequest, TryJoinResponse,
};
use common::messages::ServerNetworkMessage;
use common::questions;
use common::{constants::DEFAULT_PORT, messages::ClientNetworkMessage};
use futures_util::SinkExt;
use futures_util::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use tokio::net::TcpStream;

use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::CloseFrame;
use tungstenite::Message;
use uuid::Uuid;

use std::net::TcpListener;

#[must_use]
pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_ok()
}

#[must_use]
pub fn sample_questions() -> questions::QuestionSet {
    questions::QuestionSet::from_file(Path::new("../common/tests/files/ok_minimal.yaml")).unwrap()
}

pub type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

#[allow(dead_code)]
pub async fn connect_to_server() -> (Sender, Receiver) {
    thread::sleep(Duration::from_millis(100));

    let (conn, _) = tokio_tungstenite::connect_async(format!("ws://localhost:{DEFAULT_PORT}"))
        .await
        .expect("Failed to connect to server");

    let (sender, receiver) = conn.split();

    (sender, receiver)
}

#[allow(dead_code)]
pub async fn try_join_server(
    sender: &mut Sender,
    receiver: &mut Receiver,
) -> anyhow::Result<(Uuid, TryJoinResponse)> {
    thread::sleep(Duration::from_millis(100));

    let id = Uuid::new_v4();
    let msg = ClientNetworkMessage::TryJoinRequest(TryJoinRequest { uuid: id });

    sender
        .send(Message::Text(serde_json::to_string(&msg)?))
        .await?;

    let msg = receiver.next().await.expect("Failed to receive message")?;

    let Message::Text(msg) = msg else {
        bail!("Expected Text message")
    };

    let msg = serde_json::from_str::<ServerNetworkMessage>(&msg)?;

    let ServerNetworkMessage::TryJoinResponse(msg) = msg else {
        bail!("Expected TryJoinResponse")
    };

    Ok((id, msg))
}

#[allow(dead_code)]
pub async fn join_server(
    sender: &mut Sender,
    receiver: &mut Receiver,
    id: Uuid,
) -> anyhow::Result<(PlayerData, JoinResponse)> {
    thread::sleep(Duration::from_millis(100));

    let random_string_color = Uuid::new_v4().to_string();
    let random_string_nickname = Uuid::new_v4().to_string();

    let player_data = PlayerData {
        color: random_string_color,
        nickname: random_string_nickname,
        uuid: id,
    };

    let msg = ClientNetworkMessage::JoinRequest(JoinRequest {
        player_data: player_data.clone(),
    });

    sender
        .send(Message::Text(serde_json::to_string(&msg)?))
        .await?;

    let msg = receiver.next().await.expect("Failed to receive message")?;

    let Message::Text(msg) = msg else {
        bail!("Expected Text message")
    };

    let msg = serde_json::from_str::<ServerNetworkMessage>(&msg)?;

    let ServerNetworkMessage::JoinResponse(msg) = msg else {
        bail!("Expected JoinResponse")
    };

    Ok((player_data, msg))
}

/// Generates a new player uuid, connects to the server and joins it.
/// # Errors
/// - if the server cannot be joined, will panic.
#[allow(dead_code)]
pub async fn join_new_player() -> anyhow::Result<(Sender, Receiver, PlayerData)> {
    let (mut sender, mut receiver) = connect_to_server().await;
    let (id, _msg) = try_join_server(&mut sender, &mut receiver).await?;
    let (player_data, res) = join_server(&mut sender, &mut receiver, id).await?;

    // And it must be correct
    assert_eq!(res.can_join, CanJoin::Yes);
    assert_eq!(res.uuid, id);

    Ok((sender, receiver, player_data))
}

#[allow(dead_code)]
pub async fn receive_server_network_msg(
    receiver: &mut Receiver,
) -> anyhow::Result<ServerNetworkMessage> {
    let msg = receiver.next().await.expect("Failed to receive message")?;
    let msg = msg.to_text()?;
    let msg = serde_json::from_str::<ServerNetworkMessage>(msg)?;

    Ok(msg)
}

#[allow(dead_code)]
pub async fn receive_close_frame(receiver: &mut Receiver) -> anyhow::Result<CloseFrame> {
    let msg = receiver.next().await.expect("Failed to receive message")?;

    let Message::Close(Some(msg)) = msg else {
        bail!("Expected CloseFrame")
    };

    Ok(msg)
}

#[allow(dead_code)]
pub async fn receive_next_question(receiver: &mut Receiver) -> anyhow::Result<NextQuestion> {
    let ServerNetworkMessage::NextQuestion(question) = receive_server_network_msg(receiver).await?
    else {
        bail!("Expected NextQuestion")
    };

    Ok(question)
}

#[allow(dead_code)]
pub async fn receive_question_update(receiver: &mut Receiver) -> anyhow::Result<QuestionUpdate> {
    let ServerNetworkMessage::QuestionUpdate(update) = receive_server_network_msg(receiver).await?
    else {
        bail!("Expected QuestionUpdate")
    };

    Ok(update)
}

#[allow(dead_code)]
pub async fn receive_question_ended(receiver: &mut Receiver) -> anyhow::Result<QuestionEnded> {
    let ServerNetworkMessage::QuestionEnded(ended) = receive_server_network_msg(receiver).await?
    else {
        bail!("Expected QuestionEnded")
    };

    Ok(ended)
}

#[allow(dead_code)]
pub async fn receive_players_update(receiver: &mut Receiver) -> anyhow::Result<PlayersUpdate> {
    let ServerNetworkMessage::PlayersUpdate(update) = receive_server_network_msg(receiver).await?
    else {
        bail!("Expected PlayersUpdate")
    };

    Ok(update)
}

#[allow(dead_code)]
pub async fn receive_show_leaderboard(receiver: &mut Receiver) -> anyhow::Result<ShowLeaderboard> {
    let ServerNetworkMessage::ShowLeaderboard(show) = receive_server_network_msg(receiver).await?
    else {
        bail!("Expected ShowLeaderboard")
    };

    Ok(show)
}

#[allow(dead_code)]
pub async fn send_question_answer(
    sender: &mut Sender,
    player: &PlayerData,
    question: &questions::QuestionCensored,
    index: usize,
    selected_options: Vec<usize>, // indexes of selected options
) -> anyhow::Result<()> {
    let answer = ClientNetworkMessage::AnswerSelected(AnswerSelected {
        player_uuid: player.uuid,
        question_index: index,
        answers: question
            .choices
            .iter()
            .enumerate()
            .filter_map(|(index, choice)| {
                if selected_options.contains(&index) {
                    Some(choice.id)
                } else {
                    None
                }
            })
            .collect(),
    });

    // send the answer
    sender
        .send(Message::Text(serde_json::to_string(&answer)?))
        .await?;

    Ok(())
}

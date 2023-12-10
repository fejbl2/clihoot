use actix::prelude::*;
use actix::{Actor, Context, Message};

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use tokio::net::TcpStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};

use serde::{Deserialize, Serialize};

// message used for request to send something server, this message should be passed to websocket actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketMsg<T: Serialize + Send> {
    pub content: T,
}

// this message is send to all subscribers
#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageFromServer {
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe(pub Recipient<MessageFromServer>);

// actor which represents a gateway to the server, one can send it a request for sending a message or
// just subscribe for incoming messages
pub struct WebsocketActor {
    ws_stream_tx: Arc<
        RefCell<
            SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::protocol::Message>,
        >,
    >,
    ws_stream_rx: Arc<RefCell<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    subscribers: Arc<Mutex<RefCell<Vec<Recipient<MessageFromServer>>>>>,
}

impl WebsocketActor {
    pub async fn new() -> Self {
        let url = url::Url::parse("ws://localhost:6000").unwrap();
        let (ws_stream, _) = connect_async(url).await.expect("Client failed to connect");

        let (tx, rx) = ws_stream.split();

        WebsocketActor {
            ws_stream_rx: Arc::new(RefCell::new(rx)),
            ws_stream_tx: Arc::new(RefCell::new(tx)),
            subscribers: Arc::new(Mutex::new(RefCell::new(vec![]))),
        }
    }
}

// handler for message requests from another local actors
impl<T: Serialize + Send> Handler<WebsocketMsg<T>> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMsg<T>, ctx: &mut Context<Self>) {
        let ws_stream = Arc::clone(&self.ws_stream_tx);

        let serialized_message =
            serde_json::to_string(&msg.content).expect("cannot serialize message for websocket");

        async move {
            println!("Client websocket actor: sending message");
            ws_stream
                .borrow_mut()
                .send(tungstenite::Message::Text(serialized_message))
                .await
                .unwrap();
        }
        .into_actor(self)
        .wait(ctx);
    }
}

impl Handler<Subscribe> for WebsocketActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        println!("Client websocket actor: subscribing");
        self.subscribers.lock().unwrap().borrow_mut().push(msg.0);
    }
}

impl Actor for WebsocketActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Websocket actor is alive");

        let ws_stream_rx = Arc::clone(&self.ws_stream_rx);
        let subscribers_clone = Arc::clone(&self.subscribers);

        async move {
            // listen for messages from server
            while let Ok(incoming_msg) = ws_stream_rx.borrow_mut().next().await.unwrap() {
                let incoming_msg_text = incoming_msg.to_text().unwrap().to_string();

                let subscribers_locked = subscribers_clone.lock().unwrap();

                println!(
                    "message arrived, sending to all {} subscribers",
                    subscribers_locked.borrow().len()
                );
                for sub in subscribers_locked.borrow().iter() {
                    sub.send(MessageFromServer {
                        content: incoming_msg_text.clone(),
                    })
                    .await
                    .unwrap();
                }
            }
            println!("Client websocket closed.");
        }
        .into_actor(self)
        .spawn(ctx);
    }
}

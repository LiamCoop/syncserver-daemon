use ciborium::{from_reader, into_writer};
use futures_util::{stream::SplitSink, SinkExt};
use futures_util::{stream::SplitStream, StreamExt};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::messages::WSMessage;

pub async fn send<T: Serialize>(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    map: T,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    let _ = into_writer(&map, &mut bytes);

    let msg = Message::Binary(bytes);
    sender.send(msg).await?;
    Ok(())
}

pub async fn receive(
    receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<WSMessage, Box<dyn std::error::Error>> {
    let message = receiver.next().await.unwrap()?;
    match message {
        Message::Binary(bytes) => {
            let result: WSMessage = from_reader(bytes.as_slice())?;
            Ok(result)
        }
        _ => Err(Box::from("unexpected message type")),
    }
}

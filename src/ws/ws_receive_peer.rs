use ciborium::{from_reader, Value};
use futures_util::{stream::SplitStream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub async fn receive_peer(
    receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let message = receiver.next().await.unwrap()?;
    match message {
        Message::Binary(bytes) => {
            // check for type "peer"
            let reader_value: Value = from_reader(bytes.as_slice())?;
            let map = reader_value.into_map().unwrap();
            let (_, type_value) = map
                .iter()
                .find(|(k, _)| k == &ciborium::value::Value::Text("type".to_string()))
                .unwrap();

            if type_value != &ciborium::value::Value::Text("peer".to_string()) {
                return Err(Box::from("unexpected message type"));
            }

            let (_, peer_id_value) = map
                .iter()
                .find(|(k, _)| k == &ciborium::value::Value::Text("peerId".to_string()))
                .unwrap();

            let peer_id_result = peer_id_value.clone().into_text();
            let peer_id = match peer_id_result {
                Ok(id) => id,
                Err(_) => return Err(Box::from("")),
            };

            Ok(peer_id)
        }
        _ => Err(Box::from("unexpected message type")),
    }
}

#[cfg(test)]
mod tests {
    use crate::ws::ws_conn_open::open_ws_conn;

    use super::*;
    use ciborium::{into_writer, Value};
    use futures_util::SinkExt;
    use tokio::{net::TcpListener, task::JoinHandle};
    use tokio_tungstenite::{accept_async, tungstenite::Message};

    async fn setup_mock_server() -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("ws://{}", addr);

        // Spawn the server and have it send the peer received message
        let handle = tokio::spawn(async move {
            let map = Value::Map(vec![
                (
                    Value::Text("type".to_string()),
                    Value::Text("peer".to_string()),
                ),
                (
                    Value::Text("peerId".to_string()),
                    Value::Text("test-peer-id".to_string()),
                ),
                (
                    Value::Text("supportedProtocolVersions".to_string()),
                    Value::Array(vec![Value::Text("1".to_string())]),
                ),
            ]);

            let mut bytes = Vec::new();
            let _ = into_writer(&map, &mut bytes);

            let message = Message::Binary(bytes);

            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(stream).await.unwrap();
            // Return peer received message
            ws.send(message).await.unwrap();
        });

        (url, handle)
    }

    #[tokio::test]
    async fn test_receive_peer_succeeds() {
        let (url, _server) = setup_mock_server().await;
        let (_sender, mut receiver) = open_ws_conn(&url).await.unwrap();

        let result = receive_peer(&mut receiver).await;

        let peer_id = result.unwrap();
        assert!(
            peer_id == "test-peer-id",
            "received peerId: {:?}, expected: {:?}",
            peer_id,
            "test-peer-id"
        );
    }
}

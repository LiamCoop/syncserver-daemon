use futures_util::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::ws::send_receive::receive;

#[cfg(test)]
mod tests {
    use crate::ws::conn_open::open_ws_conn;

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

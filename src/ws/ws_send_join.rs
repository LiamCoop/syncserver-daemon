use ciborium::{into_writer, Value};
use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub async fn send_join(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    peer_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = Value::Map(vec![
        (
            Value::Text("senderId".to_string()),
            Value::Text(peer_id.to_string()),
        ),
        (
            Value::Text("supportedProtocolVersions".to_string()),
            Value::Array(vec![Value::Text("1".to_string())]),
        ),
    ]);
    let mut bytes = Vec::new();
    let _ = into_writer(&map, &mut bytes);

    let msg = Message::Binary(bytes);
    sender.send(msg).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ws::ws_conn_open::open_ws_conn;

    use super::*;
    use futures_util::StreamExt;
    use tokio::net::TcpListener;
    use tokio_tungstenite::{accept_async, tungstenite::Message};

    async fn setup_mock_server() -> (String, tokio::task::JoinHandle<Option<Message>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("ws://{}", addr);

        // Spawn the server and have it capture the first message it receives
        // JoinHandle is like a Promise — you can await it to get the result
        let handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = accept_async(stream).await.unwrap();
            // Return whatever the client sends us
            ws.next().await.map(|msg| msg.unwrap())
        });

        (url, handle)
    }

    #[tokio::test]
    async fn test_send_join_succeeds() {
        let (url, server) = setup_mock_server().await;
        let (mut sender, _receiver) = open_ws_conn(&url).await.unwrap();

        let result = send_join(&mut sender, "test-peer-id").await;
        assert!(result.is_ok(), "send_join should succeed");

        // Await the server handle to confirm it received something
        let received = server.await.unwrap();
        assert!(received.is_some(), "server should have received a message");
    }

    #[tokio::test]
    async fn test_send_join_sends_binary_message() {
        let (url, server) = setup_mock_server().await;
        let (mut sender, _receiver) = open_ws_conn(&url).await.unwrap();

        send_join(&mut sender, "test-peer-id").await.unwrap();

        let received = server.await.unwrap().unwrap();
        // The automerge-repo protocol uses binary WebSocket frames for CBOR
        assert!(received.is_binary(), "join message should be binary");
    }

    #[tokio::test]
    async fn test_send_join_cbor_contains_peer_id() {
        let (url, server) = setup_mock_server().await;
        let (mut sender, _receiver) = open_ws_conn(&url).await.unwrap();

        let peer_id = "my-test-peer-id";
        send_join(&mut sender, peer_id).await.unwrap();

        let received = server.await.unwrap().unwrap();
        let bytes = received.into_data();

        // Decode the CBOR and verify the peer ID is in there
        let decoded: ciborium::value::Value = ciborium::de::from_reader(bytes.as_slice()).unwrap();

        // The join message should be a CBOR map
        assert!(decoded.is_map(), "join message should be a CBOR map");

        // Find the senderId key and check its value
        if let ciborium::value::Value::Map(entries) = decoded {
            let sender_id_entry = entries
                .iter()
                .find(|(k, _)| k == &ciborium::value::Value::Text("senderId".to_string()));
            assert!(
                sender_id_entry.is_some(),
                "join message should contain senderId"
            );

            let (_, value) = sender_id_entry.unwrap();
            assert_eq!(
                value,
                &ciborium::value::Value::Text(peer_id.to_string()),
                "senderId should match the provided peer ID"
            );
        }
    }

    #[tokio::test]
    async fn test_send_join_cbor_contains_protocol_version() {
        let (url, server) = setup_mock_server().await;
        let (mut sender, _receiver) = open_ws_conn(&url).await.unwrap();

        send_join(&mut sender, "test-peer-id").await.unwrap();

        let received = server.await.unwrap().unwrap();
        let bytes = received.into_data();

        let decoded: ciborium::value::Value = ciborium::de::from_reader(bytes.as_slice()).unwrap();

        if let ciborium::value::Value::Map(entries) = decoded {
            let version_entry = entries.iter().find(|(k, _)| {
                k == &ciborium::value::Value::Text("supportedProtocolVersions".to_string())
            });
            assert!(
                version_entry.is_some(),
                "join message should contain supportedProtocolVersions"
            );

            let (_, value) = version_entry.unwrap();
            assert_eq!(
                value,
                &ciborium::value::Value::Array(vec![ciborium::value::Value::Text("1".to_string())]),
                "supportedProtocolVersions should be [\"1\"]"
            );
        }
    }
}

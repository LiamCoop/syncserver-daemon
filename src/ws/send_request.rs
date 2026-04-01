use automerge::sync;
use ciborium::Value;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::send_receive::send;

pub async fn send_request(
    document_id: &str,
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: &str,
    target_id: &str,
    data: sync::Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = Value::Map(vec![
        (
            Value::Text("type".to_string()),
            Value::Text("request".to_string()),
        ),
        (
            Value::Text("documentId".to_string()),
            Value::Text(document_id.to_string()),
        ),
        (
            Value::Text("senderId".to_string()),
            Value::Text(sender_id.to_string()),
        ),
        (
            Value::Text("targetId".to_string()),
            Value::Text(target_id.to_string()),
        ),
        (Value::Text("data".to_string()), Value::Bytes(data.encode())),
    ]);
    let _ = send(sender, map).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

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

    // is message sent as binary websocket frame?
    #[tokio::test]
    async fn test_send_receive_sends_binary_message() {
        unimplemented!()
    }

    // is it successful
    #[tokio::test]
    async fn test_send_receive_succeeds() {
        unimplemented!()
    }
}

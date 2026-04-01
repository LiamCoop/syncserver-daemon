use automerge::sync;
use ciborium::Value;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::send_receive::send;

pub async fn send_unavailable(
    document_id: &str,
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: &str,
    target_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = Value::Map(vec![
        (
            Value::Text("type".to_string()),
            Value::Text("doc-unavailable".to_string()),
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
    ]);
    let _ = send(sender, map).await?;
    Ok(())
}

use ciborium::Value;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::send_receive::send;

pub async fn send_ephemeral(
    document_id: &str,
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: &str,
    target_id: &str,
    count: u32,
    session_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = Value::Map(vec![
        (
            Value::Text("type".to_string()),
            Value::Text("ephemeral".to_string()),
        ),
        (
            Value::Text("senderId".to_string()),
            Value::Text(sender_id.to_string()),
        ),
        (
            Value::Text("targetId".to_string()),
            Value::Text(target_id.to_string()),
        ),
        (
            Value::Text("count".to_string()),
            Value::Integer(Into::into(count)),
        ),
        (
            Value::Text("sessionId".to_string()),
            Value::Text(session_id.to_string()),
        ),
        (
            Value::Text("documentId".to_string()),
            Value::Text(document_id.to_string()),
        ),
        (
            Value::Text("data".to_string()),
            // The data of this message (in practice this is arbitrary CBOR), I'm not sure what to
            // do here.
            Value::Text(document_id.to_string()),
        ),
    ]);
    let _ = send(sender, map).await?;
    Ok(())
}

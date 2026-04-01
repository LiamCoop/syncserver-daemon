use ciborium::Value;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::send_receive::send;

pub async fn send_remote_subscription_changed(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: &str,
    target_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = Value::Map(vec![
        (
            Value::Text("type".to_string()),
            Value::Text("remote-subscription-change".to_string()),
        ),
        (
            Value::Text("senderId".to_string()),
            Value::Text(sender_id.to_string()),
        ),
        (
            Value::Text("targetId".to_string()),
            Value::Text(target_id.to_string()),
        ),
        /*
              ; The storage IDs to add to the subscription
              ? add: [* storage_id]

              ; The storage IDs to remove from the subscription
              remove: [* storage_id]
        */
    ]);
    let _ = send(sender, map).await?;
    Ok(())
}

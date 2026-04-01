use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::messages::{PeerMetadata, WSMessage};
use crate::ws::send_receive::send;

pub async fn send_peer(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    selected_protocol_versions: Vec<String>,
    metadata: PeerMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Peer {
        sender_id,
        target_id,
        selected_protocol_versions: selected_protocol_versions,
        metadata: metadata,
    }
    .into();
    let _ = send(sender, map).await?;
    Ok(())
}

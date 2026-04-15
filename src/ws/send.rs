use automerge::sync;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::ws::{
    messages::{PeerMetadata, WSMessage},
    send_receive::send,
};

pub async fn send_ephemeral(
    document_id: String,
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    count: u32,
    session_id: String,
    data: Vec<u8>,
    metadata: Option<PeerMetadata>,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Ephemeral {
        sender_id,
        target_id,
        session_id,
        document_id,
        count,
        data,
        metadata,
    };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_error(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Error { message };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_join(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    supported_protocol_version: String,
    metadata: Option<PeerMetadata>,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Join {
        sender_id,
        supported_protocol_version,
        metadata,
    };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_leave(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Leave { sender_id };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_peer(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    selected_protocol_version: String,
    metadata: Option<PeerMetadata>,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Peer {
        sender_id,
        target_id,
        selected_protocol_version,
        metadata,
    };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_remote_subscription_changed(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    add: Option<Vec<String>>,
    remove: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::RemoteSubscriptionChange {
        sender_id,
        target_id,
        add,
        remove,
    };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_request(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    document_id: String,
    data: sync::Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Request {
        sender_id,
        target_id,
        document_id,
        data: data.encode(),
    };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_unavailable(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    document_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Unavailable {
        sender_id,
        target_id,
        document_id,
    };
    let _ = send(sender, map).await?;
    Ok(())
}

pub async fn send_sync(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    sender_id: String,
    target_id: String,
    document_id: String,
    data: sync::Message,
) -> Result<(), Box<dyn std::error::Error>> {
    let map = WSMessage::Sync {
        sender_id,
        target_id,
        document_id,
        data: data.encode(),
    };
    let _ = send(sender, map).await?;
    Ok(())
}

use automerge::sync::SyncDoc;
use automerge::AutoCommit;
use clap::Parser;
use futures_util::stream::{SplitSink, SplitStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::ws::conn_open::open_ws_conn;
use crate::ws::messages::{PeerMetadata, WSMessage};
use crate::ws::send::send_join;
use crate::ws::send_receive::receive;

mod ws;

#[derive(Parser)]
struct Cli {
    // automerge document url
    doc_url: String,
    automerge_url: String,
    // The path to sync the document referenced by URL into.
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let sync_server_url = args.automerge_url;

    let (mut sender, mut receiver) = open_ws_conn(&sync_server_url).await.unwrap();

    // generate random sender ID (our id)
    let sender_id = uuid::Uuid::new_v4().to_string();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Parse the document ID out of your doc_url
    let _doc_id = parse_doc_id(&args.doc_url)?;
    // Generate an initial sync message from your empty doc
    let mut doc = AutoCommit::new();

    let mut sync_state = automerge::sync::State::new();
    let _m = doc
        .sync()
        .generate_sync_message(&mut sync_state)
        .ok_or("failed to generate sync message")?;

    let _receiver_id = handshake(&mut sender, &mut receiver, sender_id).await;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        /*
        receive sync messages
            → apply to AutoCommit doc
                → extract doc.content as String
                    → write to file
        */

        match receive(&mut receiver).await? {
            WSMessage::Peer {
                sender_id,
                target_id,
                selected_protocol_version,
                metadata,
            } => todo!(),
            WSMessage::Ephemeral {
                sender_id,
                target_id,
                count,
                session_id,
                document_id,
                data,
                metadata,
            } => todo!(),
            WSMessage::Error { message } => todo!(),
            WSMessage::Join {
                sender_id,
                supported_protocol_version,
                metadata,
            } => todo!(),
            WSMessage::Leave { sender_id } => todo!(),
            WSMessage::Request {
                sender_id,
                target_id,
                document_id,
                data,
            } => todo!(),
            WSMessage::Sync {
                sender_id,
                target_id,
                document_id,
                data,
            } => todo!(),
            WSMessage::Unavailable {
                sender_id,
                target_id,
                document_id,
            } => todo!(),
            WSMessage::RemoteSubscriptionChange {
                sender_id,
                target_id,
                add,
                remove,
            } => todo!(),
            WSMessage::RemoteHeadsChanged {
                sender_id,
                target_id,
                document_id,
                new_heads,
            } => todo!(),
        }
    }

    println!("shutting down cleanly...");
    Ok(())
}

fn parse_doc_id(url: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let (_, right) = url.split_once(":").ok_or("failed to split")?;
    Ok(right)
}

async fn handshake(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    sender_id: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let _ = send_join(
        sender,
        sender_id,
        vec!["1".to_string()],
        PeerMetadata {
            storage_id: "".to_string(),
            is_ephemeral: true,
        },
    )
    .await?;

    match receive(receiver).await? {
        WSMessage::Peer {
            sender_id,
            target_id: _,
            selected_protocol_version: _,
            metadata: _,
        } => Ok(sender_id),
        _ => Err(Box::from("expected Peer message during handshake")),
    }
}

async fn receive_message(message: WSMessage) -> Result<(), Box<dyn std::error::Error>> {
    match message {
        WSMessage::Peer {
            sender_id: _,
            target_id: _,
            selected_protocol_version: _,
            metadata: _,
        } => todo!(),
        WSMessage::Ephemeral {
            sender_id: _,
            target_id: _,
            count: _,
            session_id: _,
            document_id: _,
            data: _,
            metadata: _,
        } => todo!(),
        WSMessage::Error { message: _ } => todo!(),
        WSMessage::Join {
            sender_id: _,
            supported_protocol_version: _,
            metadata: _,
        } => todo!(),
        WSMessage::Leave { sender_id: _ } => todo!(),
        WSMessage::Request {
            sender_id: _,
            target_id: _,
            document_id: _,
            data: _,
        } => todo!(),
        WSMessage::Sync {
            sender_id: _,
            target_id: _,
            document_id: _,
            data: _,
        } => todo!(),
        WSMessage::Unavailable {
            sender_id: _,
            target_id: _,
            document_id: _,
        } => todo!(),
        WSMessage::RemoteSubscriptionChange {
            sender_id: _,
            target_id: _,
            add: _,
            remove: _,
        } => todo!(),
        WSMessage::RemoteHeadsChanged {
            sender_id: _,
            target_id: _,
            document_id: _,
            new_heads: _,
        } => todo!(),
    }
}

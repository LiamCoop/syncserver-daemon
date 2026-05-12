use automerge::sync::Message;
use automerge::sync::SyncDoc;
use automerge::transaction::Transactable;
use automerge::AutoCommit;
use automerge::ReadDoc;
use clap::Parser;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use notify::{self, Config, RecommendedWatcher, Watcher};
use similar::{DiffOp, TextDiff};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::channel;
use tokio_tungstenite::tungstenite::Message as TungsteniteMessage;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::ws::conn_open::open_ws_conn;
use crate::ws::messages::{PeerMetadata, WSMessage};
use crate::ws::send::{send_error, send_join, send_request, send_sync};
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
    env_logger::init();
    let args = Cli::parse();

    // start file watching the local file, default number of buffered events: 32
    let (tx, mut rx) = channel(32);
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            tx.blocking_send(res).unwrap();
        },
        Config::default(),
    )?;

    watcher.watch(&args.path.clone(), notify::RecursiveMode::Recursive)?;

    let sync_server_url = args.automerge_url;

    let (mut sender, mut receiver) = open_ws_conn(&sync_server_url).await.unwrap();

    // generate random sender ID (our id)
    let sender_id = uuid::Uuid::new_v4().to_string();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Parse the document ID out of your doc_url
    let doc_id = parse_doc_id(&args.doc_url)?;
    // Generate an initial sync message from your empty doc
    let mut doc = AutoCommit::new();

    let mut sync_state = automerge::sync::State::new();
    let m = doc
        .sync()
        .generate_sync_message(&mut sync_state)
        .ok_or("failed to generate sync message")?;

    // if the handshake fails we terminate
    let receiver_id = handshake(&mut sender, &mut receiver, sender_id.clone()).await?;

    let _ = send_request(
        &mut sender,
        sender_id.clone(),
        receiver_id.clone(),
        doc_id.to_string(),
        m,
    )
    .await;

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        tokio::select! {
            msg = receive(&mut sender, &mut receiver) => {
                    match msg? {
                    WSMessage::Peer {
                        sender_id,
                        target_id: _,
                        selected_protocol_version: _,
                        metadata: _,
                    } => log::error!("received peer with sender id: {}", sender_id),
                    WSMessage::Ephemeral {
                        sender_id,
                        target_id: _,
                        count: _,
                        session_id: _,
                        document_id: _,
                        data: _,
                        metadata: _,
                    } => log::info!("received ephemeral with sender id: {}", sender_id),
                    WSMessage::Error { message } => {
                        log::error!("error received from sync server: {}", message)
                    }
                    WSMessage::Join {
                        sender_id,
                        supported_protocol_version: _,
                        metadata: _,
                    } => log::info!("id: {} has joined", sender_id),
                    WSMessage::Leave { sender_id } => log::info!("id: {} has left", sender_id),
                    WSMessage::Request {
                        sender_id,
                        target_id: _,
                        document_id: _,
                        data: _,
                    } => log::info!("received request with sender id: {}", sender_id),
                    WSMessage::Sync {
                        sender_id: sync_sender_id,
                        target_id: _,
                        document_id: _,
                        data,
                    } => {
                        let message = Message::decode(&data).unwrap();
                        if let Err(e) = doc.sync().receive_sync_message(&mut sync_state, message) {
                            log::error!("failed to apply sync message: {}", e);
                            continue;
                        }
                        let option = doc.sync().generate_sync_message(&mut sync_state);
                        match option {
                            Some(msg) => {
                                if let Err(e) = send_sync(
                                    &mut sender,
                                    sender_id.clone(),
                                    sync_sender_id,
                                    doc_id.to_string(),
                                    msg,
                                )
                                .await
                                {
                                    return Err(e);
                                };
                            }
                            // we're done collecting information from a faraway land
                            None => {
                                let option = doc.get(automerge::ROOT, "content").unwrap();
                                match option {
                                    Some((_, obj_id)) => {
                                        let doc_content = doc.text(obj_id)?;
                                        log::info!("writing {} chars: {:?}", doc_content.len(), &doc_content[..50.min(doc_content.len())]);
                                        std::fs::write(args.path.clone(), doc_content)?;
                                    }
                                    None => todo!(),
                                }
                            }
                        }
                    }
                    WSMessage::Unavailable {
                        sender_id,
                        target_id: _,
                        document_id: _,
                    } => log::info!("got unavailable from {}", sender_id),
                    WSMessage::RemoteSubscriptionChange {
                        sender_id,
                        target_id: _,
                        add: _,
                        remove: _,
                    } => log::info!(
                        "received remote subscription change with sender id: {}",
                        sender_id
                    ),
                    WSMessage::RemoteHeadsChanged {
                        sender_id,
                        target_id: _,
                        document_id: _,
                        new_heads: _,
                    } => log::info!(
                        "received remote heads changed with sender id: {}",
                        sender_id
                    ),
                }
            }
            _event = rx.recv() => {
                // log::info!("file changed: {:?}", event);
                let file = std::fs::read(args.path.clone())?;
                let file_content = String::from_utf8(file)?;
                let option = doc.get(automerge::ROOT, "content").unwrap();
                match option {
                    Some((_, obj_id)) => {
                        let doc_content = doc.text(&obj_id)?;
                        let diff = TextDiff::from_chars(&doc_content, &file_content);
                        // start at the end so we don't need to keep track of indices as they change
                        for op in diff.ops().iter().rev() {
                            match op {
                                DiffOp::Equal{ old_index: _old_index, new_index: _new_index, len: _len } => {},
                                DiffOp::Delete{ old_index, old_len, new_index: _ } => {
                                    doc.splice_text(&obj_id, *old_index, *old_len as isize, "")?;

                                }
                                DiffOp::Insert{ old_index, new_index, new_len } => {
                                    doc.splice_text(&obj_id, *old_index, 0, &file_content[*new_index..new_index + new_len])?;
                                },
                                DiffOp::Replace{ old_index, old_len, new_index, new_len } => {
                                    doc.splice_text(&obj_id, *old_index, *old_len as isize, &file_content[*new_index..new_index + new_len])?;
                                },
                            }
                        }
                    }
                    None => todo!(),
                }
                if let Some(data) = doc.sync().generate_sync_message(&mut sync_state) {
                    let _ = send_sync(&mut sender, sender_id.clone(), receiver_id.clone(), doc_id.to_string(), data).await;
                };
            }
        }
    }

    println!("shutting down cleanly...");
    Ok(())
}

fn parse_doc_id(url: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let (_, right) = url.split_once(":").ok_or("failed to split")?;
    Ok(right)
}

// we may not get a peer message back, in that case an error is returned
async fn handshake(
    sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
    receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    sender_id: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let _ = send_join(
        sender,
        sender_id,
        "1".to_string(),
        Some(PeerMetadata {
            storage_id: "test".to_string(),
            is_ephemeral: true,
        }),
    )
    .await?;

    match receive(sender, receiver).await? {
        WSMessage::Peer {
            sender_id,
            target_id: _,
            selected_protocol_version,
            metadata: _,
        } => {
            if selected_protocol_version != "1" {
                // WS should send Error
                let _ = send_error(sender, "selected protocol version was not 1".to_string()).await;
                // send web socket close
                let _ = sender.send(TungsteniteMessage::Close(None));
                return Err(Box::from(format!(
                    "handshake failed: selected_protocol_version expected 1, got {:?}",
                    selected_protocol_version
                )));
            }
            Ok(sender_id)
        }
        _ => {
            // WS should send Error
            let _ = send_error(sender, "received something other than peer".to_string()).await;
            // send web socket close
            let _ = sender.send(TungsteniteMessage::Close(None));
            // TODO: drain messages
            // return error
            Err(Box::from(
                "handshake failed: client (us) sent error message",
            ))
        }
    }
}

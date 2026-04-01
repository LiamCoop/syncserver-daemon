use ciborium::{into_writer, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use automerge::sync::{Message, SyncDoc};
use automerge::AutoCommit;
use clap::Parser;

use crate::ws::conn_open::open_ws_conn;
use crate::ws::receive_peer::receive_peer;
use crate::ws::send_join::send_join;

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

    let sync_server_url = std::env::var("SYNC_SERVER_URL")?;
    // environment variable for sync server
    let (mut sender, mut receiver) = open_ws_conn(&sync_server_url).await.unwrap();

    let sender_id = uuid::Uuid::new_v4().to_string();
    let _ = send_join(&mut sender, &sender_id).await?;

    let _peer_id = receive_peer(&mut receiver).await?;

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

    // Wrap it in a request CBOR envelope

    // Send it

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        // spin off two threads, one to sync, one to send
        /*
        receive sync messages
            → apply to AutoCommit doc
                → extract doc.content as String
                    → write to file
        */
    }

    println!("shutting down cleanly...");
    Ok(())
}

fn parse_doc_id(url: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let (_, right) = url.split_once(":").ok_or("failed to split")?;
    Ok(right)
}

fn send_message(m_type: &str, doc_id: &str, sender_id: &str, receiver_id: &str, m: Message) {
    let map = Value::Map(vec![
        (
            Value::Text("senderId".to_string()),
            Value::Text(sender_id.to_string()),
        ),
        (
            Value::Text("supportedProtocolVersions".to_string()),
            Value::Array(vec![Value::Text("1".to_string())]),
        ),
    ]);
    let mut bytes = Vec::new();
    let _ = into_writer(&map, &mut bytes);
}

// use automerge::{transaction::Transactable, AutoCommit, ObjType, ReadDoc};
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod ws;

#[derive(Parser)]
struct Cli {
    // automerge document url
    doc_url: String,
    // The path to sync the document referenced by URL into.
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("url: {:?}, path: {:?}", args.doc_url, args.path);

    // handle_sigterm();
}

fn handle_sigterm() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("shutting down cleanly...");
}

AI disclaimer: All of this code was handwritten by me, however I did use AI as a learning tool and to help with the project's direction.

## syncserver-daemon

A Rust CLI daemon that connects to an [automerge-repo](https://github.com/automerge/automerge-repo) WebSocket sync server and keeps a local file in **two-way sync** with a remote Automerge document.

Designed to work alongside [md-editor](https://github.com/liamcoop/md-editor) — a real-time collaborative markdown editor. Web users and CLI users edit the same documents simultaneously, with conflict-free merging handled automatically by Automerge.

```
automerge-sync <DOC_URL> <AUTOMERGE_URL> <PATH>
```

| Argument | Description |
| --- | --- |
| `DOC_URL` | Automerge document URL (e.g. `automerge:abc123`) |
| `AUTOMERGE_URL` | WebSocket sync server URL (e.g. `wss://your-server/sync`) |
| `PATH` | Local file path to keep in sync |

---

### How it works

The daemon maintains a live WebSocket connection to the sync server and runs two loops concurrently:

- **Remote → local**: incoming sync messages are applied to the local Automerge document and the result is written to the file on disk.
- **Local → remote**: file system changes are detected, diffed character-by-character against the in-memory document, spliced into the Automerge text object, and pushed to the sync server as a new sync message.

Both sides merge cleanly — edits from a CLI user in their editor of choice and edits from web users in md-editor will reconcile automatically without data loss.

---

### Setup

```bash
cargo build --release
```

Set the log level with the `RUST_LOG` environment variable:

```bash
RUST_LOG=info ./target/release/fs-automerge-client <DOC_URL> <AUTOMERGE_URL> <PATH>
```

To get a `DOC_URL`, open a document in [md-editor](https://github.com/liamcoop/md-editor) and copy the document ID from the URL. The `AUTOMERGE_URL` is the sync server your md-editor instance is connected to.

---

### Features

- **Bidirectional sync** — local edits propagate to all connected peers; remote edits update the local file
- **Character-level diffing** — uses `similar` to compute minimal diffs before writing to the Automerge text object, preserving fine-grained history
- **Full automerge-repo protocol** — `join` → `peer` handshake, CBOR-encoded messages, all message types handled
- **File watching** — `notify` detects saves from any editor in real time
- **TLS support** — works with both `ws://` and `wss://` sync servers
- **Graceful shutdown** — Ctrl-C closes the connection cleanly

---

### Works with md-editor

[md-editor](https://github.com/liamcoop/md-editor) is a real-time collaborative markdown editor that runs in the browser. It stores documents in an Automerge sync server, which this daemon connects to directly.

Once the daemon is running, the local file and the web editor stay in sync — your teammates' edits appear in your file, and your edits appear in their browser, in real time.

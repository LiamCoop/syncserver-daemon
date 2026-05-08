## fs-automerge-client

A Rust CLI daemon that connects to an [automerge-repo](https://github.com/automerge/automerge-repo) WebSocket sync server and keeps a local file in sync with a remote Automerge document.

```
automerge-sync <DOC_URL> <AUTOMERGE_URL> <PATH>
```

---

### What works

- WebSocket connection to a sync server (ws:// and wss://)
- Full automerge-repo handshake (`join` → `peer`)
- CBOR-encoded message protocol (all message types defined and tested)
- Initial document sync: sends a sync request, applies incoming sync messages, writes document content to a local file
- Ping/pong keepalive handling
- Graceful shutdown on Ctrl-C

---

### Obvious next steps

- **Continuous sync** — the loop currently exits after the first sync round; it needs to keep receiving and applying updates as they arrive from other peers
- **Local → remote sync** — watch the local file for changes, apply edits back into the Automerge document, and send outbound sync messages to the server
- **Flexible document key** — the field used to extract content from the document is hardcoded to `"content"`; this should be configurable or auto-detected
- **Reconnect on disconnect** — no retry logic exists if the WebSocket drops
- **Clean shutdown** — the `TODO: drain messages` path on close is unfinished

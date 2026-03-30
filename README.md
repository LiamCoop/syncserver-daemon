## Rust daemon to act as an automerge sync client

TODO:

1. Open WebSocket to your sync server
2. Send CBOR-encoded `join` message (automerge-repo protocol)
3. Receive `peer` message back
4. Enter sync loop:
   - Generate sync messages via automerge's SyncDoc
   - Wrap them in CBOR `sync` messages (automerge-repo protocol)
   - Send over WebSocket
   - Receive incoming sync messages, unwrap CBOR, feed to SyncDoc

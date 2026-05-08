# fs-automerge-client Plan

## Done
- WebSocket connection and handshake (join → peer)
- Full sync protocol: request → receive sync → apply → generate → send sync
- Automerge doc content extraction (Text field)
- Write synced document content to local file
- Ping/pong keepalive handling

## Next Steps

### Live updates
- Verify the client continues syncing after the initial sync (i.e. browser changes are picked up in subsequent sync messages)
- If not, investigate whether the sync loop is exiting early after `generate_sync_message` returns `None`

### Write back (file → server)
- Watch the local file for changes (e.g. using `notify` crate)
- Apply file changes to the local `AutoCommit` doc
- Generate and send a sync message to the server with the new state

### Robustness
- Handle disconnects and reconnect gracefully
- Drain messages properly after sending a close frame (currently a TODO)
- Handle the `Unavailable` message meaningfully (document doesn't exist on server)

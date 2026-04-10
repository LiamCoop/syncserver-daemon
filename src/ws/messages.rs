use serde::{Deserialize, Serialize};

// details all the types needed for wire protocol
// https://www.npmjs.com/package/@automerge/automerge-repo-network-websocket

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerMetadata {
    storage_id: String,
    is_ephemeral: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewHeads {
    storage_id: String,
    heads: Vec<String>,
    timestamp: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WSMessage {
    #[serde(rename_all = "camelCase")]
    Peer {
        sender_id: String,
        target_id: String,
        selected_protocol_versions: Vec<String>,
        metadata: PeerMetadata,
    },
    #[serde(rename_all = "camelCase")]
    Ephemeral {
        sender_id: String,
        target_id: String,
        count: u32,
        session_id: String,
        document_id: String,
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
        metadata: PeerMetadata,
    },
    #[serde(rename_all = "camelCase")]
    Error { message: String },
    #[serde(rename_all = "camelCase")]
    Join {
        sender_id: String,
        selected_protocol_versions: Vec<String>,
        metadata: PeerMetadata,
    },
    #[serde(rename_all = "camelCase")]
    Leave { sender_id: String },
    #[serde(rename_all = "camelCase")]
    Request {
        sender_id: String,
        target_id: String,
        document_id: String,
        data: Vec<u8>,
    },
    #[serde(rename_all = "camelCase")]
    Sync {
        sender_id: String,
        target_id: String,
        document_id: String,
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
    },
    #[serde(rename = "doc-unavailable")]
    #[serde(rename_all = "camelCase")]
    Unavailable {
        sender_id: String,
        target_id: String,
        document_id: String,
    },
    #[serde(rename = "remote-subscription-change")]
    #[serde(rename_all = "camelCase")]
    RemoteSubscriptionChange {
        sender_id: String,
        target_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        add: Option<Vec<String>>,
        remove: Vec<String>,
    },
    #[serde(rename = "remote-heads-changed")]
    #[serde(rename_all = "camelCase")]
    RemoteHeadsChanged {
        sender_id: String,
        target_id: String,
        document_id: String,
        new_heads: NewHeads,
    },
}

#[cfg(test)]
mod tests {
    use crate::ws::messages::WSMessage;
    use ciborium::{from_reader, into_writer, Value};

    fn get_entries(msg: WSMessage) -> Vec<(Value, Value)> {
        let mut bytes = Vec::new();
        let _ = into_writer(&msg, &mut bytes);
        let result: Value = from_reader(bytes.as_slice()).unwrap();
        let decoded = result.into_map().unwrap();
        let entries: Vec<(Value, Value)> = decoded.into_iter().map(|(k, v)| (k, v)).collect();
        return entries;
    }

    fn get_keys(msg: WSMessage) -> Vec<String> {
        let mut bytes = Vec::new();
        let _ = into_writer(&msg, &mut bytes);
        let result: Value = from_reader(bytes.as_slice()).unwrap();
        let decoded = result.into_map().unwrap();
        let k: Vec<String> = decoded
            .into_iter()
            .map(|(k, _)| k.into_text().unwrap())
            .collect();
        return k;
    }

    mod error {
        use crate::ws::messages::{tests::get_keys, WSMessage};

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Error {
                message: "this is an error".to_string(),
            };
            let keys = get_keys(msg);
            let type_key = keys.iter().find(|k| k == &"type").unwrap();
            assert_eq!(type_key, "type");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Error {
                message: "this is an error".to_string(),
            };
            let keys = get_keys(msg);

            let type_key = keys.iter().find(|k| k == &"type").unwrap();
            assert_eq!(type_key, "type");

            let message_key = keys.iter().find(|k| k == &"message").unwrap();
            assert_eq!(message_key, "message");
        }
    }
    mod remote_subscription_change {
        use ciborium::Value;

        use crate::ws::messages::tests::{get_entries, get_keys};
        use crate::ws::messages::WSMessage;

        // type field should be kebab case
        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::RemoteSubscriptionChange {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                add: Some(vec!["storage-id-1".to_string()]),
                remove: vec![],
            };
            let keys = get_keys(msg);
            let type_key = keys.iter().find(|k| k == &"type").unwrap();
            assert_eq!(type_key, "type");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::RemoteSubscriptionChange {
                sender_id: "sender".to_string(),
                target_id: "target".to_string(),
                add: Some(vec!["storage-id-1".to_string()]),
                remove: vec![],
            };
            let keys = get_keys(msg);

            dbg!(&keys);
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"add".to_string()));
            assert!(keys.contains(&"remove".to_string()));
        }

        #[test]
        fn add_absent_when_none() {
            let msg = WSMessage::RemoteSubscriptionChange {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                add: None,
                remove: vec![],
            };
            let keys = get_keys(msg);

            assert!(!keys.contains(&"add".to_string()));
        }

        #[test]
        fn add_present_and_correct_when_some() {
            let msg = WSMessage::RemoteSubscriptionChange {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                add: Some(vec!["storage-id-1".to_string()]),
                remove: vec![],
            };
            let entries = get_entries(msg);
            let (_, v) = entries
                .iter()
                .find(|(k, _)| k.clone().into_text().unwrap() == "add".to_string())
                .unwrap();
            assert_eq!(
                v,
                &Value::Array(vec![Value::Text("storage-id-1".to_string())])
            )
        }

        #[test]
        fn remove_always_present() {
            let msg = WSMessage::RemoteSubscriptionChange {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                add: Some(vec!["storage-id-1".to_string()]),
                remove: vec![],
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"remove".to_string()));
        }
    }
}

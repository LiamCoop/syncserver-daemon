use serde::{Deserialize, Serialize};

// details all the types needed for wire protocol
// https://www.npmjs.com/package/@automerge/automerge-repo-network-websocket

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerMetadata {
    pub storage_id: String,
    pub is_ephemeral: bool,
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
        selected_protocol_version: Vec<String>,
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
        supported_protocol_version: Vec<String>,
        metadata: PeerMetadata,
    },
    #[serde(rename_all = "camelCase")]
    Leave { sender_id: String },
    #[serde(rename_all = "camelCase")]
    Request {
        sender_id: String,
        target_id: String,
        document_id: String,
        #[serde(with = "serde_bytes")]
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
    use serde::Serialize;

    fn get_entries<T: Serialize>(msg: T) -> Vec<(Value, Value)> {
        let mut bytes = Vec::new();
        let _ = into_writer(&msg, &mut bytes);
        let result: Value = from_reader(bytes.as_slice()).unwrap();
        let decoded = result.into_map().unwrap();
        let entries: Vec<(Value, Value)> = decoded.into_iter().map(|(k, v)| (k, v)).collect();
        return entries;
    }

    fn get_keys<T: Serialize>(msg: T) -> Vec<String> {
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

    fn get_type_value(msg: WSMessage) -> String {
        let entries = get_entries(msg);
        let (_k, v) = entries
            .iter()
            .find(|(k, _v)| k == &Value::Text("type".to_string()))
            .unwrap();
        // don't need to assert for type, we found it on line 134.
        let st = v.clone().into_text().unwrap();
        return st;
    }

    mod join {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            PeerMetadata, WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Join {
                sender_id: "sender".to_string(),
                supported_protocol_version: vec!["1".to_string()],
                metadata: PeerMetadata {
                    storage_id: "storage-id".to_string(),
                    is_ephemeral: true,
                },
            };
            let v = get_type_value(msg);
            assert_eq!(v, "join");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Join {
                sender_id: "sender".to_string(),
                supported_protocol_version: vec!["1".to_string()],
                metadata: PeerMetadata {
                    storage_id: "storage-id".to_string(),
                    is_ephemeral: true,
                },
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"supportedProtocolVersion".to_string()));
            assert!(keys.contains(&"metadata".to_string()));
        }
    }
    mod peer {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            PeerMetadata, WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Peer {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                selected_protocol_version: vec!["1".to_string()],
                metadata: PeerMetadata {
                    storage_id: "storage-id".to_string(),
                    is_ephemeral: true,
                },
            };
            let v = get_type_value(msg);
            assert_eq!(v, "peer");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Peer {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                selected_protocol_version: vec!["1".to_string()],
                metadata: PeerMetadata {
                    storage_id: "storage-id".to_string(),
                    is_ephemeral: true,
                },
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"selectedProtocolVersion".to_string()));
            assert!(keys.contains(&"metadata".to_string()));
        }
    }
    mod leave {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Leave {
                sender_id: "sender".to_string(),
            };
            let v = get_type_value(msg);
            assert_eq!(v, "leave");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Leave {
                sender_id: "sender".to_string(),
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"senderId".to_string()));
        }
    }
    mod request {
        use crate::ws::messages::{
            tests::{get_entries, get_keys, get_type_value},
            WSMessage,
        };
        use ciborium::Value;

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Request {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
            };
            let v = get_type_value(msg);
            assert_eq!(v, "request");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Request {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
            };
            let keys = get_keys(msg);

            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"documentId".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"data".to_string()));
        }

        #[test]
        fn data_field_is_bytes() {
            let msg = WSMessage::Request {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
            };
            let entries = get_entries(msg);
            let (_k, v) = entries
                .iter()
                .find(|(k, _v)| k == &Value::Text("data".to_string()))
                .unwrap();
            dbg!(v);
            assert!(v == &Value::Bytes(vec![0, 1, 2]));
        }
    }
    mod sync {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Sync {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
            };
            let v = get_type_value(msg);
            assert_eq!(v, "sync");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Sync {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"documentId".to_string()));
            assert!(keys.contains(&"data".to_string()));
        }
    }
    mod unavailable {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Unavailable {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
            };
            let v = get_type_value(msg);
            assert_eq!(v, "doc-unavailable");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Unavailable {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"documentId".to_string()));
        }
    }
    mod ephemeral {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            PeerMetadata, WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Ephemeral {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                count: 23,
                session_id: "session".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
                metadata: PeerMetadata {
                    storage_id: "storage-id".to_string(),
                    is_ephemeral: true,
                },
            };
            let v = get_type_value(msg);
            assert_eq!(v, "ephemeral");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Ephemeral {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                count: 23,
                session_id: "session".to_string(),
                document_id: "document".to_string(),
                data: vec![0, 1, 2],
                metadata: PeerMetadata {
                    storage_id: "storage-id".to_string(),
                    is_ephemeral: true,
                },
            };
            let keys = get_keys(msg);
            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"senderId".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"count".to_string()));
            assert!(keys.contains(&"sessionId".to_string()));
            assert!(keys.contains(&"documentId".to_string()));
            assert!(keys.contains(&"data".to_string()));
            assert!(keys.contains(&"metadata".to_string()));
        }
    }
    mod error {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::Error {
                message: "this is an error".to_string(),
            };
            let v = get_type_value(msg);
            assert_eq!(v, "error");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::Error {
                message: "this is an error".to_string(),
            };
            let keys = get_keys(msg);

            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"message".to_string()));
        }
    }
    mod remote_subscription_change {
        use ciborium::Value;

        use crate::ws::messages::tests::{get_entries, get_keys, get_type_value};
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
            let v = get_type_value(msg);
            assert_eq!(v, "remote-subscription-change");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::RemoteSubscriptionChange {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                add: Some(vec!["storage-id-1".to_string()]),
                remove: vec![],
            };
            let keys = get_keys(msg);

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
    pub mod remote_heads_changed {
        use crate::ws::messages::{
            tests::{get_keys, get_type_value},
            NewHeads, WSMessage,
        };

        #[test]
        fn type_field_is_correct() {
            let msg = WSMessage::RemoteHeadsChanged {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                new_heads: NewHeads {
                    storage_id: "storage".to_string(),
                    heads: vec!["heads".to_string()],
                    timestamp: 23,
                },
            };
            let v = get_type_value(msg);
            assert_eq!(v, "remote-heads-changed");
        }

        #[test]
        fn field_names_camelcase() {
            let msg = WSMessage::RemoteHeadsChanged {
                sender_id: "sender".to_string(),
                target_id: "receiver".to_string(),
                document_id: "document".to_string(),
                new_heads: NewHeads {
                    storage_id: "storage".to_string(),
                    heads: vec!["heads".to_string()],
                    timestamp: 23,
                },
            };
            let keys = get_keys(msg);

            assert!(keys.contains(&"type".to_string()));
            assert!(keys.contains(&"targetId".to_string()));
            assert!(keys.contains(&"documentId".to_string()));
            assert!(keys.contains(&"newHeads".to_string()));
        }
    }
    pub mod peer_metadata {
        use crate::ws::messages::{tests::get_keys, PeerMetadata};

        #[test]
        fn field_names_camelcase() {
            let msg = PeerMetadata {
                storage_id: "storage".to_string(),
                is_ephemeral: true,
            };
            let keys = get_keys(msg);

            assert!(keys.contains(&"storageId".to_string()));
            assert!(keys.contains(&"isEphemeral".to_string()));
        }
    }
    pub mod new_heads {
        use crate::ws::messages::{tests::get_keys, NewHeads};

        #[test]
        fn field_names_camelcase() {
            let msg = NewHeads {
                storage_id: "storage".to_string(),
                heads: vec!["heads".to_string()],
                timestamp: 23,
            };
            let keys = get_keys(msg);

            assert!(keys.contains(&"storageId".to_string()));
            assert!(keys.contains(&"heads".to_string()));
            assert!(keys.contains(&"timestamp".to_string()));
        }
    }
}

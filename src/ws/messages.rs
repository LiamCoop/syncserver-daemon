use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerMetadata {
    storage_id: String,
    is_ephemeral: bool,
}

/*
* bled for this, gonna leave it in for now
impl From<PeerMetadata> for Value {
    fn from(msg: PeerMetadata) -> Self {
        Value::Map(vec![
            (
                Value::Text("storageId".to_string()),
                Value::Text(msg.storage_id.to_string()),
            ),
            (
                Value::Text("isEphemeral".to_string()),
                Value::Bool(msg.is_ephemeral),
            ),
        ])
    }
}

impl TryFrom<Value> for PeerMetadata {
    type Error = Box<dyn std::error::Error>;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let result = v.into_map();
        match result {
            Ok(v) => {
                let (_, storage_id_value) = v
                    .iter()
                    .find(|(k, _)| k == &Value::Text("storageId".to_string()))
                    .ok_or("no storage_id in map")?;

                let storage_id = storage_id_value
                    .clone()
                    .into_text()
                    .map_err(|e| format!("expected a string, got: {:?}", e))?;

                let (_, is_ephemeral_value) = v
                    .iter()
                    .find(|(k, _)| k == &Value::Text("isEphemeral".to_string()))
                    .ok_or("no is_ephemeral in map")?;

                let is_ephemeral = is_ephemeral_value
                    .clone()
                    .into_bool()
                    .map_err(|e| format!("expected a bool, got: {:?}", e))?;

                Ok(PeerMetadata {
                    storage_id,
                    is_ephemeral,
                })
            }
            Err(v) => Err(v).map_err(|e| format!("expected a map, got: {:?}", e).into()),
        }
    }
}
*/

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WSMessage {
    Peer {
        sender_id: String,
        target_id: String,
        selected_protocol_versions: Vec<String>,
        metadata: PeerMetadata,
    },
}

/*
impl From<WSMessage> for Value {
    fn from(msg: WSMessage) -> Self {
        match msg {
            WSMessage::Peer {
                sender_id,
                target_id,
                selected_protocol_versions,
                metadata,
            } => Value::Map(vec![
                (
                    Value::Text("type".to_string()),
                    Value::Text("peer".to_string()),
                ),
                (Value::Text("senderId".to_string()), Value::Text(sender_id)),
                (Value::Text("targetId".to_string()), Value::Text(target_id)),
                (
                    Value::Text("selectedProtocolVersions".to_string()),
                    Value::Array(
                        selected_protocol_versions
                            .into_iter()
                            .map(|x| Value::Text(x))
                            .collect(),
                    ),
                ),
                (Value::Text("metadata".to_string()), metadata.into()),
            ]),
        }
    }
}

impl TryFrom<Value> for WSMessage {
    type Error = Box<dyn std::error::Error>;
    fn try_from(_v: Value) -> Result<Self, Self::Error> {
        // extract the map from the Value
        // find the "type" key
        // match on its string value to decide which variant to build
        // extract the remaining fields and construct the variant
        unimplemented!()
    }
}
*/

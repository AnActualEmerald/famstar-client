use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SyncMessage {
    AddImage { content: String, path: String },
    AddMessage { content: String, path: String },
    RemoveDoc { path: String },
}

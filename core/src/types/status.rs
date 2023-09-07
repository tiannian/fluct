use serde::{Deserialize, Serialize};

/// Payload Status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Valid,
    Invalid,
    Syncing,
    Accepted,
    InvalidBlockHash,
}

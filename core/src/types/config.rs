use serde::{Deserialize, Serialize};

/// Config for all service
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_state: String,
    pub genesis: String,
    pub store_state: bool,
}

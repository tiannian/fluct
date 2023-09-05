use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub chain_state: String,
    pub genesis: String,
    pub store_state: bool,
}

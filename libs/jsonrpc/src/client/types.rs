use serde::Deserialize;
use serde_json::Value;

use crate::RpcError;

pub type RpcResponses<T> = Vec<RpcResponse<T>>;

#[derive(Debug, Clone, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<RpcError>,
    pub id: Value,
}

impl<T> From<RpcResponse<T>> for Result<Option<T>, RpcError> {
    fn from(value: RpcResponse<T>) -> Self {
        match (value.result, value.error) {
            (Some(v), _) => Ok(Some(v)),
            (None, Some(e)) => Err(e),
            (None, None) => Ok(None),
        }
    }
}

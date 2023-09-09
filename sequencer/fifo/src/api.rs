use async_trait::async_trait;
use ethers_core::types::H256;
use fluct_core::{SequencerApi, Transaction};
use fluct_service::Caller;

use crate::{ApiRequest, ApiResponse, Error, Result};

#[derive(Clone)]
pub struct DevSequencerApi {
    pub(crate) caller: Caller<ApiRequest, ApiResponse>,
}

#[async_trait]
impl SequencerApi for DevSequencerApi {
    type Error = Error;

    fn broadcast_tx(&self, tx: Transaction) -> Result<()> {
        self.caller.send(ApiRequest::Transaction(tx))?;

        Ok(())
    }

    fn comfirm_tx(&self, txhash: H256) -> Result<()> {
        self.caller.send(ApiRequest::TxHash(txhash))?;

        Ok(())
    }

    async fn txs(&self) -> Result<Vec<Transaction>> {
        let resp = self.caller.call(ApiRequest::GetAllTransaction)?.await?;

        match resp {
            ApiResponse::GetAllTransaction(v) => Ok(v),
        }
    }
}

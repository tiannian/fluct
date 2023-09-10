use async_trait::async_trait;
use fluct_core::{Block, BlockHeader, ConsensusApi, ForkChoiceState};
use fluct_service::Caller;

use crate::{ApiError, ApiRequest, ApiResponse};

pub struct SingleConsensusApi {
    pub call: Caller<ApiRequest, ApiResponse>,
}

#[async_trait]
impl ConsensusApi for SingleConsensusApi {
    type Error = ApiError;

    async fn add_block(&mut self, block: Block) -> Result<(), Self::Error> {
        let mut block = block;

        let ohash = BlockHeader::from(block.clone()).hash();

        if let Some(hash) = block.hash() {
            if hash != ohash {
                return Err(ApiError::FailedVerifyBlockHash(ohash, hash));
            }
        }

        block.fill_hash(ohash);

        self.call.send(ApiRequest::AddBlock(block))?;

        Ok(())
    }

    async fn chain_state(&self) -> Result<ForkChoiceState, Self::Error> {
        Ok(Default::default())
    }
}

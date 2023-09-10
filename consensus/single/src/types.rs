use fluct_core::{Block, ForkChoiceState};

pub enum ApiRequest {
    AddBlock(Block),
    GetChainState,
}

pub enum ApiResponse {
    GetChainState(ForkChoiceState),
}

use crate::{
    types::{ConsensusGenesis, ForkChoiceState},
    EngineApi, SequencerService, Service, Transaction,
};

/// Service of Consensus
pub trait ConsensusService<E, S>: Service + Sized
where
    E: EngineApi,
    S: SequencerService,
{
    /// Create node and init node using genesis, if node isn't inited
    fn new(
        engine_api: E,
        sequencer_api: S::Api,
        sequencer: &S,
        genesis: ConsensusGenesis<Transaction>,
        state: ForkChoiceState,
    ) -> Result<Self, Self::Error>;

    /// Get chain state
    fn chain_state(&self) -> &ForkChoiceState;
}

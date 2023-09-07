use crate::{
    types::{ConsensusGenesis, ForkChoiceState},
    EngineAPI, SequencerAPI, SequencerService, Service, Transaction,
};

pub trait ConsensusService<E, S>: Service + Sized
where
    E: EngineAPI,
    S: SequencerService,
{
    /// Use genesis to init chain.
    fn new(
        engine_api: E,
        sequencer_api: S::API,
        sequencer: &S,
        genesis: ConsensusGenesis<Transaction>,
        state: ForkChoiceState,
    ) -> Result<Self, Self::Error>;

    fn chain_state(&self) -> &ForkChoiceState;
}

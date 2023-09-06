use crate::{
    types::{ConsensusGenesis, ForkChoiceState},
    EngineAPI, Sequencer, StepService, Transaction,
};

pub trait ConsensusService<E, S>: StepService + Sized
where
    E: EngineAPI,
    S: Sequencer,
{
    /// Use genesis to init chain.
    fn new(
        engine_api: E,
        seq: S,
        genesis: ConsensusGenesis<Transaction>,
    ) -> Result<Self, Self::Error>;

    fn chain_state(&self) -> &ForkChoiceState;

    fn set_state(&mut self, state: ForkChoiceState);
}

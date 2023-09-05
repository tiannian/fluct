use crate::{
    types::{ConsensusGenesis, ForkChoiceState},
    EngineAPI, Sequencer, Service, Transaction,
};

pub trait ConsensusService<E, S>: Service
where
    E: EngineAPI,
    S: Sequencer,
{
    fn set_engine_api(&mut self, api: E);

    fn set_sequencer(&mut self, seq: S);

    /// Use genesis to init chain.
    fn init(&mut self, genesis: ConsensusGenesis<Transaction>) -> Result<(), Self::Error>;

    fn chain_state(&self) -> &ForkChoiceState;

    fn set_state(&mut self, state: ForkChoiceState);
}

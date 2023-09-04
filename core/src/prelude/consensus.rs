use crate::{
    types::{ChainState, ConsensusGenesis},
    EngineAPI, SequencerAPI, Service, Transaction,
};

pub trait ConsensusService<E, S>: Service
where
    E: EngineAPI,
    S: SequencerAPI,
{
    fn set_engine_api(&mut self, api: E);

    fn set_seq_api(&mut self, api: S);

    /// Use genesis to init chain.
    fn init(&mut self, genesis: &ConsensusGenesis<Transaction>) -> Result<(), Self::Error>;

    fn chain_state(&self) -> ChainState;
}

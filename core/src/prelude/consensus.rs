use std::error::Error;

use crate::{
    types::{ChainState, Genesis},
    EngineAPI, SequencerAPI, Service, Transaction,
};

pub trait Parser {
    type Error: Error + Sync + Send + 'static;

    fn deserialize_transaction(bytes: &[u8]) -> Result<Transaction, Self::Error>;

    fn serialize_transaction(tx: &Transaction) -> Vec<u8>;
}

pub trait ConsensusService<E, S>: Service
where
    E: EngineAPI,
    S: SequencerAPI,
{
    fn set_engine_api(&mut self, api: E);

    fn set_seq_api(&mut self, api: S);

    /// Use genesis to init chain.
    fn init(&mut self, genesis: &Genesis<Transaction>) -> Result<(), Self::Error>;

    fn chain_state(&self) -> ChainState;
}

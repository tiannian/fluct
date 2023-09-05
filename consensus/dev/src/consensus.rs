use fluct_core::{
    types::{ConsensusGenesis, ForkChoiceState},
    ConsensusService, EngineAPI, Sequencer, Service, Transaction,
};

use crate::{Error, Result};

pub struct DevConseneus<S, E> {
    sequencer: S,
    engine_api: E,
    state: ForkChoiceState,
}

impl<S, E> Service for DevConseneus<S, E>
where
    S: Sync + Send,
    E: Sync + Send,
{
    type Error = Error;

    fn start(&mut self) -> Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    fn kill(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<S, E> ConsensusService<E, S> for DevConseneus<S, E>
where
    E: EngineAPI + Sync + Send,
    S: Sequencer + Sync + Send,
{
    fn set_engine_api(&mut self, api: E) {
        self.engine_api = api;
    }

    fn set_sequencer(&mut self, seq: S) {
        self.sequencer = seq;
    }

    /// Use genesis to init chain.
    fn init(&mut self, genesis: ConsensusGenesis<Transaction>) -> Result<()> {
        self.sequencer.add_txs(genesis.transactions);
        Ok(())
    }

    fn chain_state(&self) -> &ForkChoiceState {
        &self.state
    }

    fn set_state(&mut self, state: ForkChoiceState) {
        self.state = state;
    }
}

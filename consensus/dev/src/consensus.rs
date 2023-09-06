use fluct_core::{
    types::{ConsensusGenesis, ForkChoiceState},
    ConsensusService, EngineAPI, Sequencer, StepService, Transaction,
};

use crate::{Error, Result};

pub struct DevConseneus<S, E> {
    sequencer: S,
    engine_api: E,
    state: ForkChoiceState,
}

impl<S, E> DevConseneus<S, E>
where
    S: Sequencer + Sync + Send,
    E: EngineAPI + Sync + Send,
{
    fn _step(&mut self) -> Result<()> {
        let txs = self.sequencer.claim_batch();

        Ok(())
    }
}

impl<S, E> StepService for DevConseneus<S, E>
where
    S: Sequencer + Sync + Send,
    E: EngineAPI + Sync + Send,
{
    type Error = Error;

    fn step(&mut self) -> Result<()> {
        self._step()
    }
}

impl<S, E> ConsensusService<E, S> for DevConseneus<S, E>
where
    E: EngineAPI + Sync + Send,
    S: Sequencer + Sync + Send,
{
    fn new(engine_api: E, sequencer: S, genesis: ConsensusGenesis<Transaction>) -> Result<Self> {
        let mut s = Self {
            engine_api,
            sequencer,
            state: ForkChoiceState::default(),
        };

        s.sequencer.add_txs(genesis.transactions);

        Ok(s)
    }

    fn chain_state(&self) -> &ForkChoiceState {
        &self.state
    }

    fn set_state(&mut self, state: ForkChoiceState) {
        self.state = state;
    }
}

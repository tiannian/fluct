use fluct_core::{
    ConsensusGenesis, ConsensusService, EngineApi, ForkChoiceState, SequencerApi, SequencerService,
    Transaction,
};

use crate::{Error, Result};

pub struct DevConseneus<S, E> {
    sequencer_api: S,
    engine_api: E,
    state: ForkChoiceState,
}

impl<S, E> DevConseneus<S, E>
where
    S: SequencerApi + Sync + Send,
    E: EngineApi + Sync + Send,
{
    pub fn new(
        engine_api: E,
        sequencer_api: S,
        genesis: ConsensusGenesis<Transaction>,
        state: ForkChoiceState,
    ) -> Result<Self, S> {
        let txs = genesis.transactions;

        for tx in txs {
            sequencer_api
                .broadcast_tx(tx)
                .map_err(|e| Error::SequencerApiError(e))?;
        }

        Ok(Self {
            sequencer_api,
            engine_api,
            state,
        })
    }
}

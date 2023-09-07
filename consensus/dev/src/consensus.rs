use fluct_core::{
    types::{ConsensusGenesis, ForkChoiceState},
    ConsensusService, EngineAPI, SequencerAPI, SequencerService, Transaction,
};

use crate::{Error, Result};

pub struct DevConseneus<'a, S, E>
where
    S: SequencerService,
{
    sequencer: &'a S,
    sequencer_api: S::API,
    engine_api: E,
    state: ForkChoiceState,
}

impl<'a, S, E> DevConseneus<'a, S, E>
where
    S: SequencerService + Sync + Send,
    E: EngineAPI + Sync + Send,
{
    pub fn new(
        engine_api: E,
        sequencer_api: S::API,
        sequencer: &'a S,
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
            sequencer,
            sequencer_api,
            engine_api,
            state,
        })
    }
}

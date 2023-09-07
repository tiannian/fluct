use std::{fs, path::Path};

use anyhow::Result;
use ethers_core::types::Bytes;

use crate::{
    types::{self, Config, Genesis},
    ConsensusService, ExecutionService, SequencerService,
};

pub struct Node<C, E, S> {
    consensus: C,
    execution: E,
    config: Config,
    sequencer: S,
}

impl<C, S, E> Node<C, E, S>
where
    C: ConsensusService<E::API, S>,
    S: SequencerService,
    E: ExecutionService,
{
    pub fn new(sequencer: S, execution: E, config: Config) -> Result<Self> {
        let mut execution = execution;

        let eapi = execution.api()?;
        let sapi = sequencer.api();

        // Genesis
        let gss = fs::read_to_string(&config.genesis)?;
        let genesis: Genesis<Bytes, E::Genesis> = serde_json::from_str(&gss)?;
        let genesis = genesis.from_transaction()?;

        // Chain State
        let csp = Path::new(&config.chain_state);
        let state = if csp.exists() {
            let css = fs::read_to_string(&config.chain_state)?;
            let state: types::ForkChoiceState = serde_json::from_str(&css)?;
            state
        } else {
            execution.init(genesis.execution)?;
            Default::default()
        };

        let consensus = C::new(eapi, sapi, &sequencer, genesis.consensus, state)?;

        Ok(Self {
            consensus,
            execution,
            config,
            sequencer,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        // Check is empty chain? Init it.

        self.sequencer.start()?;
        self.execution.start()?;
        self.consensus.start()?;

        if self.config.store_state {
            // Running Backend thread to write chain state
        }

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        Ok(())
    }
}

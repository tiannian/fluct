use std::{fs, marker::PhantomData, path::Path};

use anyhow::Result;
use ethers_core::types::Bytes;

use crate::{
    types::{self, Config, Genesis},
    ConsensusService, ExecutionService, Sequencer,
};

pub struct Node<C, E, S> {
    consensus: C,
    execution: E,
    config: Config,
    marker_s: PhantomData<S>,
}

impl<C, S, E> Node<C, E, S>
where
    C: ConsensusService<E::API, S>,
    S: Sequencer,
    E: ExecutionService,
{
    pub fn new(consensus: C, sequencer: S, execution: E, config: Config) -> Result<Self> {
        let mut consensus = consensus;
        let mut execution = execution;

        let eapi = execution.api()?;

        consensus.set_sequencer(sequencer);
        consensus.set_engine_api(eapi);

        // Genesis
        let gss = fs::read_to_string(&config.genesis)?;
        let genesis: Genesis<Bytes, E::Genesis> = serde_json::from_str(&gss)?;
        let genesis = genesis.from_transaction()?;

        // Chain State
        let csp = Path::new(&config.chain_state);
        if csp.exists() {
            let css = fs::read_to_string(&config.chain_state)?;
            let state: types::ForkChoiceState = serde_json::from_str(&css)?;
            consensus.set_state(state);
        } else {
            consensus.init(genesis.consensus)?;
            execution.init(genesis.execution)?;
        }

        Ok(Self {
            consensus,
            execution,
            config,
            marker_s: PhantomData,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        // Check is empty chain? Init it.

        self.execution.start()?;
        self.consensus.start()?;

        if self.config.store_state {
            // Running Backend thread to write chain state
        }

        Ok(())
    }
}

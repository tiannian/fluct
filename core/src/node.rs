use std::{
    fs,
    marker::PhantomData,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::Result;
use ethers_core::types::Bytes;

use crate::{
    types::{self, Config, Genesis},
    ConsensusService, ExecutionService, Sequencer,
};

pub struct Node<C, E, S> {
    consensus: C,
    exit_flag: AtomicBool,
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
    pub fn new(sequencer: S, execution: E, config: Config) -> Result<Self> {
        let mut execution = execution;

        let eapi = execution.api()?;

        // Genesis
        let gss = fs::read_to_string(&config.genesis)?;
        let genesis: Genesis<Bytes, E::Genesis> = serde_json::from_str(&gss)?;
        let genesis = genesis.from_transaction()?;

        let mut consensus = C::new(eapi, sequencer, genesis.consensus)?;

        // Chain State
        let csp = Path::new(&config.chain_state);
        if csp.exists() {
            let css = fs::read_to_string(&config.chain_state)?;
            let state: types::ForkChoiceState = serde_json::from_str(&css)?;
            consensus.set_state(state);
        } else {
            execution.init(genesis.execution)?;
        }

        Ok(Self {
            consensus,
            execution,
            config,
            marker_s: PhantomData,
            exit_flag: AtomicBool::new(true),
        })
    }

    pub fn start(&mut self) -> Result<()> {
        // Check is empty chain? Init it.

        self.execution.start()?;

        while self.exit_flag.load(Ordering::Relaxed) {
            self.consensus.step()?;

            if self.config.store_state {
                // Running Backend thread to write chain state
            }
        }

        self.execution.stop()?;

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.exit_flag.store(false, Ordering::Relaxed);

        Ok(())
    }
}

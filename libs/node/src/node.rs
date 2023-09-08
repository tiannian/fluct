use std::{fs, path::Path};

use anyhow::Result;
use ethers_core::types::Bytes;
use fluct_core::{
    Config, ConsensusService, ExecutionService, ForkChoiceState, Genesis, SequencerService,
};

pub struct Node<C, E, S> {
    consensus: C,
    execution: E,
    config: Config,
    sequencer: S,
}

impl<C, S, E> Node<C, E, S>
where
    C: ConsensusService,
    S: SequencerService,
    E: ExecutionService,
{
    pub fn new(sequencer: S, execution: E, consensus: C, config: Config) -> Result<Self> {
        let mut execution = execution;
        let mut sequencer = sequencer;
        let mut consensus = consensus;

        let eapi = execution.engine_api()?;
        let wapi = execution.web3_api()?;
        let sapi = sequencer.api();

        let wapi2 = execution.web3_api()?;

        // Genesis
        let gss = fs::read_to_string(&config.genesis)?;
        let genesis: Genesis<Bytes, E::Genesis> = serde_json::from_str(&gss)?;
        let genesis = genesis.from_transaction()?;

        // Chain State
        let csp = Path::new(&config.chain_state);
        let state = if csp.exists() {
            let css = fs::read_to_string(&config.chain_state)?;
            let state: ForkChoiceState = serde_json::from_str(&css)?;
            state
        } else {
            execution.init(genesis.execution)?;
            Default::default()
        };

        sequencer.set_api(wapi2);
        consensus.set_api(eapi, wapi, sapi);

        consensus.init(genesis.consensus, state)?;

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

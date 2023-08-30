use std::{fs, marker::PhantomData, path::Path};

use anyhow::Result;
use ethers_core::types::Bytes;

use crate::{
    types::{ChainState, Config, ConsensusGenesis, Genesis},
    ConsensusService, ExecutionService, Parser, SequencerService, Transaction,
};

pub struct Node<C, S, E, P>
where
    E: ExecutionService,
{
    consensus: C,
    sequencer: S,
    execution: E,
    chain_state: ChainState,
    consensus_genesis: ConsensusGenesis<Transaction>,
    execution_genesis: E::Genesis,
    marker_p: PhantomData<P>,
}

impl<C, S, E, P> Node<C, S, E, P>
where
    C: ConsensusService<E::API, S::API>,
    S: SequencerService,
    E: ExecutionService,
    P: Parser,
{
    pub fn new(consensus: C, sequencer: S, execution: E, config: Config) -> Result<Self> {
        let mut consensus = consensus;

        let sapi = sequencer.api();
        let eapi = execution.api();

        consensus.set_seq_api(sapi);
        consensus.set_engine_api(eapi);

        // Chain State
        let csp = Path::new(&config.chain_state);
        let chain_state = if csp.exists() {
            let css = fs::read_to_string(config.chain_state)?;

            serde_json::from_str(&css)?
        } else {
            ChainState::default()
        };

        // Genesis
        let gss = fs::read_to_string(config.genesis)?;
        let genesis: Genesis<Bytes, E::Genesis> = serde_json::from_str(&gss)?;

        let mut txs = Vec::with_capacity(genesis.consensus.transactions.len());

        for tx in &genesis.consensus.transactions {
            let tx = P::deserialize_transaction(tx)?;
            txs.push(tx);
        }

        Ok(Self {
            consensus,
            sequencer,
            execution,
            chain_state,
            consensus_genesis: (genesis.consensus, txs).into(),
            execution_genesis: genesis.execution,
            marker_p: PhantomData,
        })
    }

    pub fn start(&mut self) -> Result<()> {
        // Check is empty chain? Init it.
        if self.chain_state == ChainState::default() {
            self.consensus.init(&self.consensus_genesis)?;
            self.execution.init(&self.execution_genesis)?;
        }

        self.execution.start()?;
        self.sequencer.start()?;
        self.consensus.start()?;

        // Running Backend thread to write chain state

        Ok(())
    }
}

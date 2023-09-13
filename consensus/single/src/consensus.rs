use async_trait::async_trait;
use ethers_core::types::H160;
use ethers_signers::Signer;
use fluct_core::{
    ConsensusGenesis, EngineApi, ForkChoiceState, SequencerApi, Transaction, Web3Api,
};
use fluct_service::{AsyncStepService, AsyncStepServiceWapper1};

use crate::Error;

pub struct SingleConsensus<S, SA> {
    sequencer_api: Option<SA>,
    engine_api: Option<Box<dyn EngineApi + Send + Sync>>,
    signer: Option<S>,
    state: ForkChoiceState,
    proposer: H160,
    init_txs: Vec<Transaction>,
}

impl<S, SA> SingleConsensus<S, SA>
where
    S: Signer,
{
    pub fn new_proposer(signer: S) -> Self {
        let proposer = signer.address();

        Self {
            sequencer_api: None,
            engine_api: None,
            state: Default::default(),
            signer: Some(signer),
            proposer,
            init_txs: Vec::new(),
        }
    }

    pub fn new_follower(proposer: H160) -> Self {
        Self {
            sequencer_api: None,
            engine_api: None,
            state: Default::default(),
            signer: None,
            proposer,
            init_txs: Vec::new(),
        }
    }
}

#[async_trait]
impl<S, SA> AsyncStepService for SingleConsensus<S, SA>
where
    SA: SequencerApi + Send + Sync,
    S: Send + Sync,
{
    type Error = Error;

    async fn step(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

pub struct SingleConsensusService<S, SA>(AsyncStepServiceWapper1<SingleConsensus<S, SA>>);

/* impl<S, SA> SingleConsensus<S, SA>
where
    SA: SequencerApi,
    S: Signer,
{
    fn init(
        &mut self,
        genesis: ConsensusGenesis<Transaction>,
        state: ForkChoiceState,
    ) -> Result<(), Error<SA>> {
        self.init_txs = genesis.transactions;
        self.state = state;

        Ok(())
    }

    fn set_api(
        &mut self,
        engine_api: impl EngineApi + 'static,
        _web3_api: impl Web3Api,
        seqencer_api: SA,
    ) -> Result<(), Error<SA>> {
        self.engine_api = Some(Box::new(engine_api));
        self.sequencer_api = Some(seqencer_api);

        Ok(())
    }
} */

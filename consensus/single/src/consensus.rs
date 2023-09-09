use ethers_core::types::H160;
use ethers_signers::Signer;
use fluct_core::ForkChoiceState;

pub struct SingleConsensus<SequencerApi, EngineApi, Signer> {
    sequencer_api: Option<SequencerApi>,
    engine_api: Option<EngineApi>,
    signer: Option<Signer>,
    state: ForkChoiceState,
    proposer: H160,
}

impl<SequencerApi, EngineApi, SignerT> SingleConsensus<SequencerApi, EngineApi, SignerT>
where
    SignerT: Signer,
{
    pub fn new_proposer(signer: SignerT) -> Self {
        let proposer = signer.address();

        Self {
            sequencer_api: None,
            engine_api: None,
            state: Default::default(),
            signer: Some(signer),
            proposer,
        }
    }

    pub fn new_follower(proposer: H160) -> Self {
        Self {
            sequencer_api: None,
            engine_api: None,
            state: Default::default(),
            signer: None,
            proposer,
        }
    }
}

use anyhow::Result;

use crate::{ConsensusService, ExecutionService, SequencerService};

pub struct Node<C, S, E> {
    pub consensus: C,
    pub sequencer: S,
    pub execution: E,
}

impl<C, S, E> Node<C, S, E>
where
    C: ConsensusService<E::API, S::API>,
    S: SequencerService,
    E: ExecutionService,
{
    pub fn new(consensus: C, sequencer: S, execution: E) -> Self {
        let mut consensus = consensus;

        let sapi = sequencer.api();
        let eapi = execution.api();

        consensus.set_seq_api(sapi);
        consensus.set_engine_api(eapi);

        Self {
            consensus,
            sequencer,
            execution,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        self.execution.start()?;
        self.sequencer.start()?;
        self.consensus.start()?;

        Ok(())
    }
}

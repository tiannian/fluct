mod service;
pub use service::*;

mod engine;
pub use engine::*;

mod seq;
pub use seq::*;

pub trait ConsensusService<E, S>: Service
where
    E: EngineAPI,
    S: SequencerAPI,
{
    fn set_engine_api(&mut self, api: E);

    fn set_seq_api(&mut self, api: S);
}

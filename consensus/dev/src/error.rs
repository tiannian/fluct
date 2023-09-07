use fluct_core::{SequencerAPI, SequencerApiError, SequencerService};

#[derive(Debug)]
pub enum Error<S>
where
    S: SequencerService,
{
    SequencerApiError(SequencerApiError<S>),
}

pub type Result<T, S> = std::result::Result<T, Error<S>>;

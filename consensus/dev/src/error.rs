use fluct_core::SequencerApi;

#[derive(Debug)]
pub enum Error<S>
where
    S: SequencerApi,
{
    SequencerApiError(S::Error),
}

pub type Result<T, S> = std::result::Result<T, Error<S>>;

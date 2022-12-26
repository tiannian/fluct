use fluct_core::define_from_error;

#[derive(Debug)]
pub enum Error {
    CoreError(fluct_core::Error),

    MissingFieldToBuildRuntime,
}

define_from_error!(fluct_core::Error, Error, CoreError);

pub type Result<T> = std::result::Result<T, Error>;

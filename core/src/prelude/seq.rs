use std::fmt::Debug;

use crate::{Service, Transaction};

pub trait SequencerAPI {
    type SeqError: Debug;

    fn pack_transaction(&self) -> Result<Vec<Transaction>, Self::SeqError>;
}

pub trait SequencerService: Service {
    type API: SequencerAPI;

    fn api(&self) -> Self::API;
}

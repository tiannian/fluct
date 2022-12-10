use evm::Config;
use fluct_core::Store;
use primitive_types::U256;

use crate::{CoreVicinity, Error, Result, Runtime};

pub struct RuntimeBuilder<'a, KV, R> {
    config: Option<Config>,
    vicinity: Option<CoreVicinity>,
    recoder: Option<R>,
    store: Option<&'a Store<KV>>,
}

impl<'a, KV, R> RuntimeBuilder<'a, KV, R> {
    pub const fn frontier() -> Self {
        let config = Config::frontier();

        Self {
            config: Some(config),
            vicinity: None,
            recoder: None,
            store: None,
        }
    }

    pub const fn istanbul() -> Self {
        let config = Config::istanbul();

        Self {
            config: Some(config),
            vicinity: None,
            recoder: None,
            store: None,
        }
    }

    pub const fn berlin() -> Self {
        let config = Config::berlin();

        Self {
            config: Some(config),
            vicinity: None,
            recoder: None,
            store: None,
        }
    }

    pub const fn london() -> Self {
        let config = Config::london();

        Self {
            config: Some(config),
            vicinity: None,
            recoder: None,
            store: None,
        }
    }

    pub fn recoder(&mut self, recoder: R) -> &mut Self {
        self.recoder = Some(recoder);
        self
    }

    pub fn store(&mut self, store: R) -> &mut Self {
        self.recoder = Some(store);
        self
    }

    fn get_init_vicinity(&mut self) -> &mut CoreVicinity {
        if self.vicinity.is_none() {
            self.vicinity = Some(Default::default())
        }

        self.vicinity.as_mut().unwrap()
    }

    pub fn chain_id(&mut self, chain_id: U256) -> &mut Self {
        self.get_init_vicinity().chain_id = chain_id;
        self
    }

    pub fn build(self) -> Result<Runtime<'a, KV, R>> {
        let config = self.config.ok_or(Error::MissingFieldToBuildRuntime)?;
        let vicinity = self.vicinity.ok_or(Error::MissingFieldToBuildRuntime)?;
        let store = self.store.ok_or(Error::MissingFieldToBuildRuntime)?;

        Ok(Runtime {
            config,
            vicinity,
            recoder: self.recoder,
            store,
            state: None,
        })
    }
}

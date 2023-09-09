//! Configure types

use std::{
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};

/// Http Configure
pub struct Http {
    pub listen: SocketAddr,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            listen: SocketAddr::new(IpAddr::V4([127, 0, 0, 1].into()), 8545),
        }
    }
}

/// Websocket Configure
pub struct Ws {
    pub listen: SocketAddr,
}

impl Default for Ws {
    fn default() -> Self {
        Self {
            listen: SocketAddr::new(IpAddr::V4([127, 0, 0, 1].into()), 8546),
        }
    }
}

/// GcMode Configure
#[derive(Debug, Default)]
pub enum GcMode {
    #[default]
    Full,
    Archive,
}

impl GcMode {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Archive => "archive",
        }
    }
}

/// Configure for OpGeth
pub struct Config {
    pub datadir: PathBuf,
    pub http: Option<Http>,
    pub ws: Option<Ws>,
    pub chainid: u64,
    pub gcmode: GcMode,
}

impl Config {
    pub fn new(chainid: u64, datadir: impl AsRef<Path>) -> Self {
        Self {
            datadir: datadir.as_ref().to_path_buf(),
            http: Some(Http::default()),
            ws: None,
            chainid,
            gcmode: GcMode::Archive,
        }
    }
}

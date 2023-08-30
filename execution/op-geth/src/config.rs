use std::path::PathBuf;

pub struct GraphQL {
    pub corsdomain: Option<String>,
    pub vhosts: Vec<String>,
}

pub enum ApiNamespace {}

pub struct Http {
    pub addr: Option<String>,
    pub api: Vec<ApiNamespace>,
    pub corsdomain: Vec<String>,
    pub port: u64,
    pub rpc_prefix: Option<String>,
    pub vhosts: Vec<String>,
}

pub struct Ipc {
    pub disable: bool,
    pub path: PathBuf,
}

pub struct Rpc {
    pub allow_unprotected_txs: bool,
    pub evm_timeout: u64,
    pub gascap: u64,
    pub txfeecap: u64,
}

pub struct Ws {
    pub addr: Option<String>,
    pub api: Vec<ApiNamespace>,
    pub origins: Vec<String>,
    pub port: u64,
    pub rpc_prefix: Option<String>,
}

pub struct Dev {
    pub gaslimit: u128,
    pub period: u64,
}

pub struct Datadir {
    pub ancient: Option<PathBuf>,
    pub minfreedisk: u64,
}

pub enum GCMode {
    Full,
    Archive,
}

pub enum Network {
    Goerli,
    Mainnet,
    Rinkeby,
    Sepolia,
}

pub enum SyncMode {
    Snap,
    Full,
    Light,
}

pub struct Config {
    pub graphql: Option<GraphQL>,
    pub http: Option<Http>,
    pub ipc: Option<Ipc>,
    pub rpc: Option<Rpc>,
    pub ws: Option<Ws>,
    pub dev: Option<Dev>,
    pub datadir: Option<Datadir>,
    pub gcmode: GCMode,
    pub network: Option<Network>,
    pub networkid: u64,
    pub snapshot: bool,
    pub syncmode: SyncMode,
}

use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use fluct_core::{ExecutionService, Service};
use rand::{rngs::OsRng, RngCore};
use rust_embed::RustEmbed;
use subprocess::{Popen, PopenConfig, Redirection};
use tempfile::tempdir;

use crate::{Config, Error, Genesis, GethEngineAPI, GethWeb3Api, Result};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/bin/"]
#[include = "geth"]
struct Assets;

pub struct Geth {
    work_dir: PathBuf,

    cleanup: bool,

    jwt: [u8; 32],

    config: Config,

    handle: Option<Popen>,
}

impl Drop for Geth {
    fn drop(&mut self) {
        if self.cleanup {
            if let Err(e) = self.cleanup() {
                log::error!("Failed to cleanup, please do it manually. {:?}", e);
            }
        }
    }
}

impl Geth {
    pub fn get_bin_dir(&self) -> PathBuf {
        self.work_dir.join("geth")
    }

    pub fn get_genesis_dir(&self) -> PathBuf {
        self.work_dir.join("genesis.json")
    }

    pub fn get_jwt_key_dir(&self) -> PathBuf {
        self.work_dir.join("jwt_key")
    }

    pub fn new(config: Config) -> Result<Self> {
        let work_dir = tempdir()?.into_path();

        let bin_path = work_dir.join("geth");

        let bin = Assets::get("geth").ok_or(Error::NoGethBinaryFound)?;

        fs::write(&bin_path, bin.data)?;

        #[cfg(unix)]
        {
            use std::{
                fs::{metadata, set_permissions},
                os::unix::fs::PermissionsExt,
            };

            let mut permission = metadata(&bin_path)?.permissions();
            permission.set_mode(0o755);
            set_permissions(&bin_path, permission)?;
        }

        let mut jwt = [0u8; 32];
        OsRng.fill_bytes(&mut jwt);

        let jwt_path = work_dir.join("jwt_key");
        fs::write(jwt_path, hex::encode(jwt))?;

        Ok(Self {
            work_dir,
            cleanup: true,
            config,
            jwt,
            handle: None,
        })
    }

    pub fn no_cleanup(&mut self) -> &mut Self {
        self.cleanup = false;
        self
    }

    pub fn cleanup(&mut self) -> Result<()> {
        fs::remove_dir_all(&self.work_dir)?;

        Ok(())
    }

    pub fn _init(&self, genesis: &Genesis) -> Result<()> {
        let config = PopenConfig {
            stderr: Redirection::Pipe,
            ..Default::default()
        };

        let genesis_path = self.get_genesis_dir();
        let file = File::create(&genesis_path)?;
        serde_json::to_writer(file, &genesis)?;

        let mut r = Popen::create(
            &[
                self.get_bin_dir().as_os_str(),
                "init".as_ref(),
                "--datadir".as_ref(),
                self.config.datadir.as_os_str(),
                genesis_path.as_ref(),
            ],
            config,
        )?;

        if r.wait()?.success() {
            Ok(())
        } else {
            let mut buf = String::new();

            if let Some(f) = &mut r.stderr {
                f.read_to_string(&mut buf)?;
            }

            Err(Error::SubprocessExecuteError(buf))
        }
    }

    fn _start(&mut self) -> Result<()> {
        let bin = format!("{}", self.get_bin_dir().display());

        let datadir = format!("{}", self.config.datadir.display());

        let chainid = format!("--networkid={}", self.config.chainid);

        let jwt_key_dir = format!("--authrpc.jwtsecret={}", self.get_jwt_key_dir().display());

        let mut args = vec![
            bin,
            "--datadir".to_string(),
            datadir,
            chainid,
            "--nodiscover".to_string(),
            "--syncmode=full".to_string(),
            "--maxpeers=0".to_string(),
            "--authrpc.vhosts=\"*\"".to_string(),
            "--authrpc.addr=127.0.0.1".to_string(),
            "--authrpc.port=8551".to_string(),
            jwt_key_dir.to_string(),
            "--rollup.disabletxpoolgossip=true".to_string(),
            format!("--gcmode={}", self.config.gcmode.to_str()),
        ];

        if let Some(http) = &self.config.http {
            args.push("--http".to_string());
            args.push("--http.corsdomain=\"*\"".to_string());
            args.push("--http.vhosts=\"*\"".to_string());
            args.push("--http.api=web3,debug,eth,txpool,net,engine".to_string());
            args.push(format!("--http.addr={}", http.listen.ip()));
            args.push(format!("--http.port={}", http.listen.port()));
        }

        if let Some(ws) = &self.config.ws {
            args.push("--ws".to_string());
            args.push("--ws.origins=\"*\"".to_string());
            args.push("--ws.api=debug,eth,txpool,net,engine".to_string());
            args.push(format!("--ws.addr={}", ws.listen.ip()));
            args.push(format!("--ws.port={}", ws.listen.port()));
        }

        log::debug!("Start Arguments: {:#?}", args);

        let handle = Popen::create(&args, PopenConfig::default())?;

        self.handle = Some(handle);

        Ok(())
    }

    fn _stop(&mut self) -> Result<()> {
        let handle = self.handle.as_mut().ok_or(Error::NoInstanceStart)?;

        handle.terminate()?;

        Ok(())
    }

    fn _kill(&mut self) -> Result<()> {
        let handle = self.handle.as_mut().ok_or(Error::NoInstanceStart)?;

        handle.kill()?;

        Ok(())
    }
}

impl Service for Geth {
    type Error = Error;

    fn start(&mut self) -> std::result::Result<(), Self::Error> {
        self._start()
    }

    fn stop(&mut self) -> std::result::Result<(), Self::Error> {
        self._stop()
    }
}

impl ExecutionService for Geth {
    type EngineApi = GethEngineAPI;

    type Web3Api = GethWeb3Api;

    type Genesis = Genesis;

    fn engine_api(&self) -> Result<Self::EngineApi> {
        Self::EngineApi::new(&self.jwt)
    }

    fn init(&mut self, genesis: Genesis) -> Result<()> {
        self._init(&genesis)
    }

    fn reset(&mut self) -> Result<()> {
        fs::remove_dir_all(&self.config.datadir)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, thread, time::Duration};

    use fluct_core::{ExecutionService, Service};

    use crate::{Config, Geth};

    #[test]
    fn test_start() {
        let _ = env_logger::builder().is_test(true).try_init();

        let config = Config::new(99999, "../../target/node0");
        let datadir = config.datadir.clone();

        let genesis = include_str!("../genesis.json");
        let g = serde_json::from_str(genesis).unwrap();

        let mut geth = Geth::new(config).unwrap();
        geth.init(g).unwrap();

        geth.start().unwrap();

        thread::sleep(Duration::new(5, 0));

        geth.stop().unwrap();

        fs::remove_dir_all(datadir).unwrap();
    }
}

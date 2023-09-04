use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use fluct_core::{ExecutionService, Service};
use rust_embed::RustEmbed;
use subprocess::{Popen, PopenConfig, Redirection};
use tempfile::tempdir;

use crate::{Error, OpGethEngine, Result};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/bin/"]
#[include = "geth"]
struct Assets;

pub struct Geth {
    work_dir: PathBuf,

    cleanup: bool,

    datadir: PathBuf,
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

    pub fn new(datadir: impl AsRef<Path>) -> Result<Self> {
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

        Ok(Self {
            work_dir,
            cleanup: true,
            datadir: datadir.as_ref().to_path_buf(),
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

    pub fn init(&self, genesis_path: impl AsRef<Path>) -> Result<()> {
        let config = PopenConfig {
            stderr: Redirection::Pipe,
            ..Default::default()
        };

        let mut r = Popen::create(
            &[
                self.get_bin_dir().as_os_str(),
                "init".as_ref(),
                "--datadir".as_ref(),
                self.datadir.as_os_str(),
                genesis_path.as_ref().as_ref(),
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
}

impl Service for Geth {
    type Error = Error;

    fn start(&mut self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    fn stop(&mut self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    fn kill(&mut self) -> std::result::Result<(), Self::Error> {
        Ok(())
    }
}

/* impl ExecutionService for Geth {
    type API = OpGethEngine;

    type Genesis = ();

    fn api(&self) -> Self::API {}

    fn init(&mut self, genesis: &Self::Genesis) -> std::result::Result<(), Self::Error> {}
} */

#[cfg(test)]
mod tests {
    use crate::Geth;

    #[test]
    fn test_init() {
        let geth = Geth::new("../../target/node").unwrap();

        geth.init("genesis.json").unwrap();
    }
}

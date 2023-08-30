use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use rust_embed::RustEmbed;
use subprocess::{Popen, PopenConfig, Redirection};
use tempfile::tempdir;

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/bin/"]
#[include = "geth"]
struct Assets;

pub struct Geth {
    work_dir: PathBuf,

    cleanup: bool,
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

    pub fn new() -> Result<Self> {
        let work_dir = tempdir()?.into_path();

        let bin_path = work_dir.join("geth");

        let bin = Assets::get("geth").ok_or(anyhow!("No geth found"))?;

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

    pub fn init(&self, datadir: impl AsRef<Path>, genesis_path: impl AsRef<Path>) -> Result<()> {
        let config = PopenConfig {
            stderr: Redirection::Pipe,
            ..Default::default()
        };

        let mut r = Popen::create(
            &[
                self.get_bin_dir().as_os_str(),
                "init".as_ref(),
                "--datadir".as_ref(),
                datadir.as_ref().as_os_str(),
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

            Err(anyhow!("Failed: {buf}"))
        }
    }

    pub fn start(&self) -> Result<()> {
        Ok(())
    }

    pub fn engine(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Geth;

    #[test]
    fn test_init() {
        let geth = Geth::new().unwrap();

        geth.init("../target/node", "genesis.json").unwrap();
    }
}

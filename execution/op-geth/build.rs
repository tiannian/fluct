use std::{
    env,
    fs::{self, OpenOptions},
    path::Path,
};

pub const UPSTREAM_URL: &str =
    "https://media.githubusercontent.com/media/tiannian/go-prebuild-binary/main/op-geth/geth-v1.101200.0";

fn main() {
    #[cfg(unix)]
    use std::os::unix::fs::OpenOptionsExt;

    let out_dir = env::var("OUT_DIR").unwrap();

    let target = Path::new(&out_dir).join("bin");

    fs::create_dir_all(&target).unwrap();

    let target = target.join("geth");

    if !target.exists() {
        let response = attohttpc::get(UPSTREAM_URL).send().unwrap();

        let mut open = OpenOptions::new();
        open.write(true).create(true);

        #[cfg(unix)]
        open.mode(0o776);

        let file = open.open(target).unwrap();

        response.write_to(file).unwrap();
    }
}

use std::{
    env, io,
    io::Read,
    path::{Path, PathBuf},
};

use anyhow::Context;
use flate2::read::GzDecoder;
use tar::Archive;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OS {
    MacOS,
    Linux,
}

impl OS {
    fn get() -> Self {
        let os = env::var("CARGO_CFG_TARGET_OS").expect("Unable to get TARGET_OS");
        match os.as_str() {
            "linux" => Self::Linux,
            "macos" => Self::MacOS,
            os => panic!("Unsupported system {os}"),
        }
    }
}

fn env_var_rerun(name: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={name}");
    env::var(name).ok()
}

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing out dir"));
    println!("{:?}", out_dir);

    let os = OS::get();
    println!("{:?}", os);

    let xla_dir = env_var_rerun("XLA_EXTENSION_DIR")
        .map_or_else(|| out_dir.join("xla_extension"), PathBuf::from);

    println!("{:?}", xla_dir);

    if !xla_dir.exists() {
        download_xla(&out_dir)?;
    }

    Ok(())
}

fn download_xla(xla_dir: &Path) -> anyhow::Result<()> {
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Unable to retrieve TARGET_OS");
    let arch = env::var("CARGO_CFG_TARGET_ARCH").expect("Unable to retrieve TARGET_ARCH");

    let url = match (os.as_str(), arch.as_str()) {
        ("macos", arch) => format!(
            "https://github.com/elodin-sys/xla/releases/download/v0.5.4/xla_extension-{}-darwin-cpu.tar.gz",
            arch
        ),
        ("linux", arch) => format!(
            "https://github.com/elodin-sys/xla/releases/download/v0.5.4/xla_extension-{}-linux-gnu-cpu.tar.gz",
            arch
        ),
        (os, arch) => panic!("{}-{} is an unsupported platform", os, arch),
    };

    let buf = download_file(&url)?;
    let mut bytes = io::Cursor::new(buf);

    let tar = GzDecoder::new(&mut bytes);
    let mut archive = Archive::new(tar);
    archive.unpack(xla_dir)?;

    Ok(())
}

fn download_file(url: &str) -> anyhow::Result<Vec<u8>> {
    let res = ureq::get(url).call()?;
    let content_length = res
        .header("Content-Length")
        .context("Content-Length header not found")?
        .parse::<usize>()?;
    let mut buf = Vec::with_capacity(content_length);
    res.into_reader()
        .take(content_length as u64)
        .read_to_end(&mut buf)
        .context("Failed to read response")?;
    Ok(buf)
}

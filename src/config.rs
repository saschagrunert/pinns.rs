//! Configuration related structures
use anyhow::{bail, Result};
use clap::{AppSettings, Clap};
use getset::{CopyGetters, Getters};
use lazy_static::lazy_static;
use log::{debug, LevelFilter};
use std::{env::temp_dir, fs::metadata, path::PathBuf};

lazy_static! {
    static ref TEMP_DIR: String = temp_dir().display().to_string();
}

#[derive(Clap, Getters, CopyGetters)]
#[clap(
    after_help("More info at: https://github.com/saschagrunert/pinns.rs"),
    global_setting(AppSettings::ColoredHelp),
    version(env!("VERSION"))
)]
/// A simple utility to pin Linux namespaces
pub struct Config {
    #[get_copy = "pub"]
    #[clap(
        default_value("info"),
        long("log-level"),
        possible_values(&["trace", "debug", "info", "warn", "error", "off"]),
        short("l"),
        value_name("LEVEL")
    )]
    /// The logging level of the application
    log_level: LevelFilter,

    #[get = "pub"]
    #[clap(default_value(&TEMP_DIR), long("dir"), short("d"), value_name("DIRECTORY"))]
    /// The directory for the pinned namespaces
    dir: PathBuf,

    #[get_copy = "pub"]
    #[clap(long("cgroup"), short("c"))]
    /// Pin the cgroup namespace
    cgroup: bool,

    #[get_copy = "pub"]
    #[clap(long("ipc"), short("i"))]
    /// Pin the IPC namespace
    ipc: bool,

    #[get_copy = "pub"]
    #[clap(long("net"), short("n"))]
    /// Pin the network namespace
    net: bool,

    #[get_copy = "pub"]
    #[clap(long("pid"), short("p"))]
    /// Pin the PID namespace
    pid: bool,

    #[get_copy = "pub"]
    #[clap(long("user"), short("U"))]
    /// Pin the user namespace
    user: bool,

    #[get_copy = "pub"]
    #[clap(long("uts"), short("u"))]
    /// Pin the UTS namespace
    uts: bool,
}

impl Config {
    /// Validate the configuration in their parameters
    pub fn validate(&self) -> Result<()> {
        if !self.cgroup()
            && !self.ipc()
            && !self.net()
            && !self.pid()
            && !self.user()
            && !self.uts()
        {
            bail!("no namespace specified for pinning")
        }

        if !self.dir().exists() {
            bail!("pin path {} does not exist", self.dir().display())
        }

        if !metadata(self.dir())?.is_dir() {
            bail!("pin path {} is not a directory", self.dir().display())
        }

        debug!("CLI provided config is valid");
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn validate_success() -> Result<()> {
        let mut c = Config::default();
        c.cgroup = true;
        c.validate()
    }

    #[test]
    fn validate_failed_no_namespaces() {
        let c = Config::default();
        assert!(c.validate().is_err())
    }

    #[test]
    fn validate_failed_not_existing_path() {
        let mut c = Config::default();
        c.dir = PathBuf::from("/not/existing/path");
        assert!(c.validate().is_err())
    }

    #[test]
    fn validate_failed_path_not_dir() -> Result<()> {
        let mut c = Config::default();
        c.dir = NamedTempFile::new()?.path().into();
        assert!(c.validate().is_err());
        Ok(())
    }
}

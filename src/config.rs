//! Configuration related structures
use anyhow::{bail, Result};
use clap::{AppSettings, Clap};
use getset::{CopyGetters, Getters};
use log::{debug, LevelFilter};
use std::{fs::metadata, path::PathBuf};

#[derive(Clap, Getters, CopyGetters)]
#[clap(
    after_help("More info at: https://github.com/saschagrunert/pinns"),
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
    #[clap(long("dir"), short("d"), value_name("DIRECTORY"))]
    /// The directory for the pinned namespaces
    dir: PathBuf,

    #[get_copy = "pub"]
    #[clap(long("ipc"), short("i"))]
    /// Pin the IPC namespace
    ipc: bool,

    #[get_copy = "pub"]
    #[clap(long("net"), short("n"))]
    /// Pin the network namespace
    net: bool,

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
        if !self.ipc() && !self.net() && !self.user() && !self.uts() {
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn validate_success() -> Result<()> {
        let c = Config {
            dir: PathBuf::from("/"),
            log_level: LevelFilter::Off,
            ipc: true,
            net: false,
            user: false,
            uts: false,
        };
        c.validate()
    }

    #[test]
    fn validate_failed_no_namespaces() {
        let c = Config {
            dir: PathBuf::from("/"),
            log_level: LevelFilter::Off,
            ipc: false,
            net: false,
            user: false,
            uts: false,
        };
        assert!(c.validate().is_err())
    }

    #[test]
    fn validate_failed_not_existing_path() {
        let c = Config {
            dir: PathBuf::from("/not/existing/path"),
            log_level: LevelFilter::Off,
            ipc: true,
            net: true,
            user: true,
            uts: true,
        };
        assert!(c.validate().is_err())
    }

    #[test]
    fn validate_failed_path_not_dir() -> Result<()> {
        let file = NamedTempFile::new()?;
        let c = Config {
            dir: file.path().into(),
            log_level: LevelFilter::Off,
            ipc: true,
            net: true,
            user: true,
            uts: true,
        };
        assert!(c.validate().is_err());
        Ok(())
    }
}

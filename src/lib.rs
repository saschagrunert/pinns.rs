#![deny(missing_docs)]
//! # pinns
//!
//! A simple utility to pin Linux namespaces

mod config;

use config::{Config, Namespace};

use anyhow::{Context, Result};
use env_logger::try_init;
use log::debug;
use nix::{
    fcntl::{open, OFlag},
    mount::{mount, MsFlags},
    sched::{unshare, CloneFlags},
    sys::stat::Mode,
    unistd::close,
};
use std::{env::set_var, path::PathBuf};

/// The main entry point for pinns
#[derive(Default)]
pub struct Pinns {
    config: Config,
}

impl Pinns {
    /// Run pinns with the provided CLI configuration
    pub fn run(&mut self) -> Result<()> {
        self.init_logging()?;
        self.config.validate()?;
        self.unshare()?;
        self.bind_namespaces()
    }

    // Setup logging via env logger
    fn init_logging(&self) -> Result<()> {
        set_var("RUST_LOG", format!("pinns={}", self.config.log_level()));
        try_init().context("unable to init logger")
    }

    /// Unshare the configured namespaces
    fn unshare(&self) -> Result<()> {
        let flags = self
            .config
            .namespaces()
            .into_iter()
            .filter(|x| x.enabled())
            .fold(CloneFlags::empty(), |mut flags, ns| {
                flags |= ns.clone_flag();
                debug!("unsharing {} namespace", ns.name());
                flags
            });

        unshare(flags).context("failed to unshare namespaces")
    }

    /// Binds the namespaces if provided by the configuration
    fn bind_namespaces(&self) -> Result<()> {
        for ns in self.config.namespaces().into_iter().filter(|x| x.enabled()) {
            self.bind_namespace(ns)?;
        }
        Ok(())
    }

    /// Bind a single namespace
    fn bind_namespace(&self, namespace: Namespace) -> Result<()> {
        let bind_path = self.config.parent_dir_for_namespace(namespace.name())
                            .join(self.config.filename());
        debug!("binding namespace: {}", bind_path.display());

        let fd = open(
            &bind_path,
            OFlag::O_RDONLY | OFlag::O_CREAT | OFlag::O_EXCL,
            Mode::empty(),
        )
        .context(format!(
            "unable to create namespace file {}",
            bind_path.display()
        ))?;
        close(fd).context("unable to close file descriptor")?;

        let ns_path = PathBuf::from("/proc/self/ns").join(namespace.name());
        debug!("mounting {}", ns_path.display());
        mount::<_, _, PathBuf, PathBuf>(Some(&ns_path), &bind_path, None, MsFlags::MS_BIND, None)
            .context(format!(
            "unable to bind mount namespace {}",
            ns_path.display()
        ))?;

        Ok(())
    }
}

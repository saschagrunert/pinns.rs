#![deny(missing_docs)]
//! # pinns
//!
//! A simple utility to pin Linux namespaces

mod config;

use config::Config;

use anyhow::{Context, Result};
use clap::Clap;
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
pub struct Pinns {
    config: Config,
}

impl Pinns {
    /// Create a new pinns instance
    pub fn new() -> Self {
        Pinns {
            config: Config::parse(),
        }
    }

    /// Run pinns with the provided CLI configuration
    pub fn run(&self) -> Result<()> {
        // Setup logging
        set_var("RUST_LOG", format!("pinns={}", self.config.log_level()));
        try_init().context("unable to init logger")?;

        self.config.validate()?;
        self.unshare()?;
        self.bind_namespaces()
    }

    /// Unshare the configured namespaces
    fn unshare(&self) -> Result<()> {
        let mut flags = CloneFlags::empty();

        if self.config.uts() {
            flags |= CloneFlags::CLONE_NEWUTS;
            debug!("unsharing UTS namespace")
        }
        if self.config.ipc() {
            flags |= CloneFlags::CLONE_NEWIPC;
            debug!("unsharing IPC namespace")
        }
        if self.config.net() {
            flags |= CloneFlags::CLONE_NEWNET;
            debug!("unsharing NET namespace")
        }
        if self.config.user() {
            flags |= CloneFlags::CLONE_NEWUSER;
            debug!("unsharing USER namespace")
        }

        unshare(flags).context("failed to unshare namespaces")
    }

    /// Binds the namespaces if provided by the configuration
    fn bind_namespaces(&self) -> Result<()> {
        if self.config.uts() {
            self.bind_namespace("uts")?;
        }
        if self.config.ipc() {
            self.bind_namespace("ipc")?;
        }
        if self.config.net() {
            self.bind_namespace("net")?;
        }
        if self.config.user() {
            self.bind_namespace("user")?;
        }
        Ok(())
    }

    /// Bind a single namespace
    fn bind_namespace(&self, name: &str) -> Result<()> {
        let bind_path = self.config.dir().join(name);
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

        let ns_path = PathBuf::from("/proc/self/ns").join(name);
        debug!("mounting {}", ns_path.display());
        mount::<_, _, PathBuf, PathBuf>(Some(&ns_path), &bind_path, None, MsFlags::MS_BIND, None)
            .context(format!(
            "unable to bind mount namespace {}",
            ns_path.display()
        ))?;

        Ok(())
    }
}

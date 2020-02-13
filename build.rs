use anyhow::Result;
use std::process::Command;

pub fn main() -> Result<()> {
    // Retrieve the version from git
    let version = Command::new("git").arg("describe").arg("--tags").output()?;
    println!(
        "cargo:rustc-env=VERSION={}",
        String::from_utf8(version.stdout)?
    );

    Ok(())
}

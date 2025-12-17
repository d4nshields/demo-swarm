//! Version information output.

use anyhow::Result;
use clap::Args;
use serde::Serialize;

/// Version information for the demoswarm CLI.
#[derive(Serialize)]
struct VersionInfo {
    name: String,
    version: String,
}

#[derive(Args, Debug)]
pub struct VersionCommand;

pub fn run(_cmd: VersionCommand) -> Result<()> {
    let info = VersionInfo {
        name: "demoswarm".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    let json = serde_json::to_string_pretty(&info)?;
    println!("{json}");
    Ok(())
}

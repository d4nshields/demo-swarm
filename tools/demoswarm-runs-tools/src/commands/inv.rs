//! Inventory marker extraction.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;

use super::common::CompatNullIfMissing;
use crate::output::{print_null, print_scalar};

#[derive(Args, Debug)]
pub struct InvCommand {
    #[command(subcommand)]
    pub command: InvSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum InvSubcommand {
    /// Extract value from an inventory marker line
    Get {
        /// File to search
        #[arg(long)]
        file: String,

        /// Marker prefix (e.g., DEP_CI_SIGNAL)
        #[arg(long)]
        marker: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: InvCommand) -> Result<()> {
    match cmd.command {
        InvSubcommand::Get { file, marker, .. } => extract_inventory_marker(&file, &marker),
    }
}

fn extract_inventory_marker(file: &str, marker: &str) -> Result<()> {
    let path = Path::new(file);
    if !path.is_file() {
        print_null();
        return Ok(());
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    // Look for pattern: ^- <MARKER>: <value>
    let pattern = format!(r"^-\s*{}\s*:\s*(.+?)\s*$", regex::escape(marker));
    let regex = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    for line in content.lines() {
        if let Some(caps) = regex.captures(line)
            && let Some(value) = caps.get(1)
        {
            print_scalar(value.as_str().trim());
            return Ok(());
        }
    }

    print_null();
    Ok(())
}

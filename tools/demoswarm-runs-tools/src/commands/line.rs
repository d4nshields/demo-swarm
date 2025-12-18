//! Line value extraction.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};

use super::common::CompatNullIfMissing;
use crate::output::{print_null, print_scalar};

#[derive(Args, Debug)]
pub struct LineCommand {
    #[command(subcommand)]
    pub command: LineSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum LineSubcommand {
    /// Extract value from a line with a known prefix
    Get {
        /// File to search
        #[arg(long)]
        file: String,

        /// Line prefix (e.g., "Mutation Score:")
        #[arg(long)]
        prefix: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: LineCommand) -> Result<()> {
    match cmd.command {
        LineSubcommand::Get { file, prefix, .. } => extract_line_value(&file, &prefix),
    }
}

fn extract_line_value(file: &str, prefix: &str) -> Result<()> {
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

    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix(prefix) {
            let value = stripped.trim();
            if value.is_empty() {
                print_null();
            } else {
                print_scalar(value);
            }
            return Ok(());
        }
    }

    print_null();
    Ok(())
}

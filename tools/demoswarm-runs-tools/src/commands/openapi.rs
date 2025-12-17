//! OpenAPI operations.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;

use super::common::CompatNullIfMissing;
use crate::output::{print_count, print_null};

#[derive(Args, Debug)]
pub struct OpenapiCommand {
    #[command(subcommand)]
    pub command: OpenapiSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum OpenapiSubcommand {
    /// Count API paths in an OpenAPI YAML file
    CountPaths {
        /// OpenAPI YAML file
        #[arg(long)]
        file: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: OpenapiCommand) -> Result<()> {
    match cmd.command {
        OpenapiSubcommand::CountPaths { file, .. } => count_openapi_paths(&file),
    }
}

fn count_openapi_paths(file: &str) -> Result<()> {
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

    let lines: Vec<&str> = content.lines().collect();

    // Find "paths:" anchor
    let mut paths_indent: Option<usize> = None;
    let mut start_idx = 0;

    let paths_regex = match Regex::new(r"^(\s*)paths:\s*$") {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    for (i, line) in lines.iter().enumerate() {
        if let Some(captures) = paths_regex.captures(line)
            && let Some(m) = captures.get(1)
        {
            paths_indent = Some(m.as_str().len());
            start_idx = i + 1;
            break;
        }
    }

    let paths_indent = match paths_indent {
        Some(i) => i,
        None => {
            print_null();
            return Ok(());
        }
    };

    let path_regex = match Regex::new(r#"^\s+['"]?/"#) {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    let mut count = 0;
    for line in &lines[start_idx..] {
        // Skip empty lines and comments
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Count leading spaces
        let indent = line.len() - line.trim_start().len();

        // End of paths block: back to same or less indentation
        if indent <= paths_indent && !line.trim().is_empty() && !line.starts_with(' ') {
            break;
        }

        // Count path entries
        if path_regex.is_match(line) {
            count += 1;
        }
    }

    print_count(count);
    Ok(())
}

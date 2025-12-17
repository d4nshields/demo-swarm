//! YAML block operations.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;

use super::common::CompatNullIfMissing;
use crate::output::{print_count, print_null, print_scalar};

#[derive(Args, Debug)]
pub struct YamlCommand {
    #[command(subcommand)]
    pub command: YamlSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum YamlSubcommand {
    /// Extract a field from fenced YAML block
    Get {
        /// Markdown file with YAML block
        #[arg(long)]
        file: String,

        /// YAML key to extract
        #[arg(long)]
        key: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },

    /// Count items matching pattern in YAML block
    CountItems {
        /// Markdown file with YAML block
        #[arg(long)]
        file: String,

        /// Pattern to count within YAML
        #[arg(long)]
        item_regex: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: YamlCommand) -> Result<()> {
    match cmd.command {
        YamlSubcommand::Get { file, key, .. } => extract_yaml_field(&file, &key),
        YamlSubcommand::CountItems {
            file, item_regex, ..
        } => count_yaml_items(&file, &item_regex),
    }
}

fn extract_yaml_field(file: &str, key: &str) -> Result<()> {
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

    let yaml_block = match extract_yaml_block(&content) {
        Some(y) => y,
        None => {
            print_null();
            return Ok(());
        }
    };

    // Simple YAML key extraction
    let key_pattern = format!(r"^\s*{}\s*:\s*(.+?)\s*$", regex::escape(key));
    let regex = match Regex::new(&key_pattern) {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    for line in yaml_block.lines() {
        if let Some(caps) = regex.captures(line)
            && let Some(value) = caps.get(1)
        {
            let mut val = value.as_str().trim();

            // Remove quotes if present
            if (val.starts_with('"') && val.ends_with('"'))
                || (val.starts_with('\'') && val.ends_with('\''))
            {
                val = &val[1..val.len() - 1];
            }

            print_scalar(val);
            return Ok(());
        }
    }

    print_null();
    Ok(())
}

fn count_yaml_items(file: &str, pattern: &str) -> Result<()> {
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

    let yaml_block = match extract_yaml_block(&content) {
        Some(y) => y,
        None => {
            print_null();
            return Ok(());
        }
    };

    // Convert POSIX character class to Rust regex
    let rust_pattern = pattern.replace("[[:space:]]", r"\s");
    let regex = match Regex::new(&rust_pattern) {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    let count = yaml_block
        .lines()
        .filter(|line| regex.is_match(line))
        .count();
    print_count(count);
    Ok(())
}

/// Extract fenced YAML block from content.
fn extract_yaml_block(content: &str) -> Option<String> {
    let mut in_yaml = false;
    let mut yaml_lines = Vec::new();

    for line in content.lines() {
        if !in_yaml {
            if line.trim() == "```yaml" {
                in_yaml = true;
            }
            continue;
        }

        if line.trim() == "```" {
            break;
        }
        yaml_lines.push(line);
    }

    if yaml_lines.is_empty() {
        None
    } else {
        Some(yaml_lines.join("\n"))
    }
}

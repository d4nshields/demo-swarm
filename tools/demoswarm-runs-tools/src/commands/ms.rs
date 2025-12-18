//! Machine Summary extraction.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;

use super::common::CompatNullIfMissing;
use crate::output::{print_null, print_scalar};

#[derive(Args, Debug)]
pub struct MsCommand {
    #[command(subcommand)]
    pub command: MsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum MsSubcommand {
    /// Extract a field from Machine Summary block
    Get {
        /// Markdown file
        #[arg(long)]
        file: String,

        /// Section header (e.g., "## Machine Summary")
        #[arg(long)]
        section: String,

        /// Field name to extract
        #[arg(long)]
        key: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: MsCommand) -> Result<()> {
    match cmd.command {
        MsSubcommand::Get {
            file, section, key, ..
        } => extract_machine_field(&file, &section, &key),
    }
}

fn extract_machine_field(file: &str, section: &str, key: &str) -> Result<()> {
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

    // Find the section
    let section_content = match extract_section(&content, section) {
        Some(s) => s,
        None => {
            print_null();
            return Ok(());
        }
    };

    // Extract the field value
    let key_pattern = format!(r"^\s*{}\s*:\s*(.+?)\s*$", regex::escape(key));
    let regex = match Regex::new(&key_pattern) {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    for line in section_content.lines() {
        if let Some(caps) = regex.captures(line)
            && let Some(value) = caps.get(1)
        {
            let val = value.as_str().trim();

            // Template leak guard
            if val.contains('|') || val.contains('<') {
                print_null();
                return Ok(());
            }

            // Return first word
            let first_word = val.split_whitespace().next().unwrap_or("");
            if first_word.is_empty() {
                print_null();
            } else {
                print_scalar(first_word);
            }
            return Ok(());
        }
    }

    print_null();
    Ok(())
}

/// Extract content from a markdown section (from header to next ## heading).
fn extract_section(content: &str, section_header: &str) -> Option<String> {
    let mut in_section = false;
    let mut section_lines = Vec::new();

    for line in content.lines() {
        if !in_section {
            if line.trim() == section_header.trim() {
                in_section = true;
            }
            continue;
        }

        // End on next ## heading
        if line.starts_with("## ") {
            break;
        }
        section_lines.push(line);
    }

    if section_lines.is_empty() {
        None
    } else {
        Some(section_lines.join("\n"))
    }
}

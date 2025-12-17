//! Count operations: pattern matching and BDD scenarios.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use regex::Regex;

use super::common::CompatNullIfMissing;
use crate::output::{print_count, print_null};

#[derive(Args, Debug)]
pub struct CountCommand {
    #[command(subcommand)]
    pub command: CountSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum CountSubcommand {
    /// Count lines matching a regex pattern
    Pattern {
        /// File to search
        #[arg(long)]
        file: String,

        /// Extended regex pattern
        #[arg(long)]
        regex: String,

        /// Fallback regex if primary returns 0
        #[arg(long)]
        fallback_regex: Option<String>,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat_missing: CompatNullIfMissing,

        /// When set, return null instead of 0 if no matches
        #[arg(long, default_value_t = false)]
        null_if_zero: bool,
    },

    /// Count BDD scenarios in feature files
    Bdd {
        /// Features directory
        #[arg(long)]
        dir: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat_missing: CompatNullIfMissing,
    },
}

pub fn run(cmd: CountCommand) -> Result<()> {
    match cmd.command {
        CountSubcommand::Pattern {
            file,
            regex,
            fallback_regex,
            null_if_zero,
            ..
        } => count_pattern(&file, &regex, fallback_regex.as_deref(), null_if_zero),
        CountSubcommand::Bdd { dir, .. } => count_bdd_scenarios(&dir),
    }
}

fn count_pattern(
    file: &str,
    pattern: &str,
    fallback: Option<&str>,
    null_if_zero: bool,
) -> Result<()> {
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

    let regex = match Regex::new(pattern) {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    let mut count = content.lines().filter(|line| regex.is_match(line)).count();

    // Try fallback if primary returns 0
    if count == 0
        && let Some(fb_pattern) = fallback
        && let Ok(fb_regex) = Regex::new(fb_pattern)
    {
        let fb_count = content
            .lines()
            .filter(|line| fb_regex.is_match(line))
            .count();
        count = fb_count;
    }

    if count == 0 && null_if_zero {
        print_null();
    } else {
        print_count(count);
    }
    Ok(())
}

fn count_bdd_scenarios(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    if !path.is_dir() {
        print_null();
        return Ok(());
    }

    let scenario_regex = match Regex::new(r"^\s*(Scenario:|Scenario Outline:)") {
        Ok(r) => r,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    let mut total = 0;

    // Walk directory for .feature files
    for entry in walkdir(path) {
        if let Some(ext) = entry.extension()
            && ext == "feature"
            && let Ok(content) = fs::read_to_string(&entry)
        {
            total += content
                .lines()
                .filter(|line| scenario_regex.is_match(line))
                .count();
        }
    }

    print_count(total);
    Ok(())
}

/// Simple directory walker for .feature files.
fn walkdir(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                result.push(path);
            } else if path.is_dir() {
                result.extend(walkdir(&path));
            }
        }
    }
    result
}

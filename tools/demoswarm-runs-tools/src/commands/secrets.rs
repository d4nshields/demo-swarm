//! Secret scanning and redaction commands.
//!
//! Security note: This module NEVER prints secret content to stdout.
//! - `scan` writes findings (file/line/type only) to JSON
//! - `redact` modifies files in-place, prints only status

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::builder::PossibleValuesParser;
use clap::{Args, Subcommand};
use regex::Regex;
use serde_json::{Value, json};
use tempfile::NamedTempFile;

use crate::output::print_scalar;

/// Secret detection patterns. Each tuple is (regex, type-name).
const PATTERNS: &[(&str, &str)] = &[
    (r"gh[pousr]_[A-Za-z0-9_]{36,}", "github-token"),
    (r"AKIA[0-9A-Z]{16}", "aws-access-key"),
    (r"sk_live_[0-9a-zA-Z]{24,}", "stripe-key"),
    (r"-----BEGIN\s.*PRIVATE KEY-----", "private-key"),
    (
        r"eyJ[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*",
        "jwt-token",
    ),
];

#[derive(Args, Debug)]
pub struct SecretsCommand {
    #[command(subcommand)]
    pub command: SecretsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum SecretsSubcommand {
    /// Scan a file/directory for secrets (locations only); writes JSON findings
    Scan(SecretsScan),

    /// Redact a specific secret type in a file
    Redact(SecretsRedact),
}

#[derive(Args, Debug)]
pub struct SecretsScan {
    /// Path to scan (file or directory)
    #[arg(long)]
    pub path: String,

    /// Output file for JSON findings
    #[arg(long)]
    pub output: String,
}

#[derive(Args, Debug)]
pub struct SecretsRedact {
    /// File to redact secrets from
    #[arg(long)]
    pub file: String,

    /// Type of secret to redact
    #[arg(long, value_parser = PossibleValuesParser::new(["github-token", "aws-access-key", "stripe-key", "jwt-token", "private-key"]))]
    pub r#type: String,
}

pub fn run(cmd: SecretsCommand) -> Result<()> {
    match cmd.command {
        SecretsSubcommand::Scan(args) => scan(&args),
        SecretsSubcommand::Redact(args) => redact(&args),
    }
}

fn scan(args: &SecretsScan) -> Result<()> {
    let root = Path::new(&args.path);
    let out_path = Path::new(&args.output);

    if !root.exists() {
        let v = json!({ "status": "SCAN_PATH_MISSING", "findings": [] });
        write_json_atomic(out_path, &v).map_err(|e| {
            eprintln!(
                "Warning: failed to write secrets findings JSON ({}): {e:#}",
                out_path.display()
            );
            e
        })?;
        print_scalar("SCAN_PATH_MISSING");
        return Ok(());
    }

    // Precompile regexes
    let compiled: Vec<(Regex, &str)> = PATTERNS
        .iter()
        .filter_map(|(pat, typ)| Regex::new(pat).ok().map(|r| (r, *typ)))
        .collect();

    let mut findings: Vec<Value> = Vec::new();

    if root.is_file() {
        scan_one_file(root, &compiled, &mut findings);
    } else if root.is_dir() {
        for f in iter_files(root) {
            scan_one_file(&f, &compiled, &mut findings);
        }
    }

    let status = if findings.is_empty() {
        "CLEAN"
    } else {
        "SECRETS_FOUND"
    };
    let v = json!({ "status": status, "findings": findings });
    write_json_atomic(out_path, &v).map_err(|e| {
        eprintln!(
            "Warning: failed to write secrets findings JSON ({}): {e:#}",
            out_path.display()
        );
        e
    })?;
    print_scalar(status);
    Ok(())
}

/// Directory names to exclude from scanning (deterministic fixed list).
const EXCLUDED_DIRS: &[&str] = &[".git", "target", "node_modules", ".demoswarm"];

/// Walk a directory recursively (no symlink following; best-effort)
fn iter_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let rd = match fs::read_dir(&dir) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for ent in rd.flatten() {
            let p = ent.path();
            if p.is_dir() {
                // Skip excluded directories
                if let Some(name) = p.file_name().and_then(|n| n.to_str())
                    && EXCLUDED_DIRS.contains(&name)
                {
                    continue;
                }
                stack.push(p);
            } else if p.is_file() {
                out.push(p);
            }
        }
    }

    out
}

fn scan_one_file(path: &Path, patterns: &[(Regex, &str)], findings: &mut Vec<Value>) {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return,
    };

    // Lossy read; we never emit matched content
    let content = String::from_utf8_lossy(&bytes);
    let lines: Vec<&str> = content.lines().collect();

    for (re, typ) in patterns {
        let mut line_nums: Vec<String> = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if re.is_match(line) {
                line_nums.push((i + 1).to_string());
            }
        }
        if !line_nums.is_empty() {
            findings.push(json!({
                "file": path.to_string_lossy(),
                "type": typ,
                "lines": line_nums.join(",")
            }));
        }
    }
}

fn redact(args: &SecretsRedact) -> Result<()> {
    let path = Path::new(&args.file);
    if !path.is_file() {
        print_scalar("FILE_NOT_FOUND");
        return Ok(());
    }

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => {
            print_scalar("null");
            return Ok(());
        }
    };
    let s = String::from_utf8_lossy(&bytes).to_string();

    let redacted = match args.r#type.as_str() {
        "github-token" => redact_regex(
            &s,
            r"gh[pousr]_[A-Za-z0-9_]{36,}",
            "[REDACTED:github-token]",
        ),
        "aws-access-key" => redact_regex(&s, r"AKIA[0-9A-Z]{16}", "[REDACTED:aws-access-key]"),
        "stripe-key" => redact_regex(&s, r"sk_live_[0-9a-zA-Z]{24,}", "[REDACTED:stripe-key]"),
        "jwt-token" => redact_regex(
            &s,
            r"eyJ[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*",
            "[REDACTED:jwt-token]",
        ),
        "private-key" => redact_private_key_blocks(&s),
        _ => s, // Should not happen due to value_parser
    };

    if fs::write(path, redacted).is_err() {
        print_scalar("null");
        return Ok(());
    }
    print_scalar("ok");
    Ok(())
}

fn redact_regex(content: &str, pat: &str, repl: &str) -> String {
    match Regex::new(pat) {
        Ok(re) => re.replace_all(content, repl).to_string(),
        Err(_) => content.to_string(),
    }
}

/// Deterministic, line-based replacement of PEM blocks.
/// No regex backtracking required.
fn redact_private_key_blocks(content: &str) -> String {
    let mut out = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        if !in_block {
            if line.contains("-----BEGIN") && line.contains("PRIVATE KEY-----") {
                in_block = true;
                out.push("[REDACTED:private-key]".to_string());
                continue;
            }
            out.push(line.to_string());
        } else {
            // Skip until end marker
            if line.contains("-----END") && line.contains("PRIVATE KEY-----") {
                in_block = false;
            }
        }
    }

    let mut joined = out.join("\n");
    joined.push('\n');
    joined
}

fn write_json_atomic(path: &Path, v: &Value) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    // Create a temporary file in the same directory as the target
    // This ensures they're on the same filesystem for atomic rename
    let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut tmp = NamedTempFile::new_in(parent_dir)?;

    // Write the JSON content to the temporary file
    tmp.write_all(format!("{}\n", serde_json::to_string_pretty(v)?).as_bytes())?;

    // Persist the temporary file to the target path
    // This is an atomic operation that replaces the target if it exists
    tmp.persist(path)?;

    Ok(())
}

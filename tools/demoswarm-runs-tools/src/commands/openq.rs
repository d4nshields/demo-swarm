//! Open questions tracking commands.

use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use clap::{Args, Subcommand};
use regex::Regex;

use crate::output::print_scalar;

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

#[derive(Args, Debug)]
pub struct OpenqCommand {
    #[command(subcommand)]
    pub command: OpenqSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum OpenqSubcommand {
    /// Generate next open question ID (OQ-<PREFIX>-NNN)
    NextId(OpenqNextId),

    /// Append an open question entry; prints the assigned QID
    Append(OpenqAppend),
}

#[derive(Args, Debug)]
pub struct OpenqNextId {
    /// Path to the open questions file
    #[arg(long)]
    pub file: String,

    /// Prefix for the question ID (e.g., "SIGNAL" -> OQ-SIGNAL-001)
    #[arg(long)]
    pub prefix: String,
}

#[derive(Args, Debug)]
pub struct OpenqAppend {
    /// Path to the open questions file
    #[arg(long)]
    pub file: String,

    /// Prefix for the question ID
    #[arg(long)]
    pub prefix: String,

    /// The question text
    #[arg(long)]
    pub question: String,

    /// Suggested default answer
    #[arg(long)]
    pub default: String,

    /// Impact if the actual answer differs from the default
    #[arg(long)]
    pub impact: String,
}

pub fn run(cmd: OpenqCommand) -> Result<()> {
    match cmd.command {
        OpenqSubcommand::NextId(args) => {
            let qid = compute_next_id(&args.file, &args.prefix);
            print_scalar(&qid);
            Ok(())
        }
        OpenqSubcommand::Append(args) => {
            let qid = compute_next_id(&args.file, &args.prefix);
            match append_openq(&args, &qid) {
                Ok(()) => {
                    print_scalar(&qid);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Warning: openq append failed: {e:#}");
                    print_scalar("null");
                    Err(e)
                }
            }
        }
    }
}

fn compute_next_id(file: &str, prefix: &str) -> String {
    let path = Path::new(file);
    if !path.is_file() {
        return format!("OQ-{prefix}-001");
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return format!("OQ-{prefix}-001"),
    };

    let pattern = format!(r"OQ-{}-(\d{{3}})", regex::escape(prefix));
    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(_) => return format!("OQ-{prefix}-001"),
    };

    let mut max_n: Option<u32> = None;
    for caps in re.captures_iter(&content) {
        if let Some(m) = caps.get(1)
            && let Ok(n) = m.as_str().parse::<u32>()
        {
            max_n = Some(max_n.map_or(n, |x| x.max(n)));
        }
    }

    match max_n {
        None => format!("OQ-{prefix}-001"),
        Some(n) if n >= 999 => format!("OQ-{prefix}-UNK"),
        Some(n) => format!("OQ-{prefix}-{:03}", n + 1),
    }
}

fn append_openq(args: &OpenqAppend, qid: &str) -> Result<()> {
    let path = Path::new(&args.file);

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    if !path.is_file() {
        // Create with header, matching python fallback
        fs::write(path, "# Open Questions\n\n")?;
    }

    let entry = format!(
        "- QID: {qid}\n  - Q: {} [OPEN]\n  - Suggested default: {}\n  - Impact if different: {}\n  - Added: {}\n\n",
        args.question,
        args.default,
        args.impact,
        iso_now()
    );

    let mut f = fs::OpenOptions::new().append(true).open(path)?;
    f.write_all(entry.as_bytes())?;
    Ok(())
}

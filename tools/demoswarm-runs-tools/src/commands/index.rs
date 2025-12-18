//! Index operations.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use serde_json::{Value, json};

use crate::output::{print_null, print_ok, print_scalar};

#[derive(Args, Debug)]
pub struct IndexCommand {
    #[command(subcommand)]
    pub command: IndexSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum IndexSubcommand {
    /// Update run status in index.json
    UpsertStatus {
        /// Path to index.json
        #[arg(long)]
        index: String,

        /// Run ID to update
        #[arg(long)]
        run_id: String,

        /// New status
        #[arg(long)]
        status: String,

        /// Last completed flow
        #[arg(long)]
        last_flow: String,

        /// ISO8601 timestamp (optional, defaults to now)
        #[arg(long)]
        updated_at: Option<String>,
    },
}

pub fn run(cmd: IndexCommand) -> Result<()> {
    match cmd.command {
        IndexSubcommand::UpsertStatus {
            index,
            run_id,
            status,
            last_flow,
            updated_at,
        } => upsert_status(&index, &run_id, &status, &last_flow, updated_at.as_deref()),
    }
}

fn upsert_status(
    index_path: &str,
    run_id: &str,
    status: &str,
    last_flow: &str,
    updated_at: Option<&str>,
) -> Result<()> {
    let path = Path::new(index_path);

    // Index must exist (creation is owned by run-prep)
    if !path.is_file() {
        print_scalar("SKIPPED_MISSING_INDEX");
        return Ok(());
    }

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            print_scalar("SKIPPED_MISSING_INDEX");
            return Ok(());
        }
    };

    let mut index: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    // Get or create runs array
    let runs = match index.get_mut("runs") {
        Some(Value::Array(arr)) => arr,
        _ => {
            print_null();
            return Ok(());
        }
    };

    // Get timestamp
    let ts = updated_at
        .map(String::from)
        .unwrap_or_else(|| chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());

    // Find and update existing entry, or append new one
    let mut found = false;
    for entry in runs.iter_mut() {
        if let Some(id) = entry.get("run_id").and_then(|v| v.as_str())
            && id == run_id
        {
            entry["status"] = json!(status);
            entry["last_flow"] = json!(last_flow);
            entry["updated_at"] = json!(ts);
            found = true;
            break;
        }
    }

    if !found {
        // Append new entry
        runs.push(json!({
            "run_id": run_id,
            "status": status,
            "last_flow": last_flow,
            "updated_at": ts,
        }));
    }

    // Sort runs by run_id for stable diffs
    if let Some(Value::Array(arr)) = index.get_mut("runs") {
        arr.sort_by(|a, b| {
            let a_id = a.get("run_id").and_then(|v| v.as_str()).unwrap_or("");
            let b_id = b.get("run_id").and_then(|v| v.as_str()).unwrap_or("");
            a_id.cmp(b_id)
        });
    }

    // Atomic write
    let tmp_path = format!("{}.tmp", index_path);
    let json_str = serde_json::to_string_pretty(&index)?;
    if fs::write(&tmp_path, format!("{json_str}\n")).is_err() {
        print_null();
        return Ok(());
    }
    if fs::rename(&tmp_path, path).is_err() {
        let _ = fs::remove_file(&tmp_path);
        print_null();
        return Ok(());
    }

    print_ok();
    Ok(())
}

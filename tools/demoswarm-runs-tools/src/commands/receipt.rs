//! Receipt field reading.

use std::fs;
use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};
use serde_json::Value;

use super::common::CompatNullIfMissing;
use crate::output::{print_null, print_scalar};

#[derive(Args, Debug)]
pub struct ReceiptCommand {
    #[command(subcommand)]
    pub command: ReceiptSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ReceiptSubcommand {
    /// Read a field from a receipt JSON file
    Get {
        /// Receipt JSON file
        #[arg(long)]
        file: String,

        /// Top-level key to extract
        #[arg(long)]
        key: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: ReceiptCommand) -> Result<()> {
    match cmd.command {
        ReceiptSubcommand::Get { file, key, .. } => read_receipt_field(&file, &key),
    }
}

fn read_receipt_field(file: &str, key: &str) -> Result<()> {
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

    let json: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => {
            print_null();
            return Ok(());
        }
    };

    let value = match json.get(key) {
        Some(v) => v,
        None => {
            print_null();
            return Ok(());
        }
    };

    // Print scalar values only
    match value {
        Value::String(s) => print_scalar(s),
        Value::Number(n) => print_scalar(n),
        Value::Bool(b) => print_scalar(b),
        Value::Null => print_null(),
        _ => print_null(), // Arrays/objects not supported
    }

    Ok(())
}

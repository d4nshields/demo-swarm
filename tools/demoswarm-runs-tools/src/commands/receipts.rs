//! Receipt counting (existing receipts in run dir).

use std::path::Path;

use anyhow::Result;
use clap::{Args, Subcommand};

use super::common::CompatNullIfMissing;
use crate::output::{print_count, print_null};

#[derive(Args, Debug)]
pub struct ReceiptsCommand {
    #[command(subcommand)]
    pub command: ReceiptsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ReceiptsSubcommand {
    /// Count prior flow receipts in a run directory
    Count {
        /// Run directory path
        #[arg(long)]
        run_dir: String,

        /// Compatibility flag; accepted for interface parity
        #[command(flatten)]
        _compat: CompatNullIfMissing,
    },
}

pub fn run(cmd: ReceiptsCommand) -> Result<()> {
    match cmd.command {
        ReceiptsSubcommand::Count { run_dir, .. } => count_existing_receipts(&run_dir),
    }
}

fn count_existing_receipts(run_dir: &str) -> Result<()> {
    let path = Path::new(run_dir);
    if !path.is_dir() {
        print_null();
        return Ok(());
    }

    // Known receipt paths
    let receipt_files = [
        "signal/signal_receipt.json",
        "plan/plan_receipt.json",
        "build/build_receipt.json",
        "gate/gate_receipt.json",
        "deploy/deploy_receipt.json",
        "wisdom/wisdom_receipt.json",
    ];

    let count = receipt_files
        .iter()
        .filter(|f| path.join(f).is_file())
        .count();

    print_count(count);
    Ok(())
}

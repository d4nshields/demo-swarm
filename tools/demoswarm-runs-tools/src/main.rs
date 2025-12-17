//! demoswarm CLI - deterministic helpers for .runs/ operations.
//!
//! Supports two invocation modes:
//! 1. Subcommand: `demoswarm count pattern --file ... --regex ...`
//! 2. Multicall: when called as `runs_count_pattern`, acts as that command

use std::env;
use std::process::ExitCode;

use clap::Parser;

mod commands;
mod output;

use commands::{Cli, Command};
use output::print_null;

/// Multicall dispatch table: maps argv[0] suffix to subcommand.
const MULTICALL_MAP: &[(&str, &str)] = &[
    ("runs_count_pattern", "count pattern"),
    ("runs_count_bdd_scenarios", "count bdd"),
    ("runs_extract_machine_field", "ms get"),
    ("runs_extract_yaml_block_field", "yaml get"),
    ("runs_count_yaml_block_items", "yaml count-items"),
    ("runs_extract_inventory_marker", "inv get"),
    ("runs_extract_line_value", "line get"),
    ("runs_count_existing_receipts", "receipts count"),
    ("runs_read_receipt_field", "receipt get"),
    ("runs_count_openapi_paths", "openapi count-paths"),
    ("runs_index_upsert_status", "index upsert-status"),
    ("runs_iso_now", "time now"),
    // Open questions
    ("runs_openq_next_id", "openq next-id"),
    ("runs_openq_append", "openq append"),
    // Secrets
    ("runs_scan_secrets_safe", "secrets scan"),
    ("runs_redact_secret_type", "secrets redact"),
];

fn main() -> ExitCode {
    // Check for multicall invocation
    if let Some(args) = try_multicall_dispatch() {
        return run_with_args(&args);
    }

    // Standard clap parsing
    match Cli::try_parse() {
        Ok(cli) => run_cli(cli),
        Err(e) => {
            eprintln!("{e}");
            print_null();
            exit_code_for_mode()
        }
    }
}

/// Check if we're being invoked via multicall (argv[0] matches a helper name).
fn try_multicall_dispatch() -> Option<Vec<String>> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        return None;
    }

    // Extract basename of argv[0]
    let argv0 = &args[0];
    let basename = std::path::Path::new(argv0)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(argv0);

    // Look up in multicall map
    for (name, subcmd) in MULTICALL_MAP {
        if basename == *name {
            // Build synthetic argument list: ["demoswarm", subcmd parts..., original args...]
            let mut new_args = vec!["demoswarm".to_string()];
            new_args.extend(subcmd.split_whitespace().map(String::from));
            new_args.extend(args.into_iter().skip(1));
            return Some(new_args);
        }
    }

    None
}

fn run_with_args(args: &[String]) -> ExitCode {
    match Cli::try_parse_from(args) {
        Ok(cli) => run_cli(cli),
        Err(e) => {
            eprintln!("{e}");
            print_null();
            exit_code_for_mode()
        }
    }
}

fn run_cli(cli: Cli) -> ExitCode {
    match execute_command(cli.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e:#}");
            print_null();
            exit_code_for_mode()
        }
    }
}

fn execute_command(cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Count(sub) => commands::count::run(sub),
        Command::Ms(sub) => commands::ms::run(sub),
        Command::Yaml(sub) => commands::yaml::run(sub),
        Command::Inv(sub) => commands::inv::run(sub),
        Command::Line(sub) => commands::line::run(sub),
        Command::Receipts(sub) => commands::receipts::run(sub),
        Command::Receipt(sub) => commands::receipt::run(sub),
        Command::Openapi(sub) => commands::openapi::run(sub),
        Command::Index(sub) => commands::index::run(sub),
        Command::Time(sub) => commands::time::run(sub),
        Command::Openq(sub) => commands::openq::run(sub),
        Command::Secrets(sub) => commands::secrets::run(sub),
        Command::Version(sub) => commands::version::run(sub),
    }
}

fn exit_code_for_mode() -> ExitCode {
    if strict_mode() {
        ExitCode::from(2)
    } else {
        ExitCode::SUCCESS
    }
}

fn strict_mode() -> bool {
    matches!(env::var("DEMOSWARM_STRICT"), Ok(val) if {
        let lower = val.to_ascii_lowercase();
        lower == "1" || lower == "true" || lower == "yes"
    })
}

//! CLI command definitions and subcommand modules.

use clap::{Parser, Subcommand};

pub mod common;
pub mod count;
pub mod index;
pub mod inv;
pub mod line;
pub mod ms;
pub mod openapi;
pub mod openq;
pub mod receipt;
pub mod receipts;
pub mod secrets;
pub mod time;
pub mod version;
pub mod yaml;

/// Deterministic helpers for .runs/ operations.
#[derive(Parser, Debug)]
#[command(name = "demoswarm")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Count operations (pattern, bdd scenarios)
    Count(count::CountCommand),

    /// Machine Summary extraction
    Ms(ms::MsCommand),

    /// YAML block operations
    Yaml(yaml::YamlCommand),

    /// Inventory marker extraction
    Inv(inv::InvCommand),

    /// Line value extraction
    Line(line::LineCommand),

    /// Receipt counting (existing receipts in run dir)
    Receipts(receipts::ReceiptsCommand),

    /// Receipt field reading
    Receipt(receipt::ReceiptCommand),

    /// OpenAPI operations
    Openapi(openapi::OpenapiCommand),

    /// Index operations
    Index(index::IndexCommand),

    /// Time utilities
    Time(time::TimeCommand),

    /// Open questions tracking
    Openq(openq::OpenqCommand),

    /// Secrets scanning and redaction
    Secrets(secrets::SecretsCommand),

    /// Output version information as JSON
    Version(version::VersionCommand),
}

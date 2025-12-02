use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "kero", author, version, about = "Keyboard recorder controller")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Start the keyboard recorder in the background.
    Start,
    /// Stop the background keyboard recorder process.
    Stop,
    /// Query recorded keystrokes from the database.
    Query(QueryArgs),
    /// Clear recorded keystrokes from the database.
    Clear(ClearArgs),
    #[command(hide = true, name = "__daemon")]
    Daemon,
}

#[derive(Args, Debug, Default)]
pub struct QueryArgs {
    /// Only show single-key records.
    #[arg(short = 's', long = "single", conflicts_with = "combo_only")]
    pub single_only: bool,
    /// Only show combination key records.
    #[arg(short = 'c', long = "combo", conflicts_with = "single_only")]
    pub combo_only: bool,
}

#[derive(Args, Debug, Default)]
pub struct ClearArgs {
    /// Backup the database to .db.backup before clearing.
    #[arg(short = 'b', long = "backup")]
    pub backup: bool,
}

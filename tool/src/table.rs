use anyhow::Result;
use clap::{Args, Subcommand, ValueEnum};

#[derive(Debug, Args)]
pub struct Table {
    #[command(subcommand)]
    cmd: TableCmd,
}

impl Table {
    pub fn exec(self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Subcommand)]
enum TableCmd {
    /// List all table
    List,
    /// Create new table
    New(New),
}

#[derive(ValueEnum, Clone, Debug)]
enum TableType {
    Basic,
    Versioned,
}

#[derive(Debug, Args)]
struct New {
    /// Type of table
    #[arg(name = "type", short, long)]
    pub ty: TableType,

    /// Name of table
    pub name: String,
}

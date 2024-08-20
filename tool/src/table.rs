use anyhow::Result;
use clap::{Args, Subcommand};

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
pub enum TableCmd {
    List,
    New,
}

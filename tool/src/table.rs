use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Table {
    #[command(subcommand)]
    cmd: TableCmd,
}

#[derive(Debug, Subcommand)]
pub enum TableCmd {
    List,
    New,
}

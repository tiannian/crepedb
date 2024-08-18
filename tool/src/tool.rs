use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Backend {
    Redb,
    Mdbx,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, env = "CREPEDB_DB")]
    database: PathBuf,
    #[arg(short, long, env = "CREPEDB_BACKEND", default_value = "redb")]
    backend: Backend,

    #[command(subcommand)]
    subcmd: SubCmd,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    Table,
    Snapshot,
    Value,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}

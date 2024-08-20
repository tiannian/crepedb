use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use crepedb::CrepeDB;
use crepedb_redb::RedbDatabase;
use crepedb_tool::{Snapshot, Table};

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
    Table(Table),
    Snapshot(Snapshot),
    Value,
    Commit,
}

impl SubCmd {
    pub fn exec<B>(self, db: CrepeDB<B>) -> Result<()> {
        match self {
            Self::Table(t) => t.exec()?,
            Self::Snapshot(s) => s.exec()?,
            Self::Value => {}
            Self::Commit => {}
        }

        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    match args.backend {
        Backend::Redb => {
            let backend = RedbDatabase::open_or_create(&args.database).unwrap();
            let db = CrepeDB::new(backend);

            args.subcmd.exec(db).unwrap();
        }
        Backend::Mdbx => {
            let backend = RedbDatabase::memory().unwrap();
            let db = CrepeDB::new(backend);

            args.subcmd.exec(db).unwrap();
        }
    };
}

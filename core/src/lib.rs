//! # CrepeDB
//!
//! A versioned and forkable embedded database library.
//!
//! CrepeDB provides a multi-version concurrency control (MVCC) database with snapshot isolation.
//! It supports forking database snapshots and maintains version history efficiently.
//!
//! ## Features
//!
//! - **Versioned Storage**: Track changes across multiple versions
//! - **Snapshot Isolation**: Create and read from consistent snapshots
//! - **Fork Support**: Create new branches from any snapshot
//! - **Backend Abstraction**: Use different storage backends (e.g., redb)
//!
//! ## Example
//!
//! ```ignore
//! use crepedb::{CrepeDB, SnapshotId, TableType};
//!
//! // Create a database with a backend
//! let db = CrepeDB::new(backend);
//!
//! // Create root snapshot
//! let wtxn = db.write(None)?;
//! wtxn.create_table("my_table", &TableType::Versioned)?;
//! let root = wtxn.commit()?;
//!
//! // Write data
//! let wtxn = db.write(Some(root))?;
//! let mut table = wtxn.open_table("my_table")?;
//! table.set(b"key".to_vec(), b"value".to_vec())?;
//! let snapshot1 = wtxn.commit()?;
//!
//! // Read data
//! let rtxn = db.read(Some(snapshot1))?;
//! let table = rtxn.open_table("my_table")?;
//! let value = table.get(b"key".to_vec())?;
//! ```

#![no_std]

extern crate alloc;

#[cfg(feature = "tests")]
extern crate std;

pub mod backend;

pub mod types;

mod db;
pub use db::*;

mod read_txn;
pub use read_txn::*;

mod read_table;
pub use read_table::*;

mod write_txn;
pub use write_txn::*;

mod write_table;
pub use write_table::*;

mod error;
pub use error::*;

pub(crate) mod utils;

#[doc(hidden)]
#[cfg(feature = "tests")]
pub mod tests {
    use crate::{
        backend::{Backend, BackendError, ReadTxn as BackendReadTxn},
        types::SnapshotId,
        utils::snapshot_reader,
        CrepeDB, ReadTxn, Result,
    };

    use super::utils::index_reader;

    pub fn check_index_10<T, E>(txn: ReadTxn<T, E>) -> Result<()>
    where
        T: BackendReadTxn<E>,
        E: BackendError,
    {
        let idx = index_reader(&txn.txn)?;
        let snp = snapshot_reader(&txn.txn)?;

        let snapshot = txn.snapshot_id.clone();

        macro_rules! check_inner {
            (
                $p:expr,
                $bi:literal,
                $ei:literal => None,
                $( $i:literal => $v:literal),*
            ) => {
                {
                    let (v, n) = snp.read(&$p)?;
                    assert_eq!(v, $bi.into());
                    $(

                        {
                            let s = idx.read(&$p, $i)?.unwrap();
                            let (v, _) = snp.read(&s)?;
                            assert_eq!(v, $v.into());
                        }
                    )*

                    let sp = idx.read(&$p, $ei)?;
                    assert_eq!(sp, None);
                    n
                }
            };

            (
                $p:expr,
                $bi:literal,
                $ei:literal => $ev:literal,
                $( $i:literal => $v:literal),*
            ) => {
                {
                    let (v, n) = snp.read(&$p)?;
                    assert_eq!(v, $bi.into());

                    $(

                        {
                            let s = idx.read(&$p, $i)?.unwrap();
                            let (v, _) = snp.read(&s)?;
                            assert_eq!(v, $v.into());
                        }
                    )*

                    let s = idx.read(&$p, $ei)?.unwrap();
                    let (v, _) = snp.read(&s)?;
                    assert_eq!(v, $ev.into());
                    n
                }
            };

            (
                $p:expr,
                $bi:literal
            ) => {
                {
                    let (v, n) = snp.read(&$p)?;
                    assert_eq!(v, $bi.into());

                    n
                }
            }

        }

        let ppp = check_inner!(snapshot, 11, 4 => None, 1 => 9, 2 => 7, 3 => 3);
        let ppp = check_inner!(ppp,      10, 4 => None, 1 => 8, 2 => 6, 3 => 2);
        let ppp = check_inner!(ppp,      9,  4 => None, 1 => 7, 2 => 5, 3 => 1);
        let ppp = check_inner!(ppp,      8,  3 => 0,    1 => 6, 2 => 4);
        let ppp = check_inner!(ppp,      7,  3 => None, 1 => 5, 2 => 3);
        let ppp = check_inner!(ppp,      6,  3 => None, 1 => 4, 2 => 2);
        let ppp = check_inner!(ppp,      5,  3 => None, 1 => 3, 2 => 1);
        let ppp = check_inner!(ppp,      4,  2 => 0,    1 => 2);
        let ppp = check_inner!(ppp,      3,  2 => None, 1 => 1);
        let ppp = check_inner!(ppp,      2,  1 => 0,);
        let ppp = check_inner!(ppp, 1);
        check_inner!(ppp, 0);

        Ok(())
    }

    pub fn test_db_10(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);

        let write_txn = db.write(None)?;
        log::info!("{:?}", write_txn);

        write_txn.commit()?;

        let mut sid = SnapshotId::root();

        for _ in 1..12 {
            let write_txn = db.write(Some(sid))?;
            log::info!("{:?}", write_txn);

            let nsid = write_txn.commit()?;

            sid = nsid;
        }

        let txn = db.read(Some(sid))?;

        check_index_10(txn)?;

        Ok(())
    }
}

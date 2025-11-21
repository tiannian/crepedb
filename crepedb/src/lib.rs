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
//! let wtxn = db.write(SnapshotId::preroot())?;
//! wtxn.create_table("my_table", &TableType::Versioned)?;
//! let root = wtxn.commit()?;
//!
//! // Write data
//! let wtxn = db.write(root)?;
//! let mut table = wtxn.open_table("my_table")?;
//! table.set(b"key".to_vec(), b"value".to_vec())?;
//! let snapshot1 = wtxn.commit()?;
//!
//! // Read data
//! let rtxn = db.read(snapshot1)?;
//! let table = rtxn.open_table("my_table")?;
//! let value = table.get(b"key".to_vec())?;
//! ```

#![no_std]

extern crate alloc;

#[cfg(feature = "tests")]
extern crate std;

pub mod backend;

mod types;
pub use types::*;

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

pub mod utils;

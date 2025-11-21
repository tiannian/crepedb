//! # CrepeDB Redb Backend
//!
//! This crate provides a [redb](https://github.com/cberner/redb) backend implementation for CrepeDB.
//!
//! Redb is a simple, portable, high-performance, ACID, embedded key-value database.
//! This backend allows CrepeDB to use redb as its underlying storage engine.
//!
//! ## Features
//!
//! - **ACID Transactions**: Full transactional support via redb
//! - **Embedded**: No separate server process required
//! - **Persistent Storage**: Data is stored on disk
//! - **In-Memory Mode**: Support for temporary in-memory databases
//!
//! ## Example
//!
//! ```ignore
//! use crepedb::{CrepeDB, SnapshotId, TableType};
//! use crepedb_redb::RedbDatabase;
//!
//! // Open a persistent database
//! let backend = RedbDatabase::open_or_create("mydb.redb")?;
//! let db = CrepeDB::new(backend);
//!
//! // Or create an in-memory database
//! let backend = RedbDatabase::memory()?;
//! let db = CrepeDB::new(backend);
//! ```

mod db;
pub use db::*;

mod read;
pub use read::*;

mod write;
pub use write::*;

mod table;
pub use table::*;

mod range;
pub use range::*;

pub(crate) mod types;

#[cfg(test)]
mod tests {
    use crate::RedbDatabase;

    #[test]
    fn test_db_10() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::utils::tests::test_db_10(backend).unwrap();
    }
}

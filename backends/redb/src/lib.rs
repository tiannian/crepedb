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

        crepedb_core::tests::test_db_10(backend).unwrap();
    }

    #[test]
    fn test_read() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_read(backend).unwrap();
    }

    #[test]
    fn test_snapshot_isolation() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_snapshot_isolation(backend).unwrap();
    }

    #[test]
    fn test_multiple_keys() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_multiple_keys(backend).unwrap();
    }

    #[test]
    fn test_error_handling() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_error_handling(backend).unwrap();
    }

    #[test]
    fn test_multiple_tables() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_multiple_tables(backend).unwrap();
    }

    #[test]
    fn test_basic_table_type() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_basic_table_type(backend).unwrap();
    }

    #[test]
    fn test_edge_cases() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_edge_cases(backend).unwrap();
    }

    #[test]
    fn test_version_chain() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_version_chain(backend).unwrap();
    }

    #[test]
    fn test_delete_operations() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_delete_operations(backend).unwrap();
    }

    #[test]
    fn test_root_snapshot() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_root_snapshot(backend).unwrap();
    }

    #[test]
    fn test_transaction_lifecycle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_transaction_lifecycle(backend).unwrap();
    }

    #[test]
    fn test_mixed_operations() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_mixed_operations(backend).unwrap();
    }

    #[test]
    fn test_parent_child_visibility() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_parent_child_visibility(backend).unwrap();
    }

    #[test]
    fn test_complex_branching() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb_core::read_tests::test_complex_branching(backend).unwrap();
    }
}

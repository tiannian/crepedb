//! # CrepeDB RocksDB Backend
//!
//! This crate provides a [RocksDB](https://rocksdb.org/) backend implementation for CrepeDB.
//!
//! RocksDB is a high-performance embedded database for key-value data. It is a fork of LevelDB
//! by Facebook that provides improved performance and many additional features.
//! This backend allows CrepeDB to use RocksDB as its underlying storage engine.
//!
//! ## Features
//!
//! - **ACID Transactions**: Full transactional support via RocksDB's OptimisticTransactionDB
//! - **Embedded**: No separate server process required
//! - **Persistent Storage**: Data is stored on disk
//! - **In-Memory Mode**: Support for temporary in-memory databases
//! - **High Performance**: Optimized for fast reads and writes
//!
//! ## Example
//!
//! ```ignore
//! use crepedb::{CrepeDB, SnapshotId, TableType};
//! use crepedb_rocksdb::RocksdbDatabase;
//!
//! // Open a persistent database
//! let backend = RocksdbDatabase::open_or_create("mydb")?;
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

#[cfg(test)]
mod tests {
    use crate::RocksdbDatabase;
    use std::path::PathBuf;

    fn create_temp_db() -> (RocksdbDatabase, PathBuf) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "crepedb-rocksdb-test-{}-{}",
            std::process::id(),
            timestamp
        ));
        let backend = RocksdbDatabase::open_or_create(&temp_dir).unwrap();
        (backend, temp_dir)
    }

    #[test]
    fn test_db_10() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::tests::test_db_10(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_read() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_read(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_snapshot_isolation() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_snapshot_isolation(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_multiple_keys() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_multiple_keys(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_error_handling() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_error_handling(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_multiple_tables() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_multiple_tables(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_basic_table_type() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_basic_table_type(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_edge_cases() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_edge_cases(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_version_chain() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_version_chain(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_delete_operations() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_delete_operations(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_root_snapshot() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_root_snapshot(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_transaction_lifecycle() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_transaction_lifecycle(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_mixed_operations() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_mixed_operations(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_parent_child_visibility() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_parent_child_visibility(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }

    #[test]
    fn test_complex_branching() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (backend, temp_dir) = create_temp_db();

        let result = crepedb_core::read_tests::test_complex_branching(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }
}

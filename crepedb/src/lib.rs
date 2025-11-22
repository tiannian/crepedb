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
//! - **Backend Abstraction**: Use different storage backends (e.g., redb, rocksdb, mdbx)
//!
//! ## Example
//!
//! ```ignore
//! use crepedb::{CrepeDB, SnapshotId};
//! use crepedb::backend::RedbDatabase;
//!
//! // Create a database with a backend
//! let backend = RedbDatabase::memory()?;
//! let db = CrepeDB::new(backend);
//!
//! // Create root snapshot
//! let wtxn = db.write(None)?;
//! wtxn.create_versioned_table("my_table")?;
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

// Re-export all core types and modules
pub use crepedb_core::*;

/// Core types used throughout CrepeDB.
///
/// This module re-exports essential types from the core library, including:
/// - [`SnapshotId`](crate::types::SnapshotId): Unique identifier for database snapshots
/// - [`Bytes`](crate::types::Bytes): Byte array type used for keys and values
/// - [`Version`](crate::types::Version): Version number type for tracking changes
pub mod types {
    pub use crepedb_core::types::*;
}

/// Storage backend implementations.
///
/// This module provides access to different storage backend implementations
/// that can be used with CrepeDB. Each backend implements the [`Backend`](crepedb_core::backend::Backend) trait
/// and provides its own database type.
///
/// ## Available Backends
///
/// - **RedbDatabase**: A simple, portable, high-performance embedded key-value database
/// - **RocksdbDatabase**: A high-performance embedded database based on RocksDB
/// - **MdbxDatabase**: A fast, compact, powerful embedded transactional key-value database
///
/// ## Example
///
/// ```ignore
/// use crepedb::backend::RedbDatabase;
/// use crepedb::CrepeDB;
///
/// // Create a backend
/// let backend = RedbDatabase::memory()?;
/// let db = CrepeDB::new(backend);
/// ```
pub mod backend {
    /// Redb backend implementation.
    ///
    /// Redb is a simple, portable, high-performance, ACID, embedded key-value database.
    #[cfg(any(feature = "backend-redb", docsrs))]
    pub use crepedb_redb::RedbDatabase;

    /// RocksDB backend implementation.
    ///
    /// RocksDB is a high-performance embedded database for key-value data.
    #[cfg(any(feature = "backend-rocksdb", docsrs))]
    pub use crepedb_rocksdb::RocksdbDatabase;

    /// MDBX backend implementation.
    ///
    /// MDBX is a fast, compact, powerful, embedded, transactional key-value database.
    #[cfg(any(feature = "backend-mdbx", docsrs))]
    pub use crepedb_mdbx::MdbxDatabase;
}

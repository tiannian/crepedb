//! Storage backend traits and abstractions.
//!
//! This module defines the trait interfaces that storage backends must implement
//! to be used with CrepeDB. The backend abstraction allows CrepeDB to work with
//! different underlying storage engines.

use core::fmt::{Debug, Display};

use crate::Bytes;

/// Main trait for storage backends.
///
/// Implementors of this trait provide the underlying storage mechanism for CrepeDB.
/// The backend is responsible for managing transactions and providing access to tables.
pub trait Backend: Sized + 'static {
    /// The error type returned by backend operations.
    type Error: BackendError;

    /// The read transaction type.
    type ReadTxn<'a>: ReadTxn<Self::Error>;

    /// The write transaction type.
    type WriteTxn<'a>: WriteTxn<Self::Error>;

    /// Begin a read transaction.
    ///
    /// Read transactions provide a consistent view of the database at a point in time.
    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error>;

    /// Begin a write transaction.
    ///
    /// Write transactions allow modifications to the database.
    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error>;
}

/// Trait for backend error types.
///
/// Backend errors must be debuggable, displayable, and have a static lifetime.
pub trait BackendError: Debug + Display + 'static {}

impl<T> BackendError for T where T: Debug + Display + 'static {}

/// Trait for read transactions.
///
/// Read transactions provide read-only access to tables in the database.
pub trait ReadTxn<E> {
    /// The table type that can be opened for reading.
    type Table<'a>: ReadTable<E>
    where
        Self: 'a;

    /// Open a table for reading.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to open
    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, E>;
}

/// Trait for write transactions.
///
/// Write transactions provide read-write access to tables and can be committed.
pub trait WriteTxn<E> {
    /// The table type that can be opened for writing.
    type Table<'a>: WriteTable<E>
    where
        Self: 'a;

    /// Open a table for writing.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to open
    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, E>;

    /// Commit the write transaction.
    ///
    /// This persists all changes made during the transaction.
    fn commit(self) -> Result<(), E>;
}

/// Trait for read-only table access.
///
/// Provides methods to query data from a table.
pub trait ReadTable<E> {
    /// The range iterator type for this table.
    type Range<'a>: Range<E>
    where
        Self: 'a;

    /// Get the value associated with a key.
    ///
    /// Returns `None` if the key does not exist.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    fn get(&self, key: Bytes) -> Result<Option<Bytes>, E>;

    /// Create a range iterator over keys.
    ///
    /// # Arguments
    ///
    /// * `begin` - The inclusive start of the range
    /// * `end` - The exclusive end of the range
    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, E>;

    /// Get the name of the table.
    fn name(&self) -> &str;
}

/// Trait for writable table access.
///
/// Extends `ReadTable` with methods to modify data.
pub trait WriteTable<E>: ReadTable<E> {
    /// Set a key-value pair in the table.
    ///
    /// If the key already exists, its value is updated.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set
    /// * `value` - The value to associate with the key
    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), E>;

    /// Delete a key from the table.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete
    fn del(&mut self, key: Bytes) -> Result<(), E>;
}

/// Trait for range iterators.
///
/// Iterates over key-value pairs in a range.
pub trait Range<E> {
    /// Get the next key-value pair in the range.
    ///
    /// Returns `None` when the iteration is complete.
    fn back(&mut self) -> Result<Option<(Bytes, Bytes)>, E>;
}

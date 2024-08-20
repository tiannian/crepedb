//! Traits of storage backend

use core::fmt::{Debug, Display};

use crate::Bytes;

/// Trait for backend.
pub trait Backend: Sized + 'static {
    /// Error type.
    type Error: BackendError;

    /// Transaction type to read.
    type ReadTxn<'a>: BackendReadTxn<Self::Error>;

    /// Transaction type tp write
    type WriteTxn<'a>: BackendWriteTxn<Self::Error>;

    /// Create read transaction.
    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error>;

    /// Create write transaction.
    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error>;
}

/// BackendError
pub trait BackendError: Debug + Display + 'static {}

impl<T> BackendError for T where T: Debug + Display + 'static {}

/// Trait for read transaction
pub trait BackendReadTxn<E> {
    /// Read table type of transaction
    type Table<'a>: BackendReadTable<E>
    where
        Self: 'a;

    /// Open a read table on this transaction.
    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, E>;
}

/// Trait for write transaction
pub trait BackendWriteTxn<E> {
    /// Write table type of transaction.
    type Table<'a>: BackendWriteTable<E>
    where
        Self: 'a;

    /// Open a write table on this transaction.
    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, E>;

    /// Commit this transaction data.
    fn commit(self) -> Result<(), E>;
}

/// Trait for read table
pub trait BackendReadTable<E> {
    /// Range type
    type Range<'a>: Iterator<Item = (Bytes, Bytes)> + DoubleEndedIterator
    where
        Self: 'a;

    /// Iter type
    type Iter<'a>: Iterator<Item = (Bytes, Bytes)> + DoubleEndedIterator
    where
        Self: 'a;

    /// Get value by key
    fn get(&self, key: Bytes) -> Result<Option<Bytes>, E>;

    /// Get data's over the range of this table.
    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, E>;

    /// Get data's over all table.
    fn iter(&self) -> Result<Self::Iter<'_>, E>;

    /// Get name of this table.
    fn name(&self) -> &str;
}

/// Trait for read table
pub trait BackendWriteTable<E>: BackendReadTable<E> {
    /// Set value by key.
    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), E>;

    /// Del value by key.
    fn del(&mut self, key: Bytes) -> Result<(), E>;
}

use std::path::Path;

use crepedb_core::backend::Backend;
use redb::{backends::InMemoryBackend, Builder, Database, Error, ReadableDatabase};

use crate::{RedbReadTxn, RedbWriteTxn};

/// A CrepeDB backend implementation using redb.
///
/// This struct wraps a redb `Database` and implements the CrepeDB `Backend` trait,
/// allowing redb to be used as the storage engine for CrepeDB.
pub struct RedbDatabase {
    inner: Database,
}

impl RedbDatabase {
    /// Open or create a persistent redb database at the specified path.
    ///
    /// If the database doesn't exist, it will be created. If it exists, it will be opened.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the database should be stored
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be created or opened.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let db = RedbDatabase::open_or_create("mydb.redb")?;
    /// ```
    pub fn open_or_create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let db = Database::create(path)?;
        Ok(Self { inner: db })
    }

    /// Create an in-memory redb database.
    ///
    /// The database exists only in memory and will be lost when dropped.
    /// This is useful for testing or temporary storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be created.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let db = RedbDatabase::memory()?;
    /// ```
    pub fn memory() -> Result<Self, Error> {
        let backend = InMemoryBackend::new();

        let db = Builder::new().create_with_backend(backend)?;
        Ok(Self { inner: db })
    }

    /// Get a reference to the underlying redb database.
    pub fn inner(&self) -> &Database {
        &self.inner
    }

    /// Consume the database and return the underlying redb database.
    pub fn into_inner(self) -> Database {
        self.inner
    }
}

impl Backend for RedbDatabase {
    type Error = Error;

    type ReadTxn<'a> = RedbReadTxn;

    type WriteTxn<'a> = RedbWriteTxn;

    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error> {
        let txn = self.inner.begin_read()?;

        Ok(RedbReadTxn { inner: txn })
    }

    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error> {
        let txn = self.inner.begin_write()?;

        Ok(RedbWriteTxn { inner: txn })
    }
}

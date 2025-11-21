use std::path::Path;

use crepedb_core::backend::Backend;
use libmdbx::{Database, DatabaseOptions, Error, NoWriteMap};

use crate::{MdbxReadTxn, MdbxWriteTxn};

/// A CrepeDB backend implementation using MDBX.
///
/// This struct wraps a MDBX `Database` and implements the CrepeDB `Backend` trait,
/// allowing MDBX to be used as the storage engine for CrepeDB.
pub struct MdbxDatabase {
    inner: Database<NoWriteMap>,
}

impl MdbxDatabase {
    /// Open or create a persistent MDBX database at the specified path.
    ///
    /// If the database doesn't exist, it will be created. If it exists, it will be opened.
    ///
    /// # Arguments
    ///
    /// * `path` - The directory path where the database should be stored
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be created or opened.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let db = MdbxDatabase::open_or_create("mydb.mdbx")?;
    /// ```
    pub fn open_or_create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut options = DatabaseOptions::default();
        options.max_tables = Some(1024);

        let db = Database::open_with_options(path, options)?;

        Ok(Self { inner: db })
    }

    /// Get a reference to the underlying MDBX database.
    pub fn inner(&self) -> &Database<NoWriteMap> {
        &self.inner
    }

    /// Consume the database and return the underlying MDBX database.
    pub fn into_inner(self) -> Database<NoWriteMap> {
        self.inner
    }
}

impl Backend for MdbxDatabase {
    type Error = Error;

    type ReadTxn<'a> = MdbxReadTxn<'a>;

    type WriteTxn<'a> = MdbxWriteTxn<'a>;

    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error> {
        let txn = self.inner.begin_ro_txn()?;

        Ok(MdbxReadTxn { inner: txn })
    }

    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error> {
        let txn = self.inner.begin_rw_txn()?;

        Ok(MdbxWriteTxn { inner: txn })
    }
}

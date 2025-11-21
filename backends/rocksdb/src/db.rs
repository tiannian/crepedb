use std::path::Path;
use std::sync::Arc;

use crepedb_core::backend::Backend;
use rocksdb::{Error, OptimisticTransactionDB, Options};

use crate::{RocksdbReadTxn, RocksdbWriteTxn};

/// A CrepeDB backend implementation using RocksDB.
///
/// This struct wraps a RocksDB `OptimisticTransactionDB` and implements the CrepeDB `Backend` trait,
/// allowing RocksDB to be used as the storage engine for CrepeDB.
pub struct RocksdbDatabase {
    inner: Arc<OptimisticTransactionDB>,
}

impl RocksdbDatabase {
    /// Open or create a persistent RocksDB database at the specified path.
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
    /// let db = RocksdbDatabase::open_or_create("mydb")?;
    /// ```
    pub fn open_or_create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = OptimisticTransactionDB::open(&opts, path)?;
        Ok(Self {
            inner: Arc::new(db),
        })
    }
}

impl Backend for RocksdbDatabase {
    type Error = Error;

    type ReadTxn<'a> = RocksdbReadTxn;

    type WriteTxn<'a> = RocksdbWriteTxn<'a>;

    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error> {
        Ok(RocksdbReadTxn {
            db: Arc::clone(&self.inner),
        })
    }

    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error> {
        let txn = self.inner.transaction();
        Ok(RocksdbWriteTxn {
            inner: txn,
            db: Arc::clone(&self.inner),
        })
    }
}

use std::sync::Arc;

use crepedb_core::backend::WriteTxn;
use rocksdb::{Error, OptimisticTransactionDB, Transaction};

use crate::RocksdbWriteTable;

/// A write transaction wrapper for RocksDB.
///
/// Implements the CrepeDB `WriteTxn` trait, providing read-write access to tables
/// and the ability to commit changes.
pub struct RocksdbWriteTxn<'db> {
    pub(crate) inner: Transaction<'db, OptimisticTransactionDB>,
    pub(crate) db: Arc<OptimisticTransactionDB>,
}

impl<'db> WriteTxn<Error> for RocksdbWriteTxn<'db> {
    type Table<'a>
        = RocksdbWriteTable<'a>
    where
        Self: 'a;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {
        Ok(RocksdbWriteTable {
            txn: &self.inner,
            db: Arc::clone(&self.db),
            name: table.to_string(),
        })
    }

    fn commit(self) -> Result<(), Error> {
        self.inner.commit()?;
        Ok(())
    }
}

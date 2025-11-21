use std::sync::Arc;

use crepedb_core::{
    backend::{ReadTable, WriteTable},
    types::Bytes,
};
use rocksdb::{Error, OptimisticTransactionDB, Transaction};

use crate::RocksdbRange;

/// A read-only table wrapper for RocksDB.
///
/// Implements the CrepeDB `ReadTable` trait for RocksDB.
pub struct RocksdbReadTable {
    pub(crate) db: Arc<OptimisticTransactionDB>,
    pub(crate) name: String,
}

impl RocksdbReadTable {
    fn make_key(&self, key: &[u8]) -> Vec<u8> {
        let mut full_key = Vec::with_capacity(self.name.len() + 1 + key.len());
        full_key.extend_from_slice(self.name.as_bytes());
        full_key.push(b':');
        full_key.extend_from_slice(key);
        full_key
    }
}

impl ReadTable<Error> for RocksdbReadTable {
    type Range<'c>
        = RocksdbRange
    where
        Self: 'c;

    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self, key: Bytes) -> Result<Option<Bytes>, Error> {
        let full_key = self.make_key(&key);
        if let Some(value) = self.db.get(&full_key)? {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, Error> {
        let begin_key = self.make_key(&begin);
        let end_key = self.make_key(&end);

        Ok(RocksdbRange {
            db: Arc::clone(&self.db),
            begin: begin_key,
            end: end_key,
            prefix_len: self.name.len() + 1,
            current: None,
        })
    }
}

/// A writable table wrapper for RocksDB.
///
/// Implements both the CrepeDB `ReadTable` and `WriteTable` traits.
pub struct RocksdbWriteTable<'a> {
    pub(crate) txn: &'a Transaction<'a, OptimisticTransactionDB>,
    pub(crate) db: Arc<OptimisticTransactionDB>,
    pub(crate) name: String,
}

impl<'a> RocksdbWriteTable<'a> {
    fn make_key(&self, key: &[u8]) -> Vec<u8> {
        let mut full_key = Vec::with_capacity(self.name.len() + 1 + key.len());
        full_key.extend_from_slice(self.name.as_bytes());
        full_key.push(b':');
        full_key.extend_from_slice(key);
        full_key
    }
}

impl<'a> ReadTable<Error> for RocksdbWriteTable<'a> {
    type Range<'c>
        = RocksdbRange
    where
        Self: 'c;

    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self, key: Bytes) -> Result<Option<Bytes>, Error> {
        let full_key = self.make_key(&key);
        if let Some(value) = self.txn.get(&full_key)? {
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, Error> {
        let begin_key = self.make_key(&begin);
        let end_key = self.make_key(&end);

        Ok(RocksdbRange {
            db: Arc::clone(&self.db),
            begin: begin_key,
            end: end_key,
            prefix_len: self.name.len() + 1,
            current: None,
        })
    }
}

impl<'a> WriteTable<Error> for RocksdbWriteTable<'a> {
    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), Error> {
        let full_key = self.make_key(&key);
        self.txn.put(&full_key, &value)?;
        Ok(())
    }

    fn del(&mut self, key: Bytes) -> Result<(), Error> {
        let full_key = self.make_key(&key);
        self.txn.delete(&full_key)?;
        Ok(())
    }
}

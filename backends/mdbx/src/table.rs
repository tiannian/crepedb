use crepedb_core::{
    backend::{ReadTable, WriteTable},
    Bytes,
};
use libmdbx::{Error, NoWriteMap, Table, Transaction, RO, RW};

use crate::MdbxRange;

/// A read-only table wrapper for MDBX.
///
/// Implements the CrepeDB `ReadTable` trait for MDBX's read-only transactions.
pub struct MdbxReadTable<'a> {
    pub(crate) inner: Table<'a>,
    pub(crate) txn: &'a Transaction<'a, RO, NoWriteMap>,
    pub(crate) name: String,
}

impl<'a> ReadTable<Error> for MdbxReadTable<'a> {
    type Range<'c>
        = MdbxRange<'c, RO>
    where
        Self: 'c;

    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self, key: Bytes) -> Result<Option<Bytes>, Error> {
        self.txn.get::<Vec<u8>>(&self.inner, &key)
    }

    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, Error> {
        let cursor = self.txn.cursor(&self.inner)?;
        
        Ok(MdbxRange {
            cursor,
            begin,
            end,
            started: false,
        })
    }
}

/// A writable table wrapper for MDBX.
///
/// Implements both the CrepeDB `ReadTable` and `WriteTable` traits for MDBX's write transactions.
pub struct MdbxWriteTable<'a> {
    pub(crate) inner: Table<'a>,
    pub(crate) txn: &'a Transaction<'a, RW, NoWriteMap>,
    pub(crate) name: String,
}

impl<'a> ReadTable<Error> for MdbxWriteTable<'a> {
    type Range<'c>
        = MdbxRange<'c, RW>
    where
        Self: 'c;

    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self, key: Bytes) -> Result<Option<Bytes>, Error> {
        self.txn.get::<Vec<u8>>(&self.inner, &key)
    }

    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, Error> {
        let cursor = self.txn.cursor(&self.inner)?;
        
        Ok(MdbxRange {
            cursor,
            begin,
            end,
            started: false,
        })
    }
}

impl<'a> WriteTable<Error> for MdbxWriteTable<'a> {
    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), Error> {
        self.txn.put(&self.inner, key, value, Default::default())?;
        Ok(())
    }

    fn del(&mut self, key: Bytes) -> Result<(), Error> {
        match self.txn.del(&self.inner, &key, None) {
            Ok(_) => Ok(()),
            Err(Error::NotFound) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

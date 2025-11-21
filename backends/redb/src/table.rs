use crepedb_core::{
    backend::{ReadTable, WriteTable},
    types::Bytes,
};
use redb::{Error, ReadOnlyTable, ReadableTable, Table, TableHandle};

use crate::{types::BytesTy, RedbRange};

/// A read-only table wrapper for redb.
///
/// Implements the CrepeDB `ReadTable` trait for redb's `ReadOnlyTable`.
pub struct RedbReadTable {
    pub(crate) inner: ReadOnlyTable<BytesTy, BytesTy>,
    pub(crate) name: String,
}

impl ReadTable<Error> for RedbReadTable {
    type Range<'c>
        = RedbRange<'c>
    where
        Self: 'c;

    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self, key: Bytes) -> Result<Option<crepedb_core::types::Bytes>, Error> {
        if let Some(r) = self.inner.get(key)? {
            Ok(Some(r.value()))
        } else {
            Ok(None)
        }
    }

    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, Error> {
        let r = self.inner.range(begin..end)?;

        Ok(RedbRange { inner: r })
    }
}

/// A writable table wrapper for redb.
///
/// Implements both the CrepeDB `ReadTable` and `WriteTable` traits for redb's `Table`.
pub struct RedbWriteTable<'a> {
    pub(crate) inner: Table<'a, BytesTy, BytesTy>,
}

impl<'a> ReadTable<Error> for RedbWriteTable<'a> {
    type Range<'c>
        = RedbRange<'c>
    where
        Self: 'c;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn get(&self, key: Bytes) -> Result<Option<crepedb_core::types::Bytes>, Error> {
        if let Some(r) = self.inner.get(key)? {
            Ok(Some(r.value()))
        } else {
            Ok(None)
        }
    }

    fn range(&self, begin: Bytes, end: Bytes) -> Result<Self::Range<'_>, Error> {
        let r = self.inner.range(begin..end)?;

        Ok(RedbRange { inner: r })
    }
}

impl<'a> WriteTable<Error> for RedbWriteTable<'a> {
    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), Error> {
        self.inner.insert(key, value)?;

        Ok(())
    }

    fn del(&mut self, key: Bytes) -> Result<(), Error> {
        self.inner.remove(key)?;

        Ok(())
    }
}

use crepedb::{
    backend::{ReadTable, WriteTable},
    Bytes,
};
use redb::{Error, ReadOnlyTable, ReadableTable, Table, TableHandle};

use crate::{types::BytesTy, RedbRange};

pub struct RedbReadTable<'txn> {
    pub(crate) inner: ReadOnlyTable<'txn, BytesTy, BytesTy>,
    pub(crate) name: String,
}

impl<'txn> ReadTable<Error> for RedbReadTable<'txn> {
    type Range<'c> = RedbRange<'c> where Self: 'c;

    fn name(&self) -> &str {
        &self.name
    }

    fn get(&self, key: Bytes) -> Result<Option<crepedb::Bytes>, Error> {
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

pub struct RedbWriteTable<'a, 'b> {
    pub(crate) inner: Table<'a, 'b, BytesTy, BytesTy>,
}

impl<'a, 'b> ReadTable<Error> for RedbWriteTable<'a, 'b> {
    type Range<'c> = RedbRange<'c> where Self: 'c;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn get(&self, key: Bytes) -> Result<Option<crepedb::Bytes>, Error> {
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

impl<'a, 'b> WriteTable<Error> for RedbWriteTable<'a, 'b> {
    fn set(&mut self, key: Bytes, value: Bytes) -> Result<(), Error> {
        self.inner.insert(key, value)?;

        Ok(())
    }

    fn del(&mut self, key: Bytes) -> Result<(), Error> {
        self.inner.remove(key)?;

        Ok(())
    }
}

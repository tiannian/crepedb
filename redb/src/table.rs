use crepedb::backend::{ReadTable, WriteTable};
use redb::{Error, ReadableTable, Table, TableHandle};

use crate::{types::BytesTy, RedbRange};

pub struct RedbWriteTable<'a, 'b> {
    pub(crate) inner: Table<'a, 'b, BytesTy, BytesTy>,
}

impl<'a, 'b> ReadTable<Error> for RedbWriteTable<'a, 'b> {
    type Range<'c> = RedbRange<'c> where Self: 'c;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn get(&self, key: &[u8]) -> Result<Option<crepedb::Bytes>, Error> {
        if let Some(r) = self.inner.get(key.to_vec())? {
            Ok(Some(r.value()))
        } else {
            Ok(None)
        }
    }

    fn range(&self, begin: &[u8], end: &[u8]) -> Result<Self::Range<'_>, Error> {
        let begin = begin.to_vec();
        let end = end.to_vec();

        let r = self.inner.range(begin..end)?;

        Ok(RedbRange { inner: r })
    }
}

impl<'a, 'b> WriteTable<Error> for RedbWriteTable<'a, 'b> {
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.inner.insert(key.to_vec(), value.to_vec())?;

        Ok(())
    }

    fn del(&mut self, key: &[u8]) -> Result<(), Error> {
        self.inner.remove(key.to_vec())?;

        Ok(())
    }
}

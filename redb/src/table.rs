use crepedb::backend::{ReadTable as BackendReadTable, WriteTable as BackendWriteTable};
use redb::{Error, Table};

use crate::types::BytesTy;

pub struct WriteTable<'a, 'b> {
    pub(crate) inner: Table<'a, 'b, BytesTy, BytesTy>,
}

impl<'a, 'b> BackendReadTable<Error> for WriteTable<'a, 'b> {}

impl<'a, 'b> BackendWriteTable<Error> for WriteTable<'a, 'b> {
    fn set(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.inner.insert(key.to_vec(), value.to_vec())?;

        Ok(())
    }

    fn del(&self, key: &[u8]) -> Result<(), Error> {
        self.inner.remove(key.to_vec())?;

        Ok(())
    }
}

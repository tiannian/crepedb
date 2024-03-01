use crepedb::backend::WriteTable as BackendWriteTable;
use redb::{Error, Table};

use crate::types::BytesTy;

pub struct WriteTable<'a, 'b> {
    pub(crate) inner: Table<'a, 'b, BytesTy, BytesTy>,
}

impl<'a, 'b> BackendWriteTable<Error> for WriteTable<'a, 'b> {
    fn set(&self, table: &str, key: &[u8], value: &[u8]) -> Result<(), Error> {
        self.inner.insert(key, value)?;
    }

    fn del(&self, table: &str, key: &[u8]) -> Result<(), Error> {
        self.inner.remove(key)
    }
}

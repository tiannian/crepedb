use crepedb_core::backend::ReadTxn;
use libmdbx::{Error, NoWriteMap, Transaction, RO};

use crate::MdbxReadTable;

/// A read transaction wrapper for MDBX.
///
/// Implements the CrepeDB `ReadTxn` trait, providing read-only access to tables.
pub struct MdbxReadTxn<'a> {
    pub(crate) inner: Transaction<'a, RO, NoWriteMap>,
}

impl<'a> ReadTxn<Error> for MdbxReadTxn<'a> {
    type Table<'b>
        = MdbxReadTable<'b>
    where
        Self: 'b;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {
        let inner = self.inner.open_table(Some(table))?;
        let name = table.to_string();

        Ok(MdbxReadTable { 
            inner,
            txn: &self.inner,
            name,
        })
    }
}

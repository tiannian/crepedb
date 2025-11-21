use crepedb_core::backend::WriteTxn;
use libmdbx::{Error, NoWriteMap, Transaction, RW};

use crate::MdbxWriteTable;

/// A write transaction wrapper for MDBX.
///
/// Implements the CrepeDB `WriteTxn` trait, providing read-write access to tables
/// and the ability to commit changes.
pub struct MdbxWriteTxn<'a> {
    pub(crate) inner: Transaction<'a, RW, NoWriteMap>,
}

impl<'a> WriteTxn<Error> for MdbxWriteTxn<'a> {
    type Table<'b>
        = MdbxWriteTable<'b>
    where
        Self: 'b;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {
        let inner = self.inner.create_table(Some(table), Default::default())?;
        let name = table.to_string();

        Ok(MdbxWriteTable { 
            inner,
            txn: &self.inner,
            name,
        })
    }

    fn commit(self) -> Result<(), Error> {
        self.inner.commit()?;
        Ok(())
    }
}

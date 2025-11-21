use crepedb_core::backend::WriteTxn;
use redb::{Error, TableDefinition, WriteTransaction};

use crate::RedbWriteTable;

/// A write transaction wrapper for redb.
///
/// Implements the CrepeDB `WriteTxn` trait, providing read-write access to tables
/// and the ability to commit changes.
pub struct RedbWriteTxn {
    pub inner: WriteTransaction,
}

impl WriteTxn<Error> for RedbWriteTxn {
    type Table<'a>
        = RedbWriteTable<'a>
    where
        Self: 'a;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {
        let definition = TableDefinition::new(table);
        let table = self.inner.open_table(definition)?;

        Ok(RedbWriteTable { inner: table })
    }

    fn commit(self) -> Result<(), Error> {
        self.inner.commit()?;

        Ok(())
    }
}

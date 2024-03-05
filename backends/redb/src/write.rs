use crepedb::backend::WriteTxn;
use redb::{Error, TableDefinition, WriteTransaction};

use crate::RedbWriteTable;

pub struct RedbWriteTxn<'db> {
    pub inner: WriteTransaction<'db>,
}

impl<'db> WriteTxn<Error> for RedbWriteTxn<'db> {
    type Table<'a> = RedbWriteTable<'db, 'a> where Self: 'a;

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

use crepedb::backend::ReadTxn;
use redb::{Error, ReadTransaction, TableDefinition};

use crate::RedbReadTable;

pub struct RedbReadTxn<'a> {
    pub(crate) inner: ReadTransaction<'a>,
}

impl<'a> ReadTxn<Error> for RedbReadTxn<'a> {
    type Table<'b> = RedbReadTable<'b> where Self: 'b;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {
        let name = table.into();
        let definition = TableDefinition::new(table);
        let inner = self.inner.open_table(definition)?;

        Ok(RedbReadTable { inner, name })
    }
}

#[cfg(test)]
mod tests {
    use crate::RedbDatabase;

    #[test]
    fn test_read() {
        let _ = env_logger::builder().is_test(true).try_init();

        let backend = RedbDatabase::memory().unwrap();

        crepedb::read_tests::test_read(backend).unwrap();
    }
}

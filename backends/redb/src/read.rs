use crepedb_core::backend::ReadTxn;
use redb::{Error, ReadTransaction, TableDefinition};

use crate::RedbReadTable;

/// A read transaction wrapper for redb.
///
/// Implements the CrepeDB `ReadTxn` trait, providing read-only access to tables.
pub struct RedbReadTxn {
    pub(crate) inner: ReadTransaction,
}

impl ReadTxn<Error> for RedbReadTxn {
    type Table<'b>
        = RedbReadTable
    where
        Self: 'b;

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

        crepedb_core::read_tests::test_read(backend).unwrap();
    }
}

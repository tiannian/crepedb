use crepedb::backend::ReadTxn;
use redb::{Error, ReadOnlyTable, ReadTransaction, ReadableTable, TableDefinition};

use crate::{types::BytesTy, RedbRange};

pub struct RedbReadTxn<'a> {
    pub(crate) inner: ReadTransaction<'a>,
}

// impl<'a> ReadTxn<Error> for RedbReadTxn<'a> {
//     type Table<'a> = Redb;
//
//     fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {}
// }

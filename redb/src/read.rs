use crepedb::backend::ReadTxn;
use redb::{Error, ReadOnlyTable, ReadTransaction, ReadableTable, TableDefinition};

use crate::{types::BytesTy, RedbRange};

pub struct RedbReadTxn<'a> {
    pub(crate) inner: ReadTransaction<'a>,
}

// impl<'a> ReadTxn<Error> for RedbReadTxn<'a> {
//     type Range<'b> = RedbRange<'b> where Self:'b;
//
//     fn get(&self, table: &str, key: &[u8]) -> Result<Option<crepedb::Bytes>, Error> {
//         let table: ReadOnlyTable<'_, BytesTy, BytesTy> =
//             self.inner.open_table(TableDefinition::new(table))?;
//
//         if let Some(r) = table.get(key.to_vec())? {
//             Ok(Some(r.value()))
//         } else {
//             Ok(None)
//         }
//     }
//
//     fn range(&self, table: &str, begin: &[u8], end: &[u8]) -> Result<Self::Range<'_>, Error> {
//         let table: ReadOnlyTable<'_, BytesTy, BytesTy> =
//             self.inner.open_table(TableDefinition::new(table))?;
//
//         let range = table.range(begin.to_vec()..end.to_vec())?;
//
//         Ok(RedbRange { inner: range })
//     }
// }

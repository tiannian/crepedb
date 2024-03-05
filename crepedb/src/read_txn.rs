use core::marker::PhantomData;

use crate::{
    backend::{BackendError, ReadTxn as BackendReadTxn},
    utils, Error, ReadTable, Result, SnapshotId,
};

pub struct ReadTxn<T, E> {
    pub(crate) txn: T,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> ReadTxn<T, E>
where
    T: BackendReadTxn<E>,
    E: BackendError,
{
    pub fn open_table(
        &self,
        table: &str,
        snapshot_id: SnapshotId,
    ) -> Result<ReadTable<T::Table<'_>, E>> {
        let meta = utils::meta_reader(&self.txn)?;
        let table_type = meta.read_type(table)?;

        let table = self.txn.open_table(table).map_err(Error::backend)?;

        let index = utils::index_reader(&self.txn)?;

        let sr = utils::snapshot_reader(&self.txn)?;
        let (version, _) = sr.read(&snapshot_id)?;

        let table = ReadTable {
            table,
            index,
            table_type,
            snapshot_id,
            version,
            marker: PhantomData,
        };

        Ok(table)
    }
}

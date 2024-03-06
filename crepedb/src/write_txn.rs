use core::{fmt::Debug, marker::PhantomData};

use crate::{
    backend::{BackendError, WriteTxn as BackendWriteTxn},
    utils, Error, Result, SnapshotId, TableType, WriteTable,
};

pub struct WriteTxn<T, E> {
    pub(crate) txn: T,

    // None if write to root node
    pub(crate) parent_snapshot_id: Option<SnapshotId>,
    pub(crate) snapshot_id: SnapshotId,
    pub(crate) new_snapshot_id: SnapshotId,
    pub(crate) version: u64,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> Debug for WriteTxn<T, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{{ parent_snapshot_id: {:?}, ",
            self.parent_snapshot_id
        ))?;
        f.write_fmt(format_args!("snapshot_id: {:?}, ", self.snapshot_id))?;
        f.write_fmt(format_args!(
            "new_snapshot_id: {:?}, ",
            self.new_snapshot_id
        ))?;
        f.write_fmt(format_args!("version: {} }}", self.version))
    }
}

impl<T, E> WriteTxn<T, E>
where
    T: BackendWriteTxn<E>,
    E: BackendError,
{
    pub fn create_table(&self, table: &str, ty: &TableType) -> Result<()> {
        let mut meta = utils::meta_writer(&self.txn)?;

        meta.write_type(table, ty)?;

        Ok(())
    }

    pub fn open_table(&self, table: &str) -> Result<WriteTable<T::Table<'_>, E>> {
        let meta = utils::meta_reader_by_write(&self.txn)?;

        let table_type = meta.read_type(table)?;

        let table = WriteTable {
            marker: PhantomData,
            table_type,
            snapshot_id: self.new_snapshot_id.clone(),
            table: self.txn.open_table(table).map_err(Error::backend)?,
            version: self.version,
        };

        Ok(table)
    }

    pub fn commit(self) -> Result<SnapshotId> {
        {
            let mut snapshot = utils::snapshot_writer(&self.txn)?;

            // write snapshot info
            snapshot.write(&self.new_snapshot_id, &self.snapshot_id, self.version)?;

            // write next snapshot id
            snapshot.write_next_snapahot(&self.new_snapshot_id)?;
        }

        if let Some(parent_snapshot_id) = self.parent_snapshot_id {
            // Must not be root
            // build index
            let mut index = utils::index_writer(&self.txn)?;

            index.write(&self.new_snapshot_id, &parent_snapshot_id, self.version)?;
        }

        let new_snapshot_id = self.new_snapshot_id;

        self.txn.commit().map_err(Error::backend)?;

        Ok(new_snapshot_id)
    }
}

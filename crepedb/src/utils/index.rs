use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    backend::{BackendError, ReadTable, ReadTxn, WriteTable, WriteTxn},
    utils::fast_ceil_log2,
    Error, Result, SnapshotId,
};

use super::consts;

pub struct IndexTable<T, E> {
    table: T,
    marker: PhantomData<E>,
}

pub fn index_reader<T, E>(txn: &T) -> Result<IndexTable<T::Table<'_>, E>>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let table = txn
        .open_table(consts::SNAPSHOT_INDEX_TABLE)
        .map_err(Error::backend)?;
    Ok(IndexTable {
        table,
        marker: PhantomData,
    })
}

pub fn index_writer<T, E>(txn: &T) -> Result<IndexTable<T::Table<'_>, E>>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let table = txn
        .open_table(consts::SNAPSHOT_INDEX_TABLE)
        .map_err(Error::backend)?;
    Ok(IndexTable {
        table,
        marker: PhantomData,
    })
}

impl<T, E> IndexTable<T, E>
where
    T: ReadTable<E>,
    E: BackendError,
{
    pub fn read(&self, snapshot: &SnapshotId, n: u32) -> Result<Option<SnapshotId>> {
        let mut key = Vec::with_capacity(12);

        key.extend_from_slice(&snapshot.to_bytes());
        key.extend_from_slice(&n.to_le_bytes());

        let bytes = self.table.get(&key).map_err(Error::backend)?;

        if let Some(bytes) = bytes {
            let s = SnapshotId::from_bytes(&bytes)?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }
}

impl<T, E> IndexTable<T, E>
where
    T: WriteTable<E>,
    E: BackendError,
{
    fn write_index(&self, snapshot: &SnapshotId, n: u32, to: &SnapshotId) -> Result<()> {
        let mut key = Vec::with_capacity(12);

        key.extend_from_slice(&snapshot.to_bytes());
        key.extend_from_slice(&n.to_le_bytes());

        self.table
            .set(&key, &to.to_bytes())
            .map_err(Error::backend)?;

        Ok(())
    }

    pub fn write(&self, snapshot: &SnapshotId, k1: &SnapshotId, version: u64) -> Result<()> {
        debug_assert!(version >= 1);

        let step = fast_ceil_log2(version - 1);

        if step == 0 {
            return Ok(());
        }

        // Inser kn, n > 1
        for i in 1..step {
            if i == 1 {
                log::debug!("Insert version {version}, index 1");
                self.write_index(snapshot, 1, k1)?;
            } else {
                let ii = i - 1;

                log::debug!("Get `i(V{version}, {ii})`");
                let ki_1 = self
                    .read(snapshot, ii)?
                    .ok_or(Error::FatelMissingInnerIndex)?;

                log::debug!("Get `i(i(V{version}, {ii}), {ii})`");
                let ki_1 = self.read(&ki_1, ii)?;

                if let Some(ki_1) = ki_1 {
                    self.write_index(snapshot, i, &ki_1)?;
                } else {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    backend::{BackendError, ReadTable, ReadTxn, WriteTable, WriteTxn},
    Error, Result, SnapshotId, Version,
};

use super::consts;

pub struct SnapshotTable<T, E> {
    table: T,
    marker: PhantomData<E>,
}

pub fn snapshot_reader<T, E>(txn: &T) -> Result<SnapshotTable<T::Table<'_>, E>>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let table = txn
        .open_table(consts::SNAPSHOT_TABLE)
        .map_err(Error::backend)?;
    Ok(SnapshotTable {
        table,
        marker: PhantomData,
    })
}

pub fn snapshot_writer<T, E>(txn: &T) -> Result<SnapshotTable<T::Table<'_>, E>>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let table = txn
        .open_table(consts::SNAPSHOT_TABLE)
        .map_err(Error::backend)?;
    Ok(SnapshotTable {
        table,
        marker: PhantomData,
    })
}

impl<T, E> SnapshotTable<T, E>
where
    T: ReadTable<E>,
    E: BackendError,
{
    pub fn read(&self, snapshot_id: &SnapshotId) -> Result<(Version, SnapshotId)> {
        let bytes = self
            .table
            .get(snapshot_id.to_bytes().to_vec())
            .map_err(Error::backend)?
            .ok_or(Error::MissingSnaopshot(snapshot_id.clone()))?;

        let r = Version::from_bytes(&bytes)?;
        let s = SnapshotId::from_bytes(&bytes[8..])?;

        Ok((r, s))
    }

    pub fn has(&self, snapshot_id: &SnapshotId) -> Result<bool> {
        let bytes = self
            .table
            .get(snapshot_id.to_bytes().to_vec())
            .map_err(Error::backend)?;
        Ok(bytes.is_some())
    }

    pub fn read_next_snapshot_id(&self) -> Result<SnapshotId> {
        let bytes = self
            .table
            .get(consts::SNAPSHOT_NEXT_KEY.to_vec())
            .map_err(Error::backend)?;

        if let Some(bytes) = bytes {
            Ok(SnapshotId::from_bytes(&bytes)?)
        } else {
            Ok(SnapshotId::root())
        }
    }
}

impl<T, E> SnapshotTable<T, E>
where
    T: WriteTable<E>,
    E: BackendError,
{
    pub fn write(
        &mut self,
        snapshot_id: &SnapshotId,
        parent: &SnapshotId,
        version: &Version,
    ) -> Result<()> {
        let mut value = Vec::with_capacity(16);

        value.extend_from_slice(&version.to_bytes());
        value.extend_from_slice(&parent.to_bytes());

        self.table
            .set(snapshot_id.to_bytes().to_vec(), value)
            .map_err(Error::backend)?;

        Ok(())
    }

    pub fn write_next_snapahot(&mut self, snapshot_id: &SnapshotId) -> Result<()> {
        let snapshot = SnapshotId(snapshot_id.0 + 1);

        self.table
            .set(
                consts::SNAPSHOT_NEXT_KEY.to_vec(),
                snapshot.to_bytes().to_vec(),
            )
            .map_err(Error::backend)?;

        Ok(())
    }
}

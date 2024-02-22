use alloc::vec::Vec;

use crate::{
    backend::{BackendError, ReadTxn, WriteTxn},
    Error, Result, SnapshotId,
};

use super::{consts, snapshot};

pub fn read<T, E>(txn: &T, snapshot: &SnapshotId, n: u32) -> Result<Option<SnapshotId>>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let mut key = Vec::with_capacity(12);

    key.extend_from_slice(&snapshot.to_bytes());
    key.extend_from_slice(&n.to_le_bytes());

    let bytes = txn
        .get(consts::SNAPSHOT_INDEX_TABLE, &key)
        .map_err(Error::backend)?;

    if let Some(bytes) = bytes {
        let s = SnapshotId::from_bytes(&bytes)?;
        Ok(Some(s))
    } else {
        Ok(None)
    }
}

pub fn write_index<T, E>(txn: &T, snapshot: &SnapshotId, n: u32, to: &SnapshotId) -> Result<()>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let mut key = Vec::with_capacity(12);

    key.extend_from_slice(&snapshot.to_bytes());
    key.extend_from_slice(&n.to_le_bytes());

    txn.set(consts::SNAPSHOT_INDEX_TABLE, &key, &to.to_bytes())
        .map_err(Error::backend)?;

    Ok(())
}

pub fn write<T, E>(txn: &T, snapshot: &SnapshotId, k1: &SnapshotId, version: u64) -> Result<()>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let step = version.ilog2() + 1;

    // insert k1;
    write_index(txn, snapshot, 1, k1)?;

    let mut offset = k1.clone();

    // insert kn, n > 1
    for i in 2..step {
        let ki_1 = read(txn, &offset, i - 1)?;

        if let Some(ki_1) = ki_1 {
            write_index(txn, snapshot, i, &ki_1)?;
            offset = ki_1;
        } else {
            return Ok(());
        }
    }

    Ok(())
}

use alloc::vec::Vec;

use crate::{
    backend::{BackendError, ReadTxn, WriteTxn},
    Error, Result, SnapshotId,
};

use super::{consts, parse_u64};

pub fn read<T, E>(txn: &T, snapshot_id: &SnapshotId) -> Result<(u64, SnapshotId)>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let bytes = txn
        .get(consts::SNAPSHOT_TABLE, &snapshot_id.to_bytes())
        .map_err(Error::backend)?
        .ok_or(Error::MissingSnaopshot(snapshot_id.clone()))?;

    let r = parse_u64(&bytes)?;
    let s = SnapshotId::from_bytes(&bytes[6..])?;

    Ok((r, s))
}

pub fn write<T, E>(
    txn: &T,
    snapshot_id: &SnapshotId,
    parent: &SnapshotId,
    version: u64,
) -> Result<()>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let mut value = Vec::with_capacity(16);

    value.extend_from_slice(&version.to_le_bytes());
    value.extend_from_slice(&parent.to_bytes());

    txn.set(consts::SNAPSHOT_TABLE, &snapshot_id.to_bytes(), &value)
        .map_err(Error::backend)?;

    Ok(())
}

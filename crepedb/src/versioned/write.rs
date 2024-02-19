use alloc::vec::Vec;
use crepedb_core::{backend::Backend, SnapshotId};

use crate::{utils, Error, Result};

pub struct VersionedWriteTxn<'a, B>
where
    B: Backend,
{
    txn: B::WriteTxn<'a>,

    fork_nums: Vec<u64>,
    snapshot_num: u64,
    snapshot_id: SnapshotId,
}

impl<'a, B> VersionedWriteTxn<'a, B>
where
    B: Backend,
{
    pub(crate) fn new(backend: &'a B, from: SnapshotId, to: SnapshotId) -> Result<Self> {
        use crepedb_core::backend::{ReadTxn, WriteTxn};

        let txn = backend.write_txn().map_err(Error::backend)?;

        let forks_bytes = txn
            .get(utils::SNAPSHOT_FORK_TABLE, from.as_ref())
            .map_err(Error::backend)?
            .ok_or(Error::MissingKey)?;

        let forks_count = utils::parse_u32(&forks_bytes)?;
        if forks_count > 1 {
        } else {
        }

        Ok(Self {
            txn,
            fork_nums: Vec::new(),
            snapshot_num: 0,
            snapshot_id: to,
        })
    }
}

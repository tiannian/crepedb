use alloc::vec::Vec;

use crate::{
    backend::{Backend, WriteTxn as BackendWriteTxn},
    utils::{self, SnapshotTable},
    DataOp, Error, Result, SnapshotId, TableType,
};

pub struct WriteTxn<'a, B: Backend> {
    pub(crate) txn: B::WriteTxn<'a>,

    // None if write to root node
    pub(crate) parent_snapshot_id: Option<SnapshotId>,
    pub(crate) snapshot_id: SnapshotId,
    pub(crate) new_snapshot_id: SnapshotId,
    pub(crate) version: u64,
}

impl<'a, B> WriteTxn<'a, B>
where
    B: Backend,
{
    pub fn commit(self) -> Result<SnapshotId> {
        {
            let snapshot = utils::snapshot_writer(&self.txn)?;

            // write snapshot info
            snapshot.write(&self.new_snapshot_id, &self.snapshot_id, self.version)?;

            // write next snapshot id
            snapshot.write_next_snapahot(&self.new_snapshot_id)?;
        }
        if let Some(parent_snapshot_id) = self.parent_snapshot_id {
            // Must not be root
            // build index
            let index = utils::index_writer(&self.txn)?;

            index.write(&self.new_snapshot_id, &parent_snapshot_id, self.version)?;
        }

        let new_snapshot_id = self.new_snapshot_id;

        self.txn.commit().map_err(Error::backend)?;

        Ok(new_snapshot_id)
    }
}

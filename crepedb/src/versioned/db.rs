use crepedb_core::{backend::Backend, SnapshotId};

use crate::{Result, VersionedWriteTxn};

pub struct CrepeVersionedDB<B> {
    backend: B,
}

impl<B> CrepeVersionedDB<B>
where
    B: Backend,
{
    // pub fn read_txn(&self, id: SnapshotId) {}

    pub fn write_txn(&self, new: SnapshotId, from: SnapshotId) -> Result<VersionedWriteTxn<'_, B>> {
        VersionedWriteTxn::new(&self.backend, from, new)
    }
}

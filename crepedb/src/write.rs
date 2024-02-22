use alloc::vec::Vec;

use crate::{Result, SnapshotId};

pub struct WriteTxn {
    pub(crate) snapshot_id: SnapshotId,
    pub(crate) version: u64,
}

impl WriteTxn {
    pub fn set(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn del(&self, table: &str, key: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn commit(self) -> Result<SnapshotId> {
        Ok(SnapshotId(0))
    }
}

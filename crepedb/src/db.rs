use core::marker::PhantomData;

use crate::{backend::Backend, utils, Error, ReadTxn, Result, SnapshotId, Version, WriteTxn};

/// Versioned and forkable Database
pub struct CrepeDB<B> {
    pub(crate) backend: B,
}

impl<B> CrepeDB<B>
where
    B: Backend,
{
    /// Create database using backend.
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Create a transaction to read data.
    pub fn read(&self, snapshot_id: SnapshotId) -> Result<ReadTxn<B::ReadTxn<'_>, B::Error>> {
        let txn = self.backend.read_txn().map_err(Error::backend)?;

        Ok(ReadTxn {
            txn,
            snapshot_id,
            marker: PhantomData,
        })
    }

    /// Create a transaction to write data.
    pub fn write(&self, snapshot_id: SnapshotId) -> Result<WriteTxn<B::WriteTxn<'_>, B::Error>> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        if snapshot_id == SnapshotId::preroot() {
            // Create root.
            let snapshot = utils::snapshot_writer(&txn)?;

            if snapshot.has(&snapshot_id)? {
                return Err(Error::OnlySupportOneRoot);
            }

            drop(snapshot);

            Ok(WriteTxn {
                txn,
                version: Version::root(),
                new_snapshot_id: SnapshotId::root(),
                parent_snapshot_id: None,
                snapshot_id,
                marker: PhantomData,
            })
        } else {
            let snapshot = utils::snapshot_writer(&txn)?;

            let (version, parent_snapshot_id) = snapshot.read(&snapshot_id)?;

            let new_snapshot_id = snapshot.read_next_snapshot_id()?;

            drop(snapshot);

            Ok(WriteTxn {
                txn,
                version: (version.0 + 1).into(),
                new_snapshot_id,
                parent_snapshot_id: Some(parent_snapshot_id),
                snapshot_id,
                marker: PhantomData,
            })
        }
    }
}

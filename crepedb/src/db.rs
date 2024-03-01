use core::marker::PhantomData;

use crate::{backend::Backend, utils, Error, Result, SnapshotId, WriteTxn};

pub struct CrepeDB<B> {
    pub(crate) backend: B,
}

impl<B> CrepeDB<B>
where
    B: Backend,
{
    pub fn open(path: &str) -> Result<Self> {
        let backend = B::open_db(path).map_err(Error::backend)?;

        Ok(Self { backend })
    }

    pub fn open_readonly(path: &str) -> Result<Self> {
        let backend = B::open_readonly(path).map_err(Error::backend)?;

        Ok(Self { backend })
    }

    pub fn write(&self, snapshot_id: SnapshotId) -> Result<WriteTxn<B::WriteTxn<'_>, B::Error>> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        if snapshot_id == SnapshotId::preroot() {
            // Create root.
            // Need check already have root?

            let snapshot = utils::snapshot_writer(&txn)?;

            if snapshot.has(&snapshot_id)? {
                return Err(Error::OnlySupportOneRoot);
            }

            drop(snapshot);

            Ok(WriteTxn {
                txn,
                version: 0,
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
                version: version + 1,
                new_snapshot_id,
                parent_snapshot_id: Some(parent_snapshot_id),
                snapshot_id,
                marker: PhantomData,
            })
        }
    }
}

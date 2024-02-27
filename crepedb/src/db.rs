use crate::{
    backend::{Backend, WriteTxn},
    utils, Error, Result, SnapshotId, TableType, WriteTxn,
};

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

    pub fn write(&self, snapshot_id: SnapshotId) -> Result<WriteTxn<'_, B>> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        if snapshot_id == SnapshotId::preroot() {
            // Create root.
            // Need check already have root?

            let table = txn
                .open_table(utils::consts::SNAPSHOT_TABLE)
                .map_err(Error::backend)?;

            if utils::snapshot::has(&table, &snapshot_id)? {
                return Err(Error::OnlySupportOneRoot);
            }

            Ok(WriteTxn {
                txn,
                version: 0,
                new_snapshot_id: SnapshotId::root(),
                parent_snapshot_id: None,
                snapshot_id,
            })
        } else {
            let (version, parent_snapshot_id) = utils::snapshot::read(&txn, &snapshot_id)?;

            let new_snapshot_id = utils::snapshot::read_next_snapshot_id(&txn)?;

            Ok(WriteTxn {
                txn,
                version: version + 1,
                new_snapshot_id,
                parent_snapshot_id: Some(parent_snapshot_id),
                snapshot_id,
            })
        }
    }

    pub fn create_table(&self, table: &str, ty: &TableType) -> Result<()> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        utils::table::write_type(txn, table, ty)?;

        Ok(())
    }
}

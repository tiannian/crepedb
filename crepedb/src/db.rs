use crate::{backend::Backend, utils, Error, Result, SnapshotId, TableType, WriteTxn};

pub struct CrepeDB<B> {
    pub(crate) backend: B,
}

impl<B> CrepeDB<B>
where
    B: Backend,
{
    pub fn open(path: &str) -> Result<Self> {
        let backend = B::open_db(path).map_err(Error::backend)?;

        // TODO: Check db is init, If not init it.
        //
        // Create Fake Root.

        Ok(Self { backend })
    }

    pub fn open_readonly(path: &str) -> Result<Self> {
        let backend = B::open_readonly(path).map_err(Error::backend)?;

        Ok(Self { backend })
    }

    pub fn write(&self, snapshot_id: SnapshotId) -> Result<WriteTxn<'_, B>> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        if snapshot_id == SnapshotId::root() || snapshot_id == SnapshotId::unknown() {
            return Err(Error::WrongSnapshotIdMustBeCommon);
        }

        let (version, parent_snapshot_id) = utils::snapshot::read(&txn, &snapshot_id)?;

        let new_snapshot_id = utils::snapshot::read_next_snapshot_id(&txn)?;

        Ok(WriteTxn {
            txn,
            version: version + 1,
            new_snapshot_id,
            parent_snapshot_id,
            snapshot_id,
        })
    }

    pub fn create_table(&self, table: &str, ty: &TableType) -> Result<()> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        utils::table::write_type(txn, table, ty)?;

        Ok(())
    }
}

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

        Ok(Self { backend })
    }

    pub fn open_readonly(path: &str) -> Result<Self> {
        let backend = B::open_readonly(path).map_err(Error::backend)?;

        Ok(Self { backend })
    }

    // pub fn write(&self, snapshot_id: SnapshotId) -> Result<WriteTxn<'_, B>> {
    //     let txn = self.backend.write_txn().map_err(Error::backend)?;
    //
    //     let version = if snapshot_id != SnapshotId::root() {
    //         utils::snapshot::read(&txn, &snapshot_id)?.0
    //     } else {
    //         0
    //     };
    //
    //     Ok(WriteTxn {
    //         parent_snapshot_id: snapshot_id,
    //         snapshot_id,
    //         txn,
    //         version: version + 1,
    //     })
    // }

    pub fn create_table(&self, table: &str, ty: &TableType) -> Result<()> {
        let txn = self.backend.write_txn().map_err(Error::backend)?;

        utils::table::write_type(txn, table, ty)?;

        Ok(())
    }
}

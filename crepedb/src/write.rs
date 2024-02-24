use alloc::vec::Vec;

use crate::{
    backend::{Backend, WriteTxn as BackendWriteTxn},
    utils, DataOp, Error, Result, SnapshotId, TableType,
};

pub struct WriteTxn<'a, B: Backend> {
    pub(crate) txn: B::WriteTxn<'a>,

    // None if write to root nodr
    pub(crate) parent_snapshot_id: Option<SnapshotId>,
    pub(crate) snapshot_id: SnapshotId,
    pub(crate) new_snapshot_id: SnapshotId,
    pub(crate) version: u64,
}

impl<'a, B> WriteTxn<'a, B>
where
    B: Backend,
{
    /// Set Key-Value in table
    ///
    /// Table must be exist.
    pub fn set(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let table_type = utils::table::read_type(&self.txn, table)?;

        match table_type {
            TableType::Basic => self.set_basic(table, key, value),
            TableType::Versioned => self.set_versioned(table, key, value),
        }
    }

    fn set_basic(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        self.txn.set(table, &key, &value).map_err(Error::backend)?;

        Ok(())
    }

    fn set_versioned(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Set(value).to_bytes();

        self.txn.set(table, &key, &value).map_err(Error::backend)?;

        Ok(())
    }

    /// Set Value in table by Key
    ///
    /// Table must be exist.
    pub fn del(&self, table: &str, key: Vec<u8>) -> Result<()> {
        let table_type = utils::table::read_type(&self.txn, table)?;

        match table_type {
            TableType::Basic => self.del_basic(table, key),
            TableType::Versioned => self.del_versioned(table, key),
        }
    }

    fn del_basic(&self, table: &str, key: Vec<u8>) -> Result<()> {
        self.txn.del(table, &key).map_err(Error::backend)?;

        Ok(())
    }

    fn del_versioned(&self, table: &str, key: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Del.to_bytes();

        self.txn.set(table, &key, &value).map_err(Error::backend)?;

        Ok(())
    }

    fn build_key(&self, mut key: Vec<u8>) -> Vec<u8> {
        key.extend_from_slice(&self.version.to_le_bytes());
        key.extend_from_slice(&self.new_snapshot_id.to_bytes());

        key
    }

    pub fn commit(self) -> Result<SnapshotId> {
        // write snapshot info
        utils::snapshot::write(
            &self.txn,
            &self.new_snapshot_id,
            &self.snapshot_id,
            self.version,
        )?;

        // write next snapshot id
        utils::snapshot::write_next_snapahot(&self.txn, &self.new_snapshot_id)?;

        if let Some(parent_snapshot_id) = self.parent_snapshot_id {
            // Must not be root
            // build index
            utils::index::write(
                &self.txn,
                &self.new_snapshot_id,
                &parent_snapshot_id,
                self.version,
            )?;
        }

        self.txn.commit().map_err(Error::backend)?;

        Ok(self.new_snapshot_id)
    }
}

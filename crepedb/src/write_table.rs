use core::marker::PhantomData;

use crate::{
    backend::{BackendError, WriteTable as BackendWriteTable},
    Bytes, DataOp, Error, Result, SnapshotId, TableType, Version,
};

pub struct WriteTable<T, E> {
    pub(crate) table: T,

    pub(crate) table_type: TableType,

    pub(crate) snapshot_id: SnapshotId,
    pub(crate) version: Version,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> WriteTable<T, E>
where
    T: BackendWriteTable<E>,
    E: BackendError,
{
    /// Set Key-Value in table
    ///
    /// Table must be exist.
    pub fn set(&mut self, key: Bytes, value: Bytes) -> Result<()> {
        match self.table_type {
            TableType::Basic => self.set_basic(key, value),
            TableType::Versioned => self.set_versioned(key, value),
        }
    }

    fn set_basic(&mut self, key: Bytes, value: Bytes) -> Result<()> {
        self.table.set(key, value).map_err(Error::backend)?;

        Ok(())
    }

    fn set_versioned(&mut self, key: Bytes, value: Bytes) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Set(value).to_bytes();

        self.table.set(key, value).map_err(Error::backend)?;

        Ok(())
    }

    /// Set Value in table by Key
    ///
    /// Table must be exist.
    pub fn del(&mut self, key: Bytes) -> Result<()> {
        match self.table_type {
            TableType::Basic => self.del_basic(key),
            TableType::Versioned => self.del_versioned(key),
        }
    }

    fn del_basic(&mut self, key: Bytes) -> Result<()> {
        self.table.del(key).map_err(Error::backend)?;

        Ok(())
    }

    fn del_versioned(&mut self, key: Bytes) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Del.to_bytes();

        self.table.set(key, value).map_err(Error::backend)?;

        Ok(())
    }

    fn build_key(&self, mut key: Bytes) -> Bytes {
        key.extend_from_slice(&self.version.to_bytes());
        key.extend_from_slice(&self.snapshot_id.to_bytes());

        key
    }
}

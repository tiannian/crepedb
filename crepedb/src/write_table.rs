use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    backend::{BackendError, WriteTable as BackendWriteTable},
    utils::MetaTable,
    DataOp, Error, Result, SnapshotId, TableType,
};

pub struct WriteTable<T, E> {
    pub(crate) table: T,
    pub(crate) meta: MetaTable<T, E>,

    pub(crate) snapshot_id: SnapshotId,
    pub(crate) version: u64,

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
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let table = self.table.name();

        let table_type = self.meta.read_type(table)?;

        match table_type {
            TableType::Basic => self.set_basic(key, value),
            TableType::Versioned => self.set_versioned(key, value),
        }
    }

    fn set_basic(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        self.table.set(key, value).map_err(Error::backend)?;

        Ok(())
    }

    fn set_versioned(&mut self, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Set(value).to_bytes();

        self.table.set(key, value).map_err(Error::backend)?;

        Ok(())
    }

    /// Set Value in table by Key
    ///
    /// Table must be exist.
    pub fn del(&mut self, key: Vec<u8>) -> Result<()> {
        let table = self.table.name();

        let table_type = self.meta.read_type(table)?;

        match table_type {
            TableType::Basic => self.del_basic(key),
            TableType::Versioned => self.del_versioned(key),
        }
    }

    fn del_basic(&mut self, key: Vec<u8>) -> Result<()> {
        self.table.del(key).map_err(Error::backend)?;

        Ok(())
    }

    fn del_versioned(&mut self, key: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Del.to_bytes();

        self.table.set(key, value).map_err(Error::backend)?;

        Ok(())
    }

    fn build_key(&self, mut key: Vec<u8>) -> Vec<u8> {
        key.extend_from_slice(&self.version.to_le_bytes());
        key.extend_from_slice(&self.snapshot_id.to_bytes());

        key
    }
}

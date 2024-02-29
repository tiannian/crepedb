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

    marker: PhantomData<E>,
}

impl<T, E> WriteTable<T, E>
where
    T: BackendWriteTable<E>,
    E: BackendError,
{
    /// Set Key-Value in table
    ///
    /// Table must be exist.
    pub fn set(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let table_type = self.meta.read_type(table)?;

        match table_type {
            TableType::Basic => self.set_basic(table, key, value),
            TableType::Versioned => self.set_versioned(table, key, value),
        }
    }

    fn set_basic(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        self.table
            .set(table, &key, &value)
            .map_err(Error::backend)?;

        Ok(())
    }

    fn set_versioned(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Set(value).to_bytes();

        self.table
            .set(table, &key, &value)
            .map_err(Error::backend)?;

        Ok(())
    }

    /// Set Value in table by Key
    ///
    /// Table must be exist.
    pub fn del(&self, table: &str, key: Vec<u8>) -> Result<()> {
        let table_type = self.meta.read_type(table)?;

        match table_type {
            TableType::Basic => self.del_basic(table, key),
            TableType::Versioned => self.del_versioned(table, key),
        }
    }

    fn del_basic(&self, table: &str, key: Vec<u8>) -> Result<()> {
        self.table.del(table, &key).map_err(Error::backend)?;

        Ok(())
    }

    fn del_versioned(&self, table: &str, key: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Del.to_bytes();

        self.table
            .set(table, &key, &value)
            .map_err(Error::backend)?;

        Ok(())
    }

    fn build_key(&self, mut key: Vec<u8>) -> Vec<u8> {
        key.extend_from_slice(&self.version.to_le_bytes());
        key.extend_from_slice(&self.snapshot_id.to_bytes());

        key
    }
}

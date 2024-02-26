use std::{
    cell::{Ref, RefCell},
    collections::{btree_map::Range, BTreeMap},
    mem,
};

use crepedb::{
    backend::{Range as BRange, ReadTxn, WriteTxn},
    Bytes,
};

#[derive(Debug)]
pub enum Error {
    NoTargetTable,
}

pub type Result<T> = std::result::Result<T, Error>;

pub type BackendInner = BTreeMap<String, BTreeMap<Bytes, Bytes>>;

pub struct CrepeMemoryBackend {
    kv: BackendInner,
}

pub struct CrepeMemoryBackendReadTxn<'a> {
    inner: &'a BackendInner,
}

impl<'a> ReadTxn<Error> for CrepeMemoryBackendReadTxn<'a> {
    type Range<'b> = CrepeMemoryBackendRange<'b> where Self:'b;

    fn get(&self, table: &str, key: &[u8]) -> Result<Option<Bytes>> {
        if let Some(table) = self.inner.get(table) {
            if let Some(v) = table.get(key) {
                Ok(Some(v.clone()))
            } else {
                Ok(None)
            }
        } else {
            Err(Error::NoTargetTable)
        }
    }

    fn range(&self, table: &str, begin: &[u8], end: &[u8]) -> Result<Self::Range<'_>> {
        if let Some(table) = self.inner.get(table) {
            let begin = begin.to_vec();
            let end = end.to_vec();

            let inner = table.range(begin..end);

            Ok(CrepeMemoryBackendRange { inner })
        } else {
            Err(Error::NoTargetTable)
        }
    }
}

pub struct CrepeMemoryBackendRange<'a> {
    pub(crate) inner: Range<'a, Bytes, Bytes>,
}

impl<'a> BRange<Error> for CrepeMemoryBackendRange<'a> {
    fn next(&mut self) -> Result<Option<(Bytes, Bytes)>> {
        Ok(self.inner.next().map(|r| (r.0.clone(), r.1.clone())))
    }
}

pub struct CrepeMemoryBackendWriteTxn<'a> {
    inner: &'a mut BackendInner,
}
impl<'a> ReadTxn<Error> for CrepeMemoryBackendWriteTxn<'a> {
    type Range<'b> = CrepeMemoryBackendRange<'b> where Self:'b;

    fn get(&self, table: &str, key: &[u8]) -> Result<Option<Bytes>> {
        let read = CrepeMemoryBackendReadTxn { inner: &self.inner };

        read.get(table, key)
    }

    fn range(&self, table: &str, begin: &[u8], end: &[u8]) -> Result<Self::Range<'_>> {
        if let Some(table) = self.inner.get(table) {
            let begin = begin.to_vec();
            let end = end.to_vec();

            let inner = table.range(begin..end);

            Ok(CrepeMemoryBackendRange { inner })
        } else {
            Err(Error::NoTargetTable)
        }
    }
}

impl<'a> WriteTxn<Error> for CrepeMemoryBackendWriteTxn<'a> {
    fn set(&self, table: &str, key: &[u8], value: &[u8]) -> Result<()> {
        if let Some(table) = self.inner.get_mut(table) {
            table.insert(key.clone(), value.clone());

            Ok(())
        } else {
            Err(Error::NoTargetTable)
        }
    }
}

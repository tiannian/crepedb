use core::fmt::{Debug, Display};

use crate::Bytes;

pub trait Backend: Sized + 'static {
    type Error: BackendError;

    type ReadTxn<'a>: ReadTxn<Self::Error>;

    type WriteTxn<'a>: WriteTxn<Self::Error>;

    fn open_db(path: &str) -> Result<Self, Self::Error>;

    fn open_readonly(path: &str) -> Result<Self, Self::Error>;

    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error>;

    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error>;
}

pub trait BackendError: Debug + Display + 'static {}

pub trait ReadTxn<E> {
    type Table<'a>: ReadTable<E>
    where
        Self: 'a;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, E>;
}

pub trait WriteTxn<E> {
    type Table<'a>: WriteTable<E>
    where
        Self: 'a;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, E>;

    fn commit(self) -> Result<(), E>;
}

pub trait ReadTable<E> {
    type Range<'a>: Range<E>
    where
        Self: 'a;

    fn get(&self, key: &[u8]) -> Result<Option<Bytes>, E>;

    fn range(&self, begin: &[u8], end: &[u8]) -> Result<Self::Range<'_>, E>;

    fn name(&self) -> &str;
}

pub trait WriteTable<E>: ReadTable<E> {
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), E>;

    fn del(&mut self, key: &[u8]) -> Result<(), E>;
}

pub trait Range<E> {
    fn back(&mut self) -> Result<Option<(Bytes, Bytes)>, E>;
}

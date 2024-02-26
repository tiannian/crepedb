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
    type Range<'a>: Range<E>
    where
        Self: 'a;

    fn get(&self, table: &str, key: &[u8]) -> Result<Option<Bytes>, E>;

    fn range(&self, table: &str, begin: &[u8], end: &[u8]) -> Result<Self::Range<'_>, E>;
}

pub trait WriteTxn<E>: ReadTxn<E> {
    fn set(&self, table: &str, key: &[u8], value: &[u8]) -> Result<(), E>;

    fn del(&self, table: &str, key: &[u8]) -> Result<(), E>;

    fn commit(self) -> Result<(), E>;
}

pub trait Range<E> {
    fn next(&mut self) -> Result<Option<(Bytes, Bytes)>, E>;
}

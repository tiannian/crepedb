use core::fmt::{Debug, Display};

use alloc::vec::Vec;

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

    fn get(&self, table: &str, key: &[u8]) -> Result<Option<Vec<u8>>, E>;

    fn range(&self, table: &str) -> Result<Self::Range<'_>, E>;
}

pub trait WriteTxn<E>: ReadTxn<E> {
    fn set(&self, table: &str, key: &[u8]) -> Result<Vec<u8>, E>;

    fn commit(self) -> Result<(), E>;
}

pub trait Range<E> {
    fn next(&self) -> Result<(Vec<u8>, Vec<u8>), E>;
}

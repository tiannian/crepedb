use core::fmt::Display;

pub trait Backend<K, V>: Sized + 'static {
    type Error: BackendError;

    type ReadTxn<'a>: ReadTxn<K, V, Self::Error>;

    type WriteTxn<'a>: WriteTxn<K, V, Self::Error>;

    fn open_db(path: &str) -> Result<Self, Self::Error>;

    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error>;

    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error>;
}

pub trait BackendError: Display {}

pub trait ReadTxn<K, V, E> {
    type Range<'a>: Range<V, E>
    where
        Self: 'a;

    type Table<'a>
    where
        Self: 'a;

    fn open_table(&self) -> Result<Self::Table<'_>, E>;

    fn get(&self, table: Self::Table<'_>, key: K) -> Result<V, E>;

    fn range(&self) -> Result<Self::Range<'_>, E>;
}

pub trait WriteTxn<K, V, E> {
    type Table<'a>
    where
        Self: 'a;

    fn open_table(&self) -> Result<Self::Table<'_>, E>;

    fn set(&self, table: Self::Table<'_>, key: K) -> Result<V, E>;

    fn commit(self) -> Result<(), E>;
}

pub trait Range<V, E> {
    fn next(&self) -> Result<V, E>;
}

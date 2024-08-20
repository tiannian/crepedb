use std::path::Path;

use crepedb::backend::Backend;
use redb::{backends::InMemoryBackend, Builder, Database, Error};

use crate::{RedbReadTxn, RedbWriteTxn};

pub struct RedbDatabase {
    inner: Database,
}

impl RedbDatabase {
    pub fn open_or_create(path: impl AsRef<Path>) -> Result<Self, Error> {
        let db = Database::create(path)?;
        Ok(Self { inner: db })
    }

    pub fn memory() -> Result<Self, Error> {
        let backend = InMemoryBackend::new();

        let db = Builder::new().create_with_backend(backend)?;
        Ok(Self { inner: db })
    }
}

impl Backend for RedbDatabase {
    type Error = Error;

    type ReadTxn<'a> = RedbReadTxn<'a>;

    type WriteTxn<'a> = RedbWriteTxn<'a>;

    fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error> {
        let txn = self.inner.begin_read()?;

        Ok(RedbReadTxn { inner: txn })
    }

    fn write_txn(&self) -> Result<Self::WriteTxn<'_>, Self::Error> {
        let txn = self.inner.begin_write()?;

        Ok(RedbWriteTxn { inner: txn })
    }
}

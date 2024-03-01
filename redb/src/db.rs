use crepedb::backend::Backend;
use redb::{Database, Error};

use crate::{RedbReadTxn, RedbWriteTxn};

pub struct RedbDatabase {
    inner: Database,
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

    fn open_db(path: &str) -> Result<Self, Self::Error> {
        let db = Database::open(path)?;
        Ok(Self { inner: db })
    }
}

use crepedb::backend::Backend;
use redb::Database;

pub struct RedbDatabase {
    inner: Database,
}

// impl Backend for RedbDatabase {
//     fn read_txn(&self) -> Result<Self::ReadTxn<'_>, Self::Error> {}
// }

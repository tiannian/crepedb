use core::marker::PhantomData;

use crate::{
    backend::{BackendError, ReadTxn as BackendReadTxn},
    utils, Error, ReadTable, Result, SnapshotId,
};

pub struct ReadTxn<T, E> {
    pub(crate) txn: T,

    pub(crate) snapshot_id: SnapshotId,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> ReadTxn<T, E>
where
    T: BackendReadTxn<E>,
    E: BackendError,
{
    pub fn open_table(&self, table: &str) -> Result<ReadTable<T::Table<'_>, E>> {
        let meta = utils::meta_reader(&self.txn)?;
        let table_type = meta.read_type(table)?;

        let table = self.txn.open_table(table).map_err(Error::backend)?;

        let index = utils::index_reader(&self.txn)?;

        let sr = utils::snapshot_reader(&self.txn)?;
        let (version, _) = sr.read(&self.snapshot_id)?;

        let table = ReadTable {
            table,
            index,
            table_type,
            snapshot_id: self.snapshot_id.clone(),
            version,
            marker: PhantomData,
        };

        Ok(table)
    }
}

#[cfg(feature = "tests")]
pub mod read_tests {
    use std::{fs, path::Path};

    use alloc::vec;

    use crate::{backend::Backend, CrepeDB, Result, SnapshotId, TableType};

    pub fn test_read<B>() -> Result<()>
    where
        B: Backend,
    {
        let path = Path::new("/tmp/__crepedb");

        fs::create_dir_all(path).unwrap();

        let db: CrepeDB<B> = CrepeDB::open("/tmp/__crepedb/test_read")?;

        let table = "test";
        let key = vec![2];

        // Create root
        let rtxn = db.write(SnapshotId::preroot())?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Create s1 on root
        let s1 = db.write(root)?;
        {
            let mut t = s1.open_table(table)?;
            t.set(key.clone(), vec![1])?;
        }
        let s1 = s1.commit()?;

        {
            let rs1 = db.read(s1)?;
            let t = rs1.open_table(table)?;
            let r = t.get(key.clone())?;
            assert_eq!(r, Some(vec![1]));
        }

        drop(db);
        fs::remove_file("/tmp/__crepedb/test_read").unwrap();

        Ok(())
    }
}

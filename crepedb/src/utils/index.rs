use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::{
    backend::{BackendError, ReadTable, ReadTxn, WriteTable, WriteTxn},
    utils::fast_ceil_log2,
    Error, Result, SnapshotId,
};

use super::consts;

pub struct IndexTable<T, E> {
    table: T,
    marker: PhantomData<E>,
}

pub fn index_reader<T, E>(txn: &T) -> Result<IndexTable<T::Table<'_>, E>>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let table = txn
        .open_table(consts::SNAPSHOT_INDEX_TABLE)
        .map_err(Error::backend)?;
    Ok(IndexTable {
        table,
        marker: PhantomData,
    })
}

pub fn index_writer<T, E>(txn: &T) -> Result<IndexTable<T::Table<'_>, E>>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let table = txn
        .open_table(consts::SNAPSHOT_INDEX_TABLE)
        .map_err(Error::backend)?;
    Ok(IndexTable {
        table,
        marker: PhantomData,
    })
}

impl<T, E> IndexTable<T, E>
where
    T: ReadTable<E>,
    E: BackendError,
{
    pub fn read(&self, snapshot: &SnapshotId, n: u32) -> Result<Option<SnapshotId>> {
        let mut key = Vec::with_capacity(12);

        key.extend_from_slice(&snapshot.to_bytes());
        key.extend_from_slice(&n.to_le_bytes());

        let bytes = self.table.get(key).map_err(Error::backend)?;

        if let Some(bytes) = bytes {
            let s = SnapshotId::from_bytes(&bytes)?;
            Ok(Some(s))
        } else {
            Ok(None)
        }
    }
}

impl<T, E> IndexTable<T, E>
where
    T: WriteTable<E>,
    E: BackendError,
{
    fn write_index(&mut self, snapshot: &SnapshotId, n: u32, to: &SnapshotId) -> Result<()> {
        let mut key = Vec::with_capacity(12);

        key.extend_from_slice(&snapshot.to_bytes());
        key.extend_from_slice(&n.to_le_bytes());

        self.table
            .set(key, to.to_bytes().to_vec())
            .map_err(Error::backend)?;

        Ok(())
    }

    pub fn write(&mut self, snapshot: &SnapshotId, k1: &SnapshotId, version: u64) -> Result<()> {
        debug_assert!(version >= 1);

        let step = fast_ceil_log2(version);

        if step == 0 {
            return Ok(());
        }

        log::debug!("Verion {version}, Total index num is: {step}");

        // Inser kn, n > 1
        for i in 1..step {
            if i == 1 {
                self.write_index(snapshot, 1, k1)?;
                log::debug!("Insert version {version}, index {i} is: {k1:?}");
            } else {
                let ii = i - 1;

                let ki_1 = self
                    .read(snapshot, ii)?
                    .ok_or(Error::FatelMissingInnerIndex)?;
                log::debug!("Get `i(V{version}, {ii}) = {ki_1:?}`");

                log::debug!("Get `i(i(V{version}, {ii}), {ii})`");
                let ki_1 = self.read(&ki_1, ii)?;

                if let Some(ki_1) = ki_1 {
                    self.write_index(snapshot, i, &ki_1)?;
                    log::debug!("Insert version {version}, index {i} is: {ki_1:?}");
                } else {
                    log::debug!("Missing!");
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "tests")]
pub mod tests {
    use std::{fs, path::Path};

    use crate::{
        backend::{Backend, BackendError, ReadTxn as BackendReadTxn},
        utils::snapshot_reader,
        CrepeDB, ReadTxn, Result, SnapshotId,
    };

    use super::index_reader;

    pub fn check_index_10<T, E>(txn: ReadTxn<T, E>) -> Result<()>
    where
        T: BackendReadTxn<E>,
        E: BackendError,
    {
        let idx = index_reader(&txn.txn)?;
        let snp = snapshot_reader(&txn.txn)?;

        let snapshot = txn.snapshot_id.clone();

        macro_rules! check_inner {
            (
                $p:expr,
                $bi:literal,
                $ei:literal => None,
                $( $i:literal => $v:literal),*
            ) => {
                {
                    let (v, n) = snp.read(&$p)?;
                    assert_eq!(v, $bi);
                    $(

                        {
                            let s = idx.read(&$p, $i)?.unwrap();
                            let (v, _) = snp.read(&s)?;
                            assert_eq!(v, $v);
                        }
                    )*

                    let sp = idx.read(&$p, $ei)?;
                    assert_eq!(sp, None);
                    n
                }
            };

            (
                $p:expr,
                $bi:literal,
                $ei:literal => $ev:literal,
                $( $i:literal => $v:literal),*
            ) => {
                {
                    let (v, n) = snp.read(&$p)?;
                    assert_eq!(v, $bi);

                    $(

                        {
                            let s = idx.read(&$p, $i)?.unwrap();
                            let (v, _) = snp.read(&s)?;
                            assert_eq!(v, $v);
                        }
                    )*

                    let s = idx.read(&$p, $ei)?.unwrap();
                    let (v, _) = snp.read(&s)?;
                    assert_eq!(v, $ev);
                    n
                }
            };

            (
                $p:expr,
                $bi:literal
            ) => {
                {
                    let (v, n) = snp.read(&$p)?;
                    assert_eq!(v, $bi);

                    n
                }
            }

        }

        let ppp = check_inner!(snapshot, 11, 4 => None, 1 => 9, 2 => 7, 3 => 3);
        let ppp = check_inner!(ppp,      10, 4 => None, 1 => 8, 2 => 6, 3 => 2);
        let ppp = check_inner!(ppp,      9,  4 => None, 1 => 7, 2 => 5, 3 => 1);
        let ppp = check_inner!(ppp,      8,  3 => 0,    1 => 6, 2 => 4);
        let ppp = check_inner!(ppp,      7,  3 => None, 1 => 5, 2 => 3);
        let ppp = check_inner!(ppp,      6,  3 => None, 1 => 4, 2 => 2);
        let ppp = check_inner!(ppp,      5,  3 => None, 1 => 3, 2 => 1);
        let ppp = check_inner!(ppp,      4,  2 => 0,    1 => 2);
        let ppp = check_inner!(ppp,      3,  2 => None, 1 => 1);
        let ppp = check_inner!(ppp,      2,  1 => 0,);
        let ppp = check_inner!(ppp, 1);
        check_inner!(ppp, 0);

        Ok(())
    }

    pub fn test_db_10<B>() -> Result<()>
    where
        B: Backend,
    {
        let path = Path::new("/tmp/__crepedb");

        fs::create_dir_all(path).unwrap();

        let db: CrepeDB<B> = CrepeDB::open("/tmp/__crepedb/snapshot_10")?;

        let sid = SnapshotId::preroot();

        let write_txn = db.write(sid)?;
        log::info!("{:?}", write_txn);

        write_txn.commit()?;

        let mut sid = SnapshotId::preroot();

        for _ in 1..13 {
            log::trace!("SnapshotId is :{:?}", sid);

            let write_txn = db.write(sid)?;
            log::info!("{:?}", write_txn);

            let nsid = write_txn.commit()?;

            sid = nsid;
        }

        let txn = db.read(sid)?;

        check_index_10(txn)?;

        fs::remove_file("/tmp/__crepedb/snapshot_10").unwrap();

        Ok(())
    }
}

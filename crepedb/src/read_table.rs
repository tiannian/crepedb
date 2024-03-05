use core::marker::PhantomData;

use crate::{
    backend::{BackendError, Range, ReadTable as BackendReadTable},
    utils::{self, IndexTable},
    Bytes, DataOp, Error, Result, SnapshotId, TableType,
};

pub struct ReadTable<T, E> {
    pub(crate) table: T,

    pub(crate) index: IndexTable<T, E>,

    pub(crate) table_type: TableType,

    pub(crate) snapshot_id: SnapshotId,
    pub(crate) version: u64,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> ReadTable<T, E>
where
    T: BackendReadTable<E>,
    E: BackendError,
{
    pub fn get(&self, key: Bytes) -> Result<Option<Bytes>> {
        match self.table_type {
            TableType::Basic => self.get_basic(key),
            TableType::Versioned => self.get_versioned(key),
        }
    }

    fn get_basic(&self, key: Bytes) -> Result<Option<Bytes>> {
        let res = self.table.get(key).map_err(Error::backend)?;

        Ok(res)
    }

    fn get_versioned(&self, key: Bytes) -> Result<Option<Bytes>> {
        let key_len = key.len();

        let mut begin = key.clone();
        let mut end = key;

        begin.extend_from_slice(&0u64.to_le_bytes());
        begin.extend_from_slice(&SnapshotId::preroot().to_bytes());

        end.extend_from_slice(&u64::MAX.to_le_bytes());
        end.extend_from_slice(&SnapshotId::end().to_bytes());

        let mut iter = self.table.range(begin, end).map_err(Error::backend)?;

        while let Some((k, v)) = iter.back().map_err(Error::backend)? {
            let version = utils::parse_u64(&k[key_len..key_len + 8])?;
            let sss = SnapshotId::from_bytes(&k[key_len + 8..key_len + 16])?;

            if version > self.version {
                continue;
            }

            let mut target_version = self.version;
            let mut snapshot = self.snapshot_id.clone();

            while target_version > version {
                let diff = target_version - version;

                let skip_i = diff.ilog2();
                let skip = 1 << skip_i;

                if let Some(snapshot_id) = self.index.read(&snapshot, skip_i)? {
                    snapshot = snapshot_id;
                } else {
                    // TODO: Consider return error or panic
                    log::warn!("Index is wrong");
                    return Ok(None);
                }

                target_version -= skip;
            }

            if sss == snapshot && version == target_version {
                let res = DataOp::from_bytes(v)?;

                return Ok(res.into());
            } else {
                return Ok(None);
            }
        }

        Ok(None)
    }
}

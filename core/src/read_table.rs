use core::marker::PhantomData;

use crate::{
    backend::{BackendError, Range, ReadTable as BackendReadTable},
    types::{Bytes, DataOp, SnapshotId, TableType, Version},
    utils::{IndexTable, SnapshotTable},
    Error, Result,
};

/// A read-only view of a table at a specific snapshot.
///
/// Provides methods to query data from the table. For versioned tables,
/// the view represents the state at the snapshot's version.
pub struct ReadTable<T, E> {
    pub(crate) table: T,

    pub(crate) index: IndexTable<T, E>,
    pub(crate) snapshot: SnapshotTable<T, E>,

    pub(crate) table_type: TableType,

    pub(crate) snapshot_id: SnapshotId,
    pub(crate) version: Version,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> ReadTable<T, E>
where
    T: BackendReadTable<E>,
    E: BackendError,
{
    /// Get the value associated with a key.
    ///
    /// For versioned tables, this returns the value at this table's snapshot version.
    /// For basic tables, this returns the current value.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// * `Ok(Some(value))` if the key exists
    /// * `Ok(None)` if the key does not exist or was deleted
    /// * `Err(...)` if an error occurs
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
        begin.extend_from_slice(&SnapshotId::root().to_bytes());

        end.extend_from_slice(&u64::MAX.to_le_bytes());
        end.extend_from_slice(&SnapshotId::preroot().to_bytes());

        let mut iter = self.table.range(begin, end).map_err(Error::backend)?;

        while let Some((k, v)) = iter.back().map_err(Error::backend)? {
            let version = Version::from_bytes(&k[key_len..key_len + 8])?;
            let sss = SnapshotId::from_bytes(&k[key_len + 8..key_len + 16])?;

            log::trace!("version: {version}, snapshot: {sss:?}, value: {v:?}");

            if version > self.version {
                continue;
            }

            let mut target_version = self.version.0;
            let mut snapshot = self.snapshot_id.clone();

            while target_version > version.0 {
                let diff = target_version - version.0;

                if diff == 1 {
                    let (_, s) = self.snapshot.read(&snapshot)?;
                    snapshot = s;
                    target_version = version.0;
                    break;
                }

                let skip_i = diff.ilog2();
                let skip = 1 << skip_i;

                log::trace!("Read snapshot: {snapshot:?}, target_version: {target_version}, version: {version}, skip_n: {skip_i}, skip numer is: {skip}");

                if let Some(snapshot_id) = self.index.read(&snapshot, skip_i)? {
                    snapshot = snapshot_id;
                } else {
                    break;
                }

                target_version -= skip;
            }

            if sss == snapshot && version.0 == target_version {
                log::trace!(
                    "The snapshot: {sss:?} is ancestor of snapshot: {:?}",
                    self.snapshot_id
                );

                let res = DataOp::from_bytes(v)?;

                return Ok(res.into());
            }
        }

        Ok(None)
    }
}

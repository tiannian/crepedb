use alloc::vec::Vec;
use crepedb_core::{
    backend::{Backend, WriteTxn},
    SnapshotId,
};

use crate::{utils, DataOp, Error, Result};

pub struct VersionedWriteTxn<'a, B>
where
    B: Backend,
{
    txn: B::WriteTxn<'a>,

    version: u64,
    fork_id: u64,
    snapshot_id: SnapshotId,

    is_new_fork: bool,

    forks_bytes: Vec<u8>,
}

impl<'a, B> VersionedWriteTxn<'a, B>
where
    B: Backend,
{
    pub(crate) fn new(backend: &'a B, from: SnapshotId, to: SnapshotId) -> Result<Self> {
        use crepedb_core::backend::ReadTxn;

        let txn = backend.write_txn().map_err(Error::backend)?;

        let version_data = txn
            .get(utils::SNAPSHOT_TABLE, from.as_ref())
            .map_err(Error::backend)?
            .ok_or(Error::MissingKey)?;
        let version = utils::parse_u64(&version_data)?;

        log::debug!("Current snapshot {from} has Version {version}");

        let forks_bytes = txn
            .get(utils::SNAPSHOT_FORK_TABLE, from.as_ref())
            .map_err(Error::backend)?
            .ok_or(Error::MissingKey)?;

        let forks_count = utils::parse_u32(&forks_bytes)?;

        let (fork_id, is_new_fork) = match forks_count {
            1 => {
                let fork_id_bytes = &forks_bytes[4..];

                let leaf_id_bytes = txn
                    .get(utils::FORK_TABLE, fork_id_bytes)
                    .map_err(Error::backend)?
                    .ok_or(Error::MissingKey)?;
                let leaf_snapshot_id = utils::parse_bytes32(&leaf_id_bytes)?;

                let fork_id = utils::parse_u64(fork_id_bytes)?;

                if leaf_snapshot_id == from.as_ref() {
                    log::debug!("Snapshot {from} is at the end of fork {fork_id}");

                    (fork_id, false)
                } else {
                    log::debug!("Snapshot {from} isnt at the end of fork {fork_id}, need select new fork id");

                    (utils::read_next_fork_id(&txn)?, true)
                }
            }
            2.. => (utils::read_next_fork_id(&txn)?, true),
            _ => {
                panic!("Wrong fork number")
            }
        };

        log::debug!("Snapshot {to} at fork {fork_id}");

        Ok(Self {
            txn,
            version: version + 1,
            fork_id,
            snapshot_id: to,
            forks_bytes,
            is_new_fork,
        })
    }

    pub fn set(&self, table: &str, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Set(value).to_bytes();

        self.txn.set(table, &key, &value).map_err(Error::backend)?;

        Ok(())
    }

    pub fn remove(&self, table: &str, key: Vec<u8>) -> Result<()> {
        let key = self.build_key(key);
        let value = DataOp::Del.to_bytes();

        self.txn.set(table, &key, &value).map_err(Error::backend)?;

        Ok(())
    }

    fn build_key(&self, key: Vec<u8>) -> Vec<u8> {
        utils::build_key(key, self.version, &self.snapshot_id)
    }

    pub fn commit(self) -> Result<()> {
        if self.is_new_fork {
            let mut forks_bytes = self.forks_bytes;

            forks_bytes.extend_from_slice(&self.fork_id.to_le_bytes());

            self.txn
                .set(
                    utils::SNAPSHOT_FORK_TABLE,
                    self.snapshot_id.as_ref(),
                    &forks_bytes,
                )
                .map_err(Error::backend)?;
        }

        // Store version of current snapshot
        self.txn
            .set(
                utils::SNAPSHOT_TABLE,
                self.snapshot_id.as_ref(),
                &self.version.to_le_bytes(),
            )
            .map_err(Error::backend)?;

        // Store leaf_snapshot of this fork
        self.txn
            .set(
                utils::FORK_TABLE,
                &self.fork_id.to_le_bytes(),
                self.snapshot_id.as_ref(),
            )
            .map_err(Error::backend)?;

        // Store next fork id
        self.txn
            .set(
                utils::FORK_TABLE,
                utils::NEXT_FORK_ID_KEY,
                &(self.fork_id + 1).to_le_bytes(),
            )
            .map_err(Error::backend)?;

        self.txn.commit().map_err(Error::backend)?;

        Ok(())
    }
}

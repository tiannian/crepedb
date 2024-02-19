use alloc::vec::Vec;
use crepedb_core::{backend::Backend, SnapshotId};

pub struct VersionedReadTxn<'a, K, V, B>
where
    B: Backend<K, V>,
{
    pub(crate) kv: B::ReadTxn<'a>,

    pub(crate) fork_nums: Vec<u64>,
    pub(crate) snapshot_num: u64,
    pub(crate) snapshot: SnapshotId,
}

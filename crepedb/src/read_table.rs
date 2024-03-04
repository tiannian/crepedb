use core::marker::PhantomData;

use crate::{
    utils::{IndexTable, MetaTable},
    SnapshotId,
};

pub struct ReadTable<T, E> {
    pub(crate) table: T,
    pub(crate) meta: MetaTable<T, E>,
    pub(crate) index: IndexTable<T, E>,

    pub(crate) snapshot_id: SnapshotId,
    pub(crate) version: u64,

    pub(crate) marker: PhantomData<E>,
}

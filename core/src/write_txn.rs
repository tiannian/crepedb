use core::{fmt::Debug, marker::PhantomData};

use crate::{
    backend::{BackendError, WriteTxn as BackendWriteTxn},
    types::{SnapshotId, TableType, Version},
    utils, Error, Result, WriteTable,
};

/// A write transaction for modifying data and creating new snapshots.
///
/// Write transactions allow modifications to tables and create a new snapshot
/// when committed. They maintain version history and parent-child relationships
/// between snapshots.
pub struct WriteTxn<T, E> {
    pub(crate) txn: T,

    /// The parent snapshot ID. None if this is the root snapshot.
    pub(crate) parent_snapshot_id: Option<SnapshotId>,

    /// The snapshot ID being branched from.
    pub(crate) snapshot_id: SnapshotId,

    /// The new snapshot ID that will be created on commit.
    pub(crate) new_snapshot_id: SnapshotId,

    /// The version number for this transaction.
    pub(crate) version: Version,

    pub(crate) marker: PhantomData<E>,
}

impl<T, E> Debug for WriteTxn<T, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{{ parent_snapshot_id: {:?}, ",
            self.parent_snapshot_id
        ))?;
        f.write_fmt(format_args!("snapshot_id: {:?}, ", self.snapshot_id))?;
        f.write_fmt(format_args!(
            "new_snapshot_id: {:?}, ",
            self.new_snapshot_id
        ))?;
        f.write_fmt(format_args!("version: {} }}", self.version))
    }
}

impl<T, E> WriteTxn<T, E>
where
    T: BackendWriteTxn<E>,
    E: BackendError,
{
    /// Create a new basic (non-versioned) table.
    ///
    /// Basic tables store data directly with no version tracking. Updates overwrite
    /// previous values. This is more efficient for data that doesn't need version history.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to create
    ///
    /// # Errors
    ///
    /// Returns an error if the table already exists or cannot be created.
    pub fn create_basic_table(&self, table: &str) -> Result<()> {
        let mut meta = utils::meta_writer(&self.txn)?;
        meta.write_type(table, &TableType::Basic)?;
        Ok(())
    }

    /// Create a new versioned table with full history tracking.
    ///
    /// Versioned tables track all changes across snapshots. Each write creates a new
    /// version entry. Reads can retrieve data from any snapshot in the version history.
    /// This enables time-travel queries and branching.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to create
    ///
    /// # Errors
    ///
    /// Returns an error if the table already exists or cannot be created.
    pub fn create_versioned_table(&self, table: &str) -> Result<()> {
        let mut meta = utils::meta_writer(&self.txn)?;
        meta.write_type(table, &TableType::Versioned)?;
        Ok(())
    }

    /// Open a table for writing.
    ///
    /// Returns a writable view of the table in this transaction.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to open
    ///
    /// # Errors
    ///
    /// Returns an error if the table does not exist or cannot be opened.
    pub fn open_table(&self, table: &str) -> Result<WriteTable<T::Table<'_>, E>> {
        let meta = utils::meta_reader_by_write(&self.txn)?;

        let table_type = meta.read_type(table)?;

        let table = WriteTable {
            marker: PhantomData,
            table_type,
            snapshot_id: self.new_snapshot_id.clone(),
            table: self.txn.open_table(table).map_err(Error::backend)?,
            version: self.version.clone(),
        };

        Ok(table)
    }

    /// Commit the write transaction.
    ///
    /// This persists all changes and creates a new snapshot. The snapshot ID
    /// of the new snapshot is returned.
    ///
    /// # Returns
    ///
    /// The snapshot ID of the newly created snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error if the commit fails.
    pub fn commit(self) -> Result<SnapshotId> {
        {
            let mut snapshot = utils::snapshot_writer(&self.txn)?;

            // write snapshot info
            snapshot.write(&self.new_snapshot_id, &self.snapshot_id, &self.version)?;

            // write next snapshot id
            snapshot.write_next_snapahot(&self.new_snapshot_id)?;
        }

        if let Some(parent_snapshot_id) = self.parent_snapshot_id {
            // Must not be root
            // build index
            let mut index = utils::index_writer(&self.txn)?;

            index.write(&self.new_snapshot_id, &parent_snapshot_id, self.version)?;
        }

        let new_snapshot_id = self.new_snapshot_id;

        self.txn.commit().map_err(Error::backend)?;

        Ok(new_snapshot_id)
    }
}

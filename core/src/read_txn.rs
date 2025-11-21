use core::marker::PhantomData;

use crate::{
    backend::{BackendError, ReadTxn as BackendReadTxn},
    types::SnapshotId,
    utils, Error, ReadTable, Result,
};

/// A read transaction for querying data at a specific snapshot.
///
/// Read transactions provide a consistent view of the database at a particular
/// snapshot in time. Multiple read transactions can run concurrently.
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
    /// Open a table for reading.
    ///
    /// Returns a read-only view of the table at this transaction's snapshot.
    ///
    /// # Arguments
    ///
    /// * `table` - The name of the table to open
    ///
    /// # Errors
    ///
    /// Returns an error if the table does not exist or cannot be opened.
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
            snapshot: sr,
            table_type,
            snapshot_id: self.snapshot_id.clone(),
            version,
            marker: PhantomData,
        };

        Ok(table)
    }
}

#[doc(hidden)]
#[cfg(feature = "tests")]
pub mod read_tests {
    use alloc::vec;

    use crate::{backend::Backend, types::TableType, CrepeDB, Result};

    pub fn test_read(backend: impl Backend) -> Result<()> {
        // let db: CrepeDB<B> = CrepeDB::open("/tmp/__crepedb/test_read")?;
        let db = CrepeDB::new(backend);

        let table = "test";
        let key = vec![2];

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Create s1 on root
        let s1 = db.write(Some(root.clone()))?;
        {
            let mut t = s1.open_table(table)?;
            t.set(key.clone(), vec![1])?;
        }
        let s1 = s1.commit()?;

        // Try to read on s1
        {
            let rs1 = db.read(Some(s1))?;
            let t = rs1.open_table(table)?;
            let r = t.get(key.clone())?;
            assert_eq!(r, Some(vec![1]));
        }

        // Create s1 on root
        let s1 = db.write(Some(root))?;
        {
            let mut t = s1.open_table(table)?;
            t.set(key.clone(), vec![2])?;
        }
        let s2 = s1.commit()?;

        let s2 = {
            let s = db.write(Some(s2))?;
            s.commit()?
        };

        let s2 = {
            let s = db.write(Some(s2))?;
            s.commit()?
        };

        let s2 = {
            let s = db.write(Some(s2))?;
            s.commit()?
        };

        let s2 = {
            let s = db.write(Some(s2))?;
            s.commit()?
        };

        let s2 = {
            let s = db.write(Some(s2))?;
            s.commit()?
        };

        // Try to read on s2
        {
            let rs1 = db.read(Some(s2.clone()))?;
            let t = rs1.open_table(table)?;
            let r = t.get(key.clone())?;
            assert_eq!(r, Some(vec![2]));
        }

        // Try to read on s2
        {
            let rs1 = db.read(Some(s2.clone()))?;
            let t = rs1.open_table(table)?;
            let r = t.get(vec![100])?;
            assert_eq!(r, None);
        }

        drop(db);

        Ok(())
    }

    /// Test snapshot isolation: multiple branches from the same parent
    pub fn test_snapshot_isolation(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_isolation";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Create branch 1 from root
        let branch1_txn = db.write(Some(root.clone()))?;
        {
            let mut t = branch1_txn.open_table(table)?;
            t.set(vec![1], vec![10])?;
        }
        let branch1 = branch1_txn.commit()?;

        // Create branch 2 from root with different data
        let branch2_txn = db.write(Some(root.clone()))?;
        {
            let mut t = branch2_txn.open_table(table)?;
            t.set(vec![1], vec![20])?;
        }
        let branch2 = branch2_txn.commit()?;

        // Read from branch 1 should see its own value
        {
            let rtxn = db.read(Some(branch1))?;
            let t = rtxn.open_table(table)?;
            let value = t.get(vec![1])?;
            assert_eq!(value, Some(vec![10]));
        }

        // Read from branch 2 should see its own value
        {
            let rtxn = db.read(Some(branch2))?;
            let t = rtxn.open_table(table)?;
            let value = t.get(vec![1])?;
            assert_eq!(value, Some(vec![20]));
        }

        // Read from root should see nothing
        {
            let rtxn = db.read(Some(root))?;
            let t = rtxn.open_table(table)?;
            let value = t.get(vec![1])?;
            assert_eq!(value, None);
        }

        Ok(())
    }

    /// Test multiple keys in the same snapshot
    pub fn test_multiple_keys(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_multi_keys";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Write multiple keys
        let wtxn = db.write(Some(root.clone()))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![100])?;
            t.set(vec![2], vec![200])?;
            t.set(vec![3], vec![250])?;
            t.set(vec![4], vec![255])?;
        }
        let s1 = wtxn.commit()?;

        // Read all keys
        {
            let rtxn = db.read(Some(s1))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
            assert_eq!(t.get(vec![2])?, Some(vec![200]));
            assert_eq!(t.get(vec![3])?, Some(vec![250]));
            assert_eq!(t.get(vec![4])?, Some(vec![255]));
            assert_eq!(t.get(vec![5])?, None);
        }

        // Create another branch from root with different data
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![111])?;
            t.set(vec![2], vec![222])?;
        }
        let s2 = wtxn.commit()?;

        // Verify s2 has its own data
        {
            let rtxn = db.read(Some(s2))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![111]));
            assert_eq!(t.get(vec![2])?, Some(vec![222]));
            // Keys not set in this branch should be None
            assert_eq!(t.get(vec![3])?, None);
            assert_eq!(t.get(vec![4])?, None);
        }

        Ok(())
    }

    /// Test error handling: opening non-existent table
    pub fn test_error_handling(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table("existing_table", &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Try to open non-existent table
        let rtxn = db.read(Some(root))?;
        let result = rtxn.open_table("non_existent_table");
        assert!(result.is_err());

        Ok(())
    }

    /// Test multiple tables in the same transaction
    pub fn test_multiple_tables(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);

        // Create root with multiple tables
        let rtxn = db.write(None)?;
        rtxn.create_table("table1", &TableType::Versioned)?;
        rtxn.create_table("table2", &TableType::Versioned)?;
        rtxn.create_table("table3", &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Write to different tables
        let wtxn = db.write(Some(root))?;
        {
            let mut t1 = wtxn.open_table("table1")?;
            t1.set(vec![1], vec![100])?;

            let mut t2 = wtxn.open_table("table2")?;
            t2.set(vec![1], vec![200])?;

            let mut t3 = wtxn.open_table("table3")?;
            t3.set(vec![1], vec![250])?;
        }
        let s1 = wtxn.commit()?;

        // Read from all tables and verify isolation
        {
            let rtxn = db.read(Some(s1))?;

            let t1 = rtxn.open_table("table1")?;
            assert_eq!(t1.get(vec![1])?, Some(vec![100]));

            let t2 = rtxn.open_table("table2")?;
            assert_eq!(t2.get(vec![1])?, Some(vec![200]));

            let t3 = rtxn.open_table("table3")?;
            assert_eq!(t3.get(vec![1])?, Some(vec![250]));
        }

        Ok(())
    }

    /// Test Basic table type
    pub fn test_basic_table_type(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);

        // Create root with Basic table
        let rtxn = db.write(None)?;
        rtxn.create_table("basic_table", &TableType::Basic)?;
        let root = rtxn.commit()?;

        // Write to basic table
        let wtxn = db.write(Some(root.clone()))?;
        {
            let mut t = wtxn.open_table("basic_table")?;
            t.set(vec![1], vec![100])?;
        }
        let s1 = wtxn.commit()?;

        // Read from basic table
        {
            let rtxn = db.read(Some(s1))?;
            let t = rtxn.open_table("basic_table")?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
        }

        // Basic table should show updates from any branch
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table("basic_table")?;
            t.set(vec![1], vec![200])?;
        }
        let s2 = wtxn.commit()?;

        {
            let rtxn = db.read(Some(s2))?;
            let t = rtxn.open_table("basic_table")?;
            assert_eq!(t.get(vec![1])?, Some(vec![200]));
        }

        Ok(())
    }

    /// Test edge cases: empty keys and values
    pub fn test_edge_cases(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_edges";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Test empty value
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![])?;
        }
        let s1 = wtxn.commit()?;

        {
            let rtxn = db.read(Some(s1.clone()))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![]));
        }

        // Test large key and value
        let wtxn = db.write(Some(s1))?;
        {
            let mut t = wtxn.open_table(table)?;
            let large_key = vec![255u8; 1024];
            let large_value = vec![128u8; 4096];
            t.set(large_key.clone(), large_value.clone())?;
        }
        let s2 = wtxn.commit()?;

        {
            let rtxn = db.read(Some(s2))?;
            let t = rtxn.open_table(table)?;
            let large_key = vec![255u8; 1024];
            let large_value = vec![128u8; 4096];
            assert_eq!(t.get(large_key)?, Some(large_value));
        }

        Ok(())
    }

    /// Test long version chains
    pub fn test_version_chain(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_chain";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Create a long chain of snapshots, each adding a new key
        let mut current = root;
        let chain_length = 20u8;

        for i in 0u8..chain_length {
            let wtxn = db.write(Some(current))?;
            {
                let mut t = wtxn.open_table(table)?;
                // Each snapshot writes its own key
                t.set(vec![i], vec![i])?;
            }
            current = wtxn.commit()?;
        }

        // Read from the final snapshot - should see all keys
        {
            let rtxn = db.read(Some(current))?;
            let t = rtxn.open_table(table)?;
            for i in 0u8..chain_length {
                let value = t.get(vec![i])?;
                assert_eq!(value, Some(vec![i]), "Key {} should have value {}", i, i);
            }
        }

        Ok(())
    }

    /// Test delete operations
    pub fn test_delete_operations(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_delete";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Branch 1: Write a key
        let wtxn = db.write(Some(root.clone()))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![100])?;
            t.set(vec![2], vec![200])?;
        }
        let s1 = wtxn.commit()?;

        // Verify keys exist in s1
        {
            let rtxn = db.read(Some(s1.clone()))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
            assert_eq!(t.get(vec![2])?, Some(vec![200]));
        }

        // Branch 2: From root, create a branch where key 1 doesn't exist
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table(table)?;
            // Only set key 2, key 1 doesn't exist
            t.set(vec![2], vec![222])?;
        }
        let s2 = wtxn.commit()?;

        // Verify key 1 doesn't exist in s2, but key 2 does
        {
            let rtxn = db.read(Some(s2))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, None);
            assert_eq!(t.get(vec![2])?, Some(vec![222]));
        }

        // Verify s1 still has both keys (snapshot isolation)
        {
            let rtxn = db.read(Some(s1))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
            assert_eq!(t.get(vec![2])?, Some(vec![200]));
        }

        Ok(())
    }

    /// Test reading from root snapshot
    pub fn test_root_snapshot(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_root";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Create first snapshot with no data (just to establish snapshot chain)
        let wtxn = db.write(Some(root))?;
        {
            // Open table but don't write anything
            let _t = wtxn.open_table(table)?;
        }
        let s0 = wtxn.commit()?;

        // Verify s0 is readable and empty
        {
            let rtxn = db.read(Some(s0.clone()))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, None);
        }

        // Create child snapshot with data
        let wtxn = db.write(Some(s0.clone()))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![100])?;
        }
        let s1 = wtxn.commit()?;

        // Read from child
        {
            let rtxn = db.read(Some(s1))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
        }

        // s0 should still be empty (parent doesn't see child's changes)
        {
            let rtxn = db.read(Some(s0))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, None);
        }

        Ok(())
    }

    /// Test transaction lifecycle: opening same table multiple times
    pub fn test_transaction_lifecycle(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_lifecycle";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Write data
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![100])?;
        }
        let s1 = wtxn.commit()?;

        // Open the same table multiple times in one read transaction
        {
            let rtxn = db.read(Some(s1))?;

            let t1 = rtxn.open_table(table)?;
            assert_eq!(t1.get(vec![1])?, Some(vec![100]));

            let t2 = rtxn.open_table(table)?;
            assert_eq!(t2.get(vec![1])?, Some(vec![100]));

            // Both should see the same data
            assert_eq!(t1.get(vec![1])?, t2.get(vec![1])?);
        }

        Ok(())
    }

    /// Test mixed reads and non-existent keys
    pub fn test_mixed_operations(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_mixed";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Write some keys
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table(table)?;
            for i in 0..10 {
                if i % 2 == 0 {
                    t.set(vec![i], vec![i * 10])?;
                }
            }
        }
        let s1 = wtxn.commit()?;

        // Read mixed existing and non-existing keys
        {
            let rtxn = db.read(Some(s1))?;
            let t = rtxn.open_table(table)?;

            for i in 0..10 {
                let value = t.get(vec![i])?;
                if i % 2 == 0 {
                    assert_eq!(value, Some(vec![i * 10]));
                } else {
                    assert_eq!(value, None);
                }
            }
        }

        Ok(())
    }

    /// Test parent-child snapshot relationship
    pub fn test_parent_child_visibility(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_visibility";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Parent snapshot with some data
        let wtxn = db.write(Some(root))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![1], vec![100])?;
            t.set(vec![2], vec![200])?;
        }
        let parent = wtxn.commit()?;

        // Child snapshot adds more data
        let wtxn = db.write(Some(parent.clone()))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![3], vec![250])?;
        }
        let child = wtxn.commit()?;

        // Child should see parent's data + its own
        {
            let rtxn = db.read(Some(child))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
            assert_eq!(t.get(vec![2])?, Some(vec![200]));
            assert_eq!(t.get(vec![3])?, Some(vec![250]));
        }

        // Parent should not see child's data
        {
            let rtxn = db.read(Some(parent))?;
            let t = rtxn.open_table(table)?;
            assert_eq!(t.get(vec![1])?, Some(vec![100]));
            assert_eq!(t.get(vec![2])?, Some(vec![200]));
            assert_eq!(t.get(vec![3])?, None);
        }

        Ok(())
    }

    /// Test complex branching scenario
    pub fn test_complex_branching(backend: impl Backend) -> Result<()> {
        let db = CrepeDB::new(backend);
        let table = "test_branching";

        // Create root
        let rtxn = db.write(None)?;
        rtxn.create_table(table, &TableType::Versioned)?;
        let root = rtxn.commit()?;

        // Base snapshot
        let wtxn = db.write(Some(root.clone()))?;
        {
            let mut t = wtxn.open_table(table)?;
            t.set(vec![0], vec![0])?;
        }
        let base = wtxn.commit()?;

        // Create 3 branches from base
        let mut branches = alloc::vec::Vec::new();
        for i in 1u8..=3u8 {
            let wtxn = db.write(Some(base.clone()))?;
            {
                let mut t = wtxn.open_table(table)?;
                t.set(vec![i], vec![i * 10])?;
            }
            branches.push((wtxn.commit()?, i));
        }

        // Each branch should see base data + its own
        for (branch, i) in branches {
            let rtxn = db.read(Some(branch))?;
            let t = rtxn.open_table(table)?;

            // Should see base data
            assert_eq!(t.get(vec![0])?, Some(vec![0]));

            // Should see its own data
            assert_eq!(t.get(vec![i])?, Some(vec![i * 10]));

            // Should not see other branches' data
            for j in 1u8..=3u8 {
                if j != i {
                    assert_eq!(t.get(vec![j])?, None);
                }
            }
        }

        Ok(())
    }
}

use std::sync::Arc;

use crepedb_core::backend::ReadTxn;
use rocksdb::{Error, OptimisticTransactionDB};

use crate::RocksdbReadTable;

/// A read transaction wrapper for RocksDB.
///
/// Implements the CrepeDB `ReadTxn` trait, providing read-only access to tables.
pub struct RocksdbReadTxn {
    pub(crate) db: Arc<OptimisticTransactionDB>,
}

impl ReadTxn<Error> for RocksdbReadTxn {
    type Table<'b>
        = RocksdbReadTable
    where
        Self: 'b;

    fn open_table(&self, table: &str) -> Result<Self::Table<'_>, Error> {
        Ok(RocksdbReadTable {
            db: Arc::clone(&self.db),
            name: table.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::RocksdbDatabase;

    #[test]
    fn test_read() {
        let _ = env_logger::builder().is_test(true).try_init();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "crepedb-rocksdb-test-{}-{}",
            std::process::id(),
            timestamp
        ));
        let backend = RocksdbDatabase::open_or_create(&temp_dir).unwrap();

        let result = crepedb_core::read_tests::test_read(backend);

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);

        result.unwrap();
    }
}

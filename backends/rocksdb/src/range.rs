use std::sync::Arc;

use rocksdb::{Error, IteratorMode, OptimisticTransactionDB};

/// A range iterator wrapper for RocksDB.
///
/// Implements the CrepeDB `Range` trait for RocksDB's iterator.
pub struct RocksdbRange {
    pub(crate) db: Arc<OptimisticTransactionDB>,
    pub(crate) begin: Vec<u8>,
    pub(crate) end: Vec<u8>,
    pub(crate) prefix_len: usize,
    pub(crate) current: Option<usize>,
}

impl crepedb_core::backend::Range<Error> for RocksdbRange {
    fn back(
        &mut self,
    ) -> Result<Option<(crepedb_core::types::Bytes, crepedb_core::types::Bytes)>, Error> {
        // Initialize iterator on first call
        if self.current.is_none() {
            self.current = Some(0);
        }

        let iter = self
            .db
            .iterator(IteratorMode::From(&self.begin, rocksdb::Direction::Forward));

        // Skip to current position
        let skip_count = self.current.unwrap();
        let mut iter = iter.skip(skip_count);

        if let Some(result) = iter.next() {
            let (key, value) = result?;

            // Check if key is within range
            if key.as_ref() >= self.end.as_slice() {
                return Ok(None);
            }

            // Increment position for next call
            self.current = Some(skip_count + 1);

            // Strip the table prefix from the key
            let stripped_key = if key.len() > self.prefix_len {
                key[self.prefix_len..].to_vec()
            } else {
                return Ok(None);
            };

            Ok(Some((stripped_key, value.to_vec())))
        } else {
            Ok(None)
        }
    }
}

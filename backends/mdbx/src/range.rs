use libmdbx::{Cursor, Error, TransactionKind};
use crepedb_core::Bytes;

/// A range iterator wrapper for MDBX.
///
/// Implements the CrepeDB `Range` trait for MDBX's cursor iterator.
pub struct MdbxRange<'a, K: TransactionKind> {
    pub(crate) cursor: Cursor<'a, K>,
    pub(crate) begin: Bytes,
    pub(crate) end: Bytes,
    pub(crate) started: bool,
}

impl<'a, K: TransactionKind> crepedb_core::backend::Range<Error> for MdbxRange<'a, K> {
    fn back(&mut self) -> Result<Option<(Bytes, Bytes)>, Error> {
        let result = if !self.started {
            self.started = true;
            // Position cursor at or after begin key
            self.cursor.set_range::<Vec<u8>, Vec<u8>>(&self.begin)
        } else {
            // Move to next entry
            self.cursor.next::<Vec<u8>, Vec<u8>>()
        };

        match result {
            Ok(Some((key, value))) => {
                // Check if key is within range
                if key < self.end {
                    Ok(Some((key, value)))
                } else {
                    Ok(None)
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

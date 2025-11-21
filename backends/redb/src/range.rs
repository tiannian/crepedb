use redb::{Error, Range};

use crate::types::BytesTy;

/// A range iterator wrapper for redb.
///
/// Implements the CrepeDB `Range` trait for redb's range iterator.
pub struct RedbRange<'a> {
    pub(crate) inner: Range<'a, BytesTy, BytesTy>,
}

impl<'a> crepedb_core::backend::Range<Error> for RedbRange<'a> {
    fn back(&mut self) -> Result<Option<(crepedb_core::Bytes, crepedb_core::Bytes)>, Error> {
        let r = self.inner.next();

        if let Some(r) = r {
            let r = r?;

            let key = r.0.value();
            let value = r.1.value();

            Ok(Some((key, value)))
        } else {
            Ok(None)
        }
    }
}

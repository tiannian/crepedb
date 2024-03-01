use redb::{Error, Range};

use crate::types::BytesTy;

pub struct RedbRange<'a> {
    pub(crate) inner: Range<'a, BytesTy, BytesTy>,
}

impl<'a> crepedb::backend::Range<Error> for RedbRange<'a> {
    fn back(&mut self) -> Result<Option<(crepedb::Bytes, crepedb::Bytes)>, Error> {
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

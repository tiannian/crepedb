use redb::Range;

pub struct RedbRange<'a> {
    inner: Range<'a, Vec<u8>, Vec<u8>>,
}

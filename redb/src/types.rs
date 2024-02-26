use crepedb::Bytes;
use redb::{RedbKey, RedbValue};

#[derive(Debug)]
pub struct BytesTy(pub(crate) Bytes);

impl RedbValue for BytesTy {
    type AsBytes<'a> = <Bytes as RedbValue>::AsBytes<'a>;

    type SelfType<'a> = <Bytes as RedbValue>::SelfType<'a>;

    fn type_name() -> redb::TypeName {
        crepedb::Bytes::type_name()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        Bytes::as_bytes(value)
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        Bytes::from_bytes(data)
    }

    fn fixed_width() -> Option<usize> {
        Bytes::fixed_width()
    }
}

impl RedbKey for BytesTy {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}

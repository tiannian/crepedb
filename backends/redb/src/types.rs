//! Type definitions for redb integration.
//!
//! This module provides types that bridge CrepeDB's byte arrays with redb's
//! type system.

use crepedb_core::Bytes;
use redb::{Key, Value};

/// A type adapter for using CrepeDB's `Bytes` type with redb.
///
/// This struct implements redb's `Key` and `Value` traits, allowing
/// `Vec<u8>` to be used as both keys and values in redb tables.
#[derive(Debug)]
pub struct BytesTy;

impl Value for BytesTy {
    type AsBytes<'a> = <Bytes as Value>::AsBytes<'a>;

    type SelfType<'a> = <Bytes as Value>::SelfType<'a>;

    fn type_name() -> redb::TypeName {
        crepedb_core::Bytes::type_name()
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

impl Key for BytesTy {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}

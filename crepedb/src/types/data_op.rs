use alloc::vec;

use crate::{Bytes, Error, Result};

/// Operation of data.
pub enum DataOp {
    /// Set data
    Set(Bytes),
    /// Delete data
    Del,
}

impl From<DataOp> for Option<Bytes> {
    fn from(value: DataOp) -> Self {
        match value {
            DataOp::Set(v) => Some(v),
            DataOp::Del => None,
        }
    }
}

impl DataOp {
    /// Convert this type to bytes.
    pub fn to_bytes(self) -> Bytes {
        match self {
            Self::Set(mut v) => {
                v.push(0x00);
                v
            }
            Self::Del => vec![0x01],
        }
    }

    /// Convert type from bytes
    pub fn from_bytes(bytes: Bytes) -> Result<DataOp> {
        let mut bytes = bytes;

        let flag = bytes.pop().ok_or(Error::MissingDataOpFlag)?;

        match flag {
            0x00 => Ok(DataOp::Set(bytes)),
            0x01 => Ok(DataOp::Del),
            _ => Err(Error::UnexpectedDataOpType(flag)),
        }
    }
}

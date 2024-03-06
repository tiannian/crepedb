use alloc::{vec, vec::Vec};

use crate::{Bytes, Error, Result};

pub enum DataOp {
    Set(Vec<u8>),
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
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Self::Set(mut v) => {
                v.push(0x00);
                v
            }
            Self::Del => vec![0x01],
        }
    }

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

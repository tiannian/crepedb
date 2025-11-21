use alloc::{vec, vec::Vec};

use crate::{types::Bytes, Error, Result};

/// Represents a data operation in versioned tables.
///
/// Versioned tables store operations rather than just values, allowing
/// them to distinguish between "value is set" and "value is deleted".
pub enum DataOp {
    /// Set a value for a key.
    Set(Vec<u8>),

    /// Delete a key.
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
    /// Serialize the data operation to bytes.
    ///
    /// The operation type is encoded as a flag byte appended to the end.
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Self::Set(mut v) => {
                v.push(0x00);
                v
            }
            Self::Del => vec![0x01],
        }
    }

    /// Deserialize a data operation from bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The bytes are empty (missing flag)
    /// - The flag byte is not a valid operation type
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

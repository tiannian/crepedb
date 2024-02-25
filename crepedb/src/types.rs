use alloc::{vec, vec::Vec};

use crate::{utils, Error, Result};

pub type Bytes = Vec<u8>;

pub enum DataOp {
    Set(Vec<u8>),
    Del,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    Basic,
    Versioned,
}

impl TableType {
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Basic => 1,
            Self::Versioned => 2,
        }
    }

    pub fn from_byte(v: u8) -> Result<Self> {
        match v {
            1 => Ok(Self::Basic),
            2 => Ok(Self::Versioned),
            _ => Err(Error::UnexpectedTableType(v)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnapshotId(pub(crate) u64);

impl From<[u8; 8]> for SnapshotId {
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(value))
    }
}

impl SnapshotId {
    pub fn to_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = utils::parse_u64(bytes)?;
        Ok(Self(r))
    }

    pub const fn preroot() -> Self {
        Self(0)
    }

    pub const fn root() -> Self {
        Self(1)
    }
}

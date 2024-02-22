use alloc::{vec, vec::Vec};

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

pub enum TableType {
    Basic,
    Versioned,
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
}

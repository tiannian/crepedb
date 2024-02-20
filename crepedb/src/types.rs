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

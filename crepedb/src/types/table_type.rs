use crate::{Error, Result};

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

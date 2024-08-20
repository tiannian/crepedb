use crate::Error;

/// Type of table
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    Basic,
    Versioned,
}

impl From<TableType> for u8 {
    fn from(value: TableType) -> Self {
        match value {
            TableType::Basic => 1,
            TableType::Versioned => 2,
        }
    }
}

impl TryFrom<u8> for TableType {
    type Error = Error;

    fn try_from(value: u8) -> core::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Basic),
            2 => Ok(Self::Versioned),
            _ => Err(Error::UnexpectedTableType(value)),
        }
    }
}

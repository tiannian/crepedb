use crate::{Error, Result};

/// The type of storage strategy used by a table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableType {
    /// Basic non-versioned table.
    ///
    /// Data is stored directly with no version tracking. Updates overwrite
    /// previous values. This is more efficient for data that doesn't need
    /// version history.
    Basic,

    /// Versioned table with full history tracking.
    ///
    /// Each write creates a new version entry. Reads can retrieve data from
    /// any snapshot in the version history. This enables time-travel queries
    /// and branching.
    Versioned,
}

impl TableType {
    /// Convert the table type to its byte representation.
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Basic => 1,
            Self::Versioned => 2,
        }
    }

    /// Parse a table type from its byte representation.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte value is not a valid table type.
    pub fn from_byte(v: u8) -> Result<Self> {
        match v {
            1 => Ok(Self::Basic),
            2 => Ok(Self::Versioned),
            _ => Err(Error::UnexpectedTableType(v)),
        }
    }
}

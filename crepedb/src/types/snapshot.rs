use crate::{utils, Result};

/// Unique identifier for a database snapshot.
///
/// Snapshots represent consistent states of the database at specific points in time.
/// They form a tree structure where each snapshot (except root) has a parent.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnapshotId(pub(crate) u64);

impl From<[u8; 8]> for SnapshotId {
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(value))
    }
}

impl SnapshotId {
    /// Convert the snapshot ID to bytes.
    pub fn to_bytes(&self) -> [u8; 8] {
        utils::dump_u64(self.0)
    }

    /// Parse a snapshot ID from bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice has the wrong length.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = utils::parse_u64(bytes)?;
        Ok(Self(r))
    }

    /// Get the special "preroot" snapshot ID.
    ///
    /// This ID is used when creating the root snapshot.
    pub const fn preroot() -> Self {
        Self(u64::MAX)
    }

    /// Get the root snapshot ID.
    ///
    /// The root is the first snapshot in the database.
    pub const fn root() -> Self {
        Self(0)
    }
}

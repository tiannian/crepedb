use crate::{utils, Result};

/// Id of snapshot
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnapshotId(pub(crate) u64);

impl From<[u8; 8]> for SnapshotId {
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(value))
    }
}

impl From<SnapshotId> for [u8; 8] {
    fn from(value: SnapshotId) -> Self {
        utils::dump_u64(value.0)
    }
}

impl SnapshotId {
    /// Build snapshot id from slice of bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = utils::parse_u64(bytes)?;
        Ok(Self(r))
    }

    /// PreRoot of Snapshot
    pub const fn preroot() -> Self {
        Self(u64::MAX)
    }

    /// Root of Snapshot
    pub const fn root() -> Self {
        Self(0)
    }
}

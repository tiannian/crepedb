use crate::{utils, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SnapshotId(pub(crate) u64);

impl From<[u8; 8]> for SnapshotId {
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(value))
    }
}

impl SnapshotId {
    pub fn to_bytes(&self) -> [u8; 8] {
        utils::dump_u64(self.0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = utils::parse_u64(bytes)?;
        Ok(Self(r))
    }

    pub const fn preroot() -> Self {
        Self(u64::MAX)
    }

    pub const fn root() -> Self {
        Self(0)
    }
}

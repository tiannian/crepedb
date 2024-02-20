use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotId([u8; 32]);

impl Display for SnapshotId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&hex::encode(self.0))
    }
}

impl From<[u8; 32]> for SnapshotId {
    fn from(value: [u8; 32]) -> Self {
        Self(value)
    }
}

impl AsRef<[u8]> for SnapshotId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for SnapshotId {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsMut<[u8]> for SnapshotId {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsMut<[u8; 32]> for SnapshotId {
    fn as_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}

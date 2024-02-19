pub struct SnapshotId([u8; 32]);

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

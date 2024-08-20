use core::fmt::Display;

use crate::{utils, Result};

/// Version of snaoshot, begin from 0
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Version(pub(crate) u64);

impl From<[u8; 8]> for Version {
    fn from(value: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(value))
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Version> for [u8; 8] {
    fn from(value: Version) -> Self {
        utils::dump_u64(value.0)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Version {
    /// Build snapshot id from slice of bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = utils::parse_u64(bytes)?;
        Ok(Self(r))
    }

    /// Root version
    pub const fn root() -> Self {
        Self(0)
    }
}

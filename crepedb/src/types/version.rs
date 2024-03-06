use core::fmt::Display;

use crate::{utils, Result};

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

impl Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Version {
    pub fn to_bytes(&self) -> [u8; 8] {
        utils::dump_u64(self.0)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let r = utils::parse_u64(bytes)?;
        Ok(Self(r))
    }

    pub const fn root() -> Self {
        Self(0)
    }
}

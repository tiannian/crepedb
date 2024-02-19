use crate::{Error, Result};

/// Name of fork table
///
/// fork_number(u64) => leaf_snapshot_id(bytes32)
pub const FORK_TABLE: &str = "__crepe_fork";

/// Name of snapshot table
///
/// snapshot_id(bytes32) => snapshot_num(u63)
pub const SNAPSHOT_TABLE: &str = "__crepe_snapshot";

/// Name of forks of snapshot
///
/// snapshot_id(bytes32) => [fork_number(u64)]
pub const SNAPSHOT_FORK_TABLE: &str = "__crepe_snapshot_fork";

pub fn parse_u32(bytes: &[u8]) -> Result<u32> {
    if bytes.len() < 4 {
        return Err(Error::WrongBytesLength(4));
    }

    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

pub fn parse_u64(b: &[u8]) -> Result<u64> {
    if b.len() < 8 {
        return Err(Error::WrongBytesLength(4));
    }

    Ok(u64::from_le_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

// pub fn parse_fork_numbers(bytes: &[u8]) -> Result<Vec<u64>> {
//     Ok(Vec::new())
// }

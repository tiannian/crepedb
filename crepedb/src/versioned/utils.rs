use alloc::vec::Vec;
use crepedb_core::{
    backend::{BackendError, ReadTxn},
    SnapshotId,
};
use seq_macro::seq;

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

pub const NEXT_FORK_ID_KEY: &[u8; 8] = seq!(N in 0..8 { &[ #(0xff,)* ] });

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

pub fn parse_bytes32(b: &[u8]) -> Result<[u8; 32]> {
    if b.len() < 32 {
        return Err(Error::WrongBytesLength(32));
    }

    let r = seq!(N in 0..32 {
        [
            #(b[N],)*
        ]
    });

    Ok(r)
}

pub fn read_next_fork_id<E>(txn: &impl ReadTxn<E>) -> Result<u64>
where
    E: BackendError,
{
    let fork_id_data = txn
        .get(FORK_TABLE, NEXT_FORK_ID_KEY)
        .map_err(Error::backend)?;

    let fork_id = if let Some(b) = fork_id_data {
        parse_u64(&b)?
    } else {
        0
    };

    Ok(fork_id)
}

pub fn build_key(mut key: Vec<u8>, version: u64, snapshot_id: &SnapshotId) -> Vec<u8> {
    key.extend_from_slice(&version.to_le_bytes());
    key.extend_from_slice(snapshot_id.as_ref());

    key
}

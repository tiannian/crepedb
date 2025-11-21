use seq_macro::seq;

/// Name of meta table
///
/// table(str) => bool
pub const META_TABLE: &str = "__crepe_meta";

/// Name of snapshot table
///
/// snapshot_id(u64) => version,parent
pub const SNAPSHOT_TABLE: &str = "__crepe_snapshot";

/// Name of index of snapshot
///
/// snapshot_id(u64),k(u64) => snapshot_id(u64)
pub const SNAPSHOT_INDEX_TABLE: &str = "__crepe_snapshot_index";

pub const SNAPSHOT_NEXT_KEY: &[u8; 8] = &seq!(N in 0..8 { [ #(0xff,)* ] });

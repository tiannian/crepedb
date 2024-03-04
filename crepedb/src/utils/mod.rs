mod index;
pub use index::*;

mod snapshot;
pub use snapshot::*;

mod table;
pub use table::*;

pub mod consts;

mod parse;
pub use parse::*;

pub fn fast_ceil_log2(n: u64) -> u32 {
    u64::BITS - n.leading_zeros()
}

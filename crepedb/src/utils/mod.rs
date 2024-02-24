pub mod index;
pub mod snapshot;
pub mod table;

pub mod consts;

mod parse;
pub use parse::*;

pub fn fast_ceil_log2(n: u64) -> u32 {
    u64::BITS - n.leading_zeros()
}

#[cfg(test)]
mod tests {
    use std::println;

    use crate::utils::fast_ceil_log2;

    #[test]
    fn test_ceil_log2() {
        for i in 0..10 {
            println!("{}", fast_ceil_log2(i))
        }
    }
}

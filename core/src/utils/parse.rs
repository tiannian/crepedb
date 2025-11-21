use seq_macro::seq;

use crate::{Error, Result};

pub fn parse_u64(b: &[u8]) -> Result<u64> {
    if b.len() < 8 {
        return Err(Error::WrongBytesLength(8));
    }

    let r = seq!(N in 0..8 {
        [
            #(b[N],)*
        ]
    });

    Ok(u64::from_be_bytes(r))
}

pub fn dump_u64(v: u64) -> [u8; 8] {
    v.to_be_bytes()
}

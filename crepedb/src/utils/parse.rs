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

    Ok(u64::from_le_bytes(r))
}

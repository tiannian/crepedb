//! Byte array type definitions.

use alloc::vec::Vec;

/// Type alias for byte arrays used throughout CrepeDB.
///
/// This represents both keys and values in the database.
pub type Bytes = Vec<u8>;

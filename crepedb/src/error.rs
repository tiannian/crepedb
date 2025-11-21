//! Error types for CrepeDB operations.

use alloc::boxed::Box;

use crate::{backend::BackendError, SnapshotId};

/// Errors that can occur during CrepeDB operations.
#[derive(Debug)]
pub enum Error {
    /// The requested snapshot does not exist.
    MissingSnaopshot(SnapshotId),
    
    /// A byte slice has an unexpected length.
    WrongBytesLength(usize),
    
    /// An unknown table type identifier was encountered.
    UnexpectedTableType(u8),
    
    /// A required table is missing.
    MissingTable,
    
    /// The snapshot ID must be a common ancestor but is not.
    WrongSnapshotIdMustBeCommon,
    
    /// Only one root snapshot is supported, but an attempt was made to create another.
    OnlySupportOneRoot,
    
    /// An internal index is missing (fatal error).
    FatelMissingInnerIndex,
    
    /// The data operation flag is missing from serialized data.
    MissingDataOpFlag,
    
    /// An unknown data operation type was encountered.
    UnexpectedDataOpType(u8),

    /// An error from the underlying storage backend.
    BackendError(Box<dyn BackendError>),
}

impl Error {
    /// Wrap a backend error in a CrepeDB error.
    ///
    /// # Arguments
    ///
    /// * `e` - The backend error to wrap
    pub fn backend(e: impl BackendError + 'static) -> Self {
        Self::BackendError(Box::new(e))
    }
}

/// Result type alias for CrepeDB operations.
pub type Result<T> = core::result::Result<T, Error>;

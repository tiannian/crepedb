use alloc::boxed::Box;

use crate::{backend::BackendError, SnapshotId};

#[derive(Debug)]
pub enum Error {
    MissingSnaopshot(SnapshotId),
    WrongBytesLength(usize),
    UnexpectedTableType(u8),
    MissingTable,
    WrongSnapshotIdMustBeCommon,

    BackendError(Box<dyn BackendError>),
}

impl Error {
    pub fn backend(e: impl BackendError + 'static) -> Self {
        Self::BackendError(Box::new(e))
    }
}

pub type Result<T> = core::result::Result<T, Error>;

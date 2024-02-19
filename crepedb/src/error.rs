use alloc::boxed::Box;
use crepedb_core::backend::BackendError;

#[derive(Debug)]
pub enum Error {
    MissingKey,
    WrongBytesLength(usize),

    BackendError(Box<dyn BackendError>),
}

impl Error {
    pub fn backend(e: impl BackendError + 'static) -> Self {
        Self::BackendError(Box::new(e))
    }
}

pub type Result<T> = core::result::Result<T, Error>;

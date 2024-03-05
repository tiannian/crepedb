use core::marker::PhantomData;

pub struct ReadTxn<T, E> {
    pub(crate) txn: T,

    pub(crate) marker: PhantomData<E>,
}

use crepedb_core::backend::Backend;read

pub struct ReadTxn<'a, K, V, B>
where
    B: Backend<K, V>,
{
    pub(crate) txn: B::ReadTxn<'a>,
}

impl<'a, K, V, B> ReadTxn<'a, K, V, B>
where
    B: Backend<K, V>,
{
    pub fn get(&self, key: &K) -> Result<V, B::Error> {
        use crepedb_core::backend::ReadTxn;

        self.txn.get(key)
    }
}

use core::marker::PhantomData;

use crepedb_core::backend::Backend;

use crate::ReadTxn;

pub struct CrepeDB<K, V, B> {
    backend: B,
    marker_k: PhantomData<K>,
    marker_v: PhantomData<V>,
}

impl<K, V, B> CrepeDB<K, V, B>
where
    B: Backend<K, V>,
{
    pub fn open(path: &str) -> Result<Self, B::Error> {
        let backend = B::open_db(path)?;

        Ok(Self {
            backend,
            marker_k: PhantomData,
            marker_v: PhantomData,
        })
    }

    pub fn open_readonly(path: &str) -> Result<Self, B::Error> {
        let backend = B::open_readonly(path)?;

        Ok(Self {
            backend,
            marker_k: PhantomData,
            marker_v: PhantomData,
        })
    }

    pub fn read_txn(&self, table: &str) -> Result<ReadTxn<'_, K, V, B>, B::Error> {
        let txn = self.backend.read_txn(table)?;

        Ok(ReadTxn { txn })
    }
}

use core::marker::PhantomData;

use crate::{
    backend::{BackendError, ReadTable, ReadTxn, WriteTable, WriteTxn},
    Error, Result, TableType,
};

use super::consts;

pub struct MetaTable<T, E> {
    table: T,
    marker: PhantomData<E>,
}

pub fn meta_reader<T, E>(txn: &T) -> Result<MetaTable<T::Table<'_>, E>>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let table = txn.open_table(consts::META_TABLE).map_err(Error::backend)?;
    Ok(MetaTable {
        table,
        marker: PhantomData,
    })
}

pub fn meta_writer<T, E>(txn: &T) -> Result<MetaTable<T::Table<'_>, E>>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    let table = txn.open_table(consts::META_TABLE).map_err(Error::backend)?;
    Ok(MetaTable {
        table,
        marker: PhantomData,
    })
}

impl<T, E> MetaTable<T, E>
where
    T: ReadTable<E>,
    E: BackendError,
{
    pub fn read_type(&self, table: &str) -> Result<TableType> {
        let bytes = self
            .table
            .get(table.as_bytes())
            .map_err(Error::backend)?
            .ok_or(Error::MissingTable)?;

        let byte = bytes.first().ok_or(Error::WrongBytesLength(1))?;

        let ty = TableType::from_byte(*byte)?;

        Ok(ty)
    }
}

impl<T, E> MetaTable<T, E>
where
    T: WriteTable<E>,
    E: BackendError,
{
    pub fn write_type(&self, table: &str, ty: &TableType) -> Result<()> {
        self.table
            .set(table.as_bytes(), &[ty.to_byte()])
            .map_err(Error::backend)?;

        Ok(())
    }
}

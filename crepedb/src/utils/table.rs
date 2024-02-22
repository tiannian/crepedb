use crate::{
    backend::{BackendError, ReadTxn, WriteTxn},
    Error, Result, TableType,
};

use super::consts;

pub fn read_type<T, E>(txn: &T, table: &str) -> Result<TableType>
where
    T: ReadTxn<E>,
    E: BackendError,
{
    let bytes = txn
        .get(consts::META_TABLE, table.as_bytes())
        .map_err(Error::backend)?
        .ok_or(Error::MissingTable)?;

    let byte = bytes.first().ok_or(Error::WrongBytesLength(1))?;

    let ty = TableType::from_byte(*byte)?;

    Ok(ty)
}

pub fn write_type<T, E>(txn: T, table: &str, ty: &TableType) -> Result<()>
where
    T: WriteTxn<E>,
    E: BackendError,
{
    txn.set(consts::META_TABLE, table.as_bytes(), &[ty.to_byte()])
        .map_err(Error::backend)?;

    txn.commit().map_err(Error::backend)?;

    Ok(())
}

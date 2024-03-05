mod db;
pub use db::*;

mod read;
pub use read::*;

mod write;
pub use write::*;

mod table;
pub use table::*;

mod range;
pub use range::*;

pub(crate) mod types;

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use crepedb::{CrepeDB, Result, SnapshotId};

    use crate::RedbDatabase;

    #[test]
    fn test_db_10() -> Result<()> {
        let _ = env_logger::builder().is_test(true).try_init();

        let path = Path::new("/tmp/__crepedb");

        fs::create_dir_all(path).unwrap();

        let db: CrepeDB<RedbDatabase> = CrepeDB::open("/tmp/__crepedb/snapshot_10")?;

        let mut sid = SnapshotId::preroot();

        for _ in 1..14 {
            log::trace!("SnapshotId is :{:?}", sid);

            let write_txn = db.write(sid)?;
            log::info!("{:?}", write_txn);

            let nsid = write_txn.commit()?;

            sid = nsid;
        }

        fs::remove_file("/tmp/__crepedb/snapshot_10").unwrap();

        Ok(())
    }
}

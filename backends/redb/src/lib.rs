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
    use crate::RedbDatabase;

    #[test]
    fn test_db_10() {
        let _ = env_logger::builder().is_test(true).try_init();

        crepedb::utils::tests::test_db_10::<RedbDatabase>().unwrap();
    }
}

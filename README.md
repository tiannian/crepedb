# CrepeDB

An versioned and forkable embedded Key-Value database.

## Supported Features

- Create shopshots based on any snapshot.
- Read data based on any snapshot.
- All snapshots are organized into this tree.
- Table can be Versioned and Basic(No snapshot).
- Versioned and Basic table at one transaction.
- High-performance reads and inserts
- Multi backend support.
- Provide tools for analyzing databases, migrating, and managing data.

## backends

For now, CrepeDB supports these backend:

- [x] redb
- [ ] mdbx
- [ ] sled
- [ ] rocksdb
- [ ] leveldb

## Usage

### Crates

If you want to use this crate, please add `crepedb` and a backend what you want
to use.

```bash
cargo add crepedb 
cargo add crepedb-redb # replace backend crate you want to use.
```

### Usage

```rust
use crepedb::CrepeDB;
use crepedb_redb::RedbDatabase;

let backend = RedbDatabase::memory().unwarp();
let db = CrepeDB::new(backend);
```


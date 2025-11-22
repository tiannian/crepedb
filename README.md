# CrepeDB

A versioned and forkable embedded Key-Value database. It aims to be used as storage for blockchain applications.

CrepeDB provides a multi-version concurrency control (MVCC) database with snapshot isolation. It supports forking database snapshots and maintains version history efficiently.

## Features

- **Versioned Storage**: Track changes across multiple versions
- **Snapshot Isolation**: Create and read from consistent snapshots
- **Fork Support**: Create new branches from any snapshot
- **Table Types**: Support both versioned and basic (non-versioned) tables
- **High Performance**: Optimized for fast reads and writes
- **Multi Backend Support**: Use different storage backends
- **ACID Transactions**: Full transactional support

## Supported Backends

CrepeDB supports multiple storage backends:

- [x] **redb** - A simple, portable, high-performance embedded key-value database
- [x] **rocksdb** - A high-performance embedded database based on RocksDB
- [x] **mdbx** - A fast, compact, powerful embedded transactional key-value database

## Installation

Add `crepedb` to your `Cargo.toml` with the desired backend feature:

```bash
# Using cargo add (redb is enabled by default)
cargo add crepedb

# Or specify a different backend
cargo add crepedb --features backend-rocksdb
cargo add crepedb --features backend-mdbx
```

Or manually in `Cargo.toml`:

```toml
[dependencies]
# Default: redb backend is enabled
crepedb = { version = "0.1" }

# Or enable a specific backend
crepedb = { version = "0.1", features = ["backend-redb"] }
crepedb = { version = "0.1", features = ["backend-rocksdb"] }
crepedb = { version = "0.1", features = ["backend-mdbx"] }

# Enable multiple backends
crepedb = { version = "0.1", features = ["backend-redb", "backend-rocksdb"] }
```

## Usage

### Basic Example

```rust
use crepedb::CrepeDB;
use crepedb::backend::RedbDatabase;

// Create a database with a backend
let backend = RedbDatabase::memory()?;
let db = CrepeDB::new(backend);

// Create root snapshot
let wtxn = db.write(None)?;
wtxn.create_versioned_table("my_table")?;
let root = wtxn.commit()?;

// Write data
let wtxn = db.write(Some(root))?;
let mut table = wtxn.open_table("my_table")?;
table.set(b"key".to_vec(), b"value".to_vec())?;
let snapshot1 = wtxn.commit()?;

// Read data
let rtxn = db.read(Some(snapshot1))?;
let table = rtxn.open_table("my_table")?;
let value = table.get(b"key".to_vec())?;
assert_eq!(value, Some(b"value".to_vec()));
```

### Table Types

CrepeDB supports two types of tables:

1. **Versioned Tables**: Track all changes across snapshots. Each write creates a new version entry. Reads can retrieve data from any snapshot in the version history.

   ```rust
   wtxn.create_versioned_table("versioned_table")?;
   ```

2. **Basic Tables**: Store data directly with no version tracking. Updates overwrite previous values. More efficient for data that doesn't need version history.

```rust
wtxn.create_basic_table("basic_table")?;
```

### Forking Snapshots

You can create multiple branches from the same snapshot:

```rust
// Create branch 1 from root
let wtxn1 = db.write(Some(root.clone()))?;
let mut table1 = wtxn1.open_table("my_table")?;
table1.set(b"key".to_vec(), b"value1".to_vec())?;
let branch1 = wtxn1.commit()?;

// Create branch 2 from root (independent of branch1)
let wtxn2 = db.write(Some(root))?;
let mut table2 = wtxn2.open_table("my_table")?;
table2.set(b"key".to_vec(), b"value2".to_vec())?;
let branch2 = wtxn2.commit()?;

// Each branch maintains its own data
let rtxn1 = db.read(Some(branch1))?;
let table1 = rtxn1.open_table("my_table")?;
assert_eq!(table1.get(b"key".to_vec())?, Some(b"value1".to_vec()));

let rtxn2 = db.read(Some(branch2))?;
let table2 = rtxn2.open_table("my_table")?;
assert_eq!(table2.get(b"key".to_vec())?, Some(b"value2".to_vec()));
```

### Using Different Backends

#### Redb Backend

```rust
use crepedb::CrepeDB;
use crepedb::backend::RedbDatabase;

// In-memory database
let backend = RedbDatabase::memory()?;

// Persistent database
let backend = RedbDatabase::open("path/to/db")?;
```

#### RocksDB Backend

```rust
use crepedb::CrepeDB;
use crepedb::backend::RocksdbDatabase;

let backend = RocksdbDatabase::open_or_create("path/to/db")?;
```

#### MDBX Backend

```rust
use crepedb::CrepeDB;
use crepedb::backend::MdbxDatabase;

let backend = MdbxDatabase::open_or_create("path/to/db.mdbx")?;
```

## Command Line Tools (WIP)

You can use the `crepedb` command line tool to read and manage databases. See the [tool documentation](https://github.com/tiannian/crepedb/tree/main/tool) for more information.

Install the tool:

```bash
cargo install crepedb-tools
```

## Documentation

For more detailed documentation, see:

- [Design Document](docs/design.md) - Architecture and design details
- [API Documentation](https://docs.rs/crepedb) - Full API reference

## License

Licensed under the Apache License, Version 2.0.

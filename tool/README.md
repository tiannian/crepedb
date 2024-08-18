# Tools for CrepeDB

> A tools to read, load, export or operations for CrepeDB.

## Install

```sh
cargo install crepedb-tools
```

## Reference

### List all tables

```bash
crepedb table list
```

### Get Snapahot

```bash
# Get root snapshot
crepedb snapshot root

# Get latest snapshot
crepedb snapshot latest

# Get snapshot by version
crepedb snapshot 100

# Get snapshot by snapshot id
crepedb snapshot --id 100
```

### Get Value by Key

```bash
# Get value by table and Key
crepedb get table:key

# Get value by table and Key at snapshot
crepedb get table:key --snapshot-id 100
```

### Create Database

```bash
crepedb --database /path/to/db --backend redb new
```

### Create table

```bash
crepedb table new --type basic table-name --database /path/to/database --backend redb
```


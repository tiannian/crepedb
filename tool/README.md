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
crepedb value get table:key

# Get value by table and Key at snapshot
crepedb value get table:key --snapshot-id 100
```

### Create table

```bash
crepedb table new --type basic table-name
```


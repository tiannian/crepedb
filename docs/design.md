# Design

## Backend

Backend must support these features:

1. Support get value by key
2. Support iterate key value pair in range.
3. Support multi namespace or tables.

Generally, the data structure of B+ tree is used on the disk.

## Table

Table have two type:

1. Basic: Unversioned table. It's just a plain Key-Value table. Can't make any `Snapshot`.
2. Versioned: You can make or read any Snapshot.

### `Meta Table`

CrepeDB will record table's property in `__crepe_meta`. The key is table name,
The first byte in value is table type. `1` means table is `Basic`. `2` means table is `Versioned`.

## Snapshot

Snapshot ID is `u64`. `0` is `Preroot`, `1` is `Root`.
Version is `u64`.

### `Snapshot Table`

This table use to store snapshot related property.

- Key: Snapshot ID
- Value: Version, Parent Snapshot ID.

Snapshot Table use name `__crepe_snapshot`.

### `Snapshot Index Table`

### `Snapshot Index Table`

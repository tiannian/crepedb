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

### Versioned Table

This table's key is made up by these parts:

1. key (variable length)
2. version (u64, little-endian)
3. snapshot id (u64, little-endian)

The value is encoded with a data operation flag at the end:
- `0x00`: Set operation, the value contains actual data followed by the flag
- `0x01`: Del operation, only the flag is stored

When reading a key, the system searches backward from the highest version/snapshot combination
that is an ancestor of the current snapshot, ensuring snapshot isolation.

## Snapshot

Snapshot ID is `u64`. `u64::MAX` (0xFFFFFFFFFFFFFFFF) is `Preroot`, `0` is `Root`.
Version is `u64`. Root snapshot has version `0`.

When creating a new snapshot from an existing snapshot:
- The new snapshot ID is automatically assigned incrementally (1, 2, 3, ...)
- The new version is parent version + 1
- The parent snapshot ID is stored for tracking the snapshot tree

### `Snapshot Table`

This table stores snapshot-related properties.
Snapshot Table uses name `__crepe_snapshot`.

Regular entries:
- Key: Snapshot ID (u64)
- Value: Version (u64) + Parent Snapshot ID (u64)

Special entry for ID allocation:
- Key: `0xFFFFFFFFFFFFFFFF` (u64::MAX, same as Preroot)
- Value: Next Snapshot ID to be allocated (u64)

This special key tracks the next available snapshot ID to ensure unique ID allocation.

### `Snapshot Index Table`

Snapshot Index Table stores skip-list-like indices to enable efficient ancestor traversal.
It uses name `__crepe_snapshot_index`.

- Key: Snapshot ID (u64) + Index Number (u32)
- Value: Referenced Snapshot ID (u64)

To build index, we use these expression:

- `Vn`: Snapshot with version N.
- `i(Va, b)`: Index `b` of Snapshot with version `a`.
- `V0`: Root node (version 0).

If snapshot has version `a`, it will have `i = ceil(log2(a))` indices stored in the index table.

Note: Index 0 is conceptually the direct parent snapshot, but it's not stored in the index table. 
Instead, it's stored in the Snapshot Table as the "Parent Snapshot ID". 
Only indices 1, 2, 3, ... are actually stored in the Snapshot Index Table for skip-list optimization.
Higher indices provide skip-list jumps for faster ancestor lookup.

For example:

```
V0 = (Root Node)
! = (Empty)

i(V1, 0) = V0

i(V2, 0) = V1
i(V2, 1) = i(i(V2, 0), 0) = i(V1, 0) = V0

i(V3, 0) = V2
i(V3, 1) = i(i(V3, 0), 0) = i(V2, 0) = V1
i(V3, 2) = i(i(V3, 1), 1) = i(V1, 1) = !

i(V4, 0) = V3
i(V4, 1) = i(i(V4, 0), 0) = i(V3, 0) = V2
i(V4, 2) = i(i(V4, 1), 1) = i(V2, 1) = V0

i(V5, 0) = V4
i(V5, 1) = i(i(V5, 0), 0) = i(V4, 0) = V3
i(V5, 2) = i(i(V5, 1), 1) = i(V3, 1) = V1
i(V5, 3) = i(i(V5, 2), 2) = i(V1, 2) = !

i(V6, 0) = V5
i(V6, 1) = i(i(V6, 0), 0) = i(V5, 0) = V4
i(V6, 2) = i(i(V6, 1), 1) = i(V4, 1) = V2
i(V6, 3) = i(i(V6, 2), 2) = i(V2, 2) = !

i(V7, 0) = V6
i(V7, 1) = i(i(V7, 0), 0) = i(V6, 0) = V5
i(V7, 2) = i(i(V7, 1), 1) = i(V5, 1) = V3
i(V7, 3) = i(i(V7, 2), 2) = i(V3, 2) = !


i(V8, 0) = V7
i(V8, 1) = i(i(V8, 0), 0) = i(V7, 0) = V6
i(V8, 2) = i(i(V8, 1), 1) = i(V6, 1) = V4
i(V8, 3) = i(i(V8, 2), 2) = i(V4, 2) = V0


i(V9, 0) = V8
i(V9, 1) = i(i(V9, 0), 0) = i(V8, 0) = V7
i(V9, 2) = i(i(V9, 1), 1) = i(V7, 1) = V5
i(V9, 3) = i(i(V9, 2), 2) = i(V5, 2) = V1
i(V9, 4) = i(i(V9, 3), 3) = i(V2, 2) = !

i(V10, 0) = V9
i(V10, 1) = i(i(V10, 0), 0) = i(V9, 0) = V8
i(V10, 2) = i(i(V10, 1), 1) = i(V8, 1) = V6
i(V10, 3) = i(i(V10, 2), 2) = i(V6, 2) = V2
i(V10, 4) = i(i(V10, 3), 3) = i(V2, 2) = !
```


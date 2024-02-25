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

To build index, we use these expression.

- `Vn`: Snapshot with version N.
- `i(Va, b)`: Index `b` of Snapshot with version `a`.
- `V0`: Root node.

If snapshot have version `a`, it will have `i = ceil(log2(a))`.

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


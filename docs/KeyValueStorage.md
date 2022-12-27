# Storage Design

## Basic Storage

### Key-Value Storage

The basic storage is Key-Value storage.

```
key => value
```

This type of storage support these operation:

``` rust
/// Get value based on key
fn get(key: &[u8]) -> Bytes

/// Set key value
fn set(key: Bytes, value: Bytes);

/// Delete value based on key
fn del(key: &[u8]);

/// Range of key
fn range(begin: &[u8], end: &[u8], reverse: bool) -> Iter;
```

### Versioned Key-Value Storage

Also we have a Versioned Storage. To support, this storage
use following key structure.

```
key:version => value
[u8][u8;32] => [u8]
```

#### Operations

This storage support these operations.

- get_by_version(key, version) -> value
- del_by_version(key, version)
- set_by_version(key, value, version)
- iterate_version(key, reverse) -> iterator

## Structure Storage

### Block

### Transaction

### Receipt

### State



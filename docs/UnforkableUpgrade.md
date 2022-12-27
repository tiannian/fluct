# Unforkable Upgrade

## WebAssembly Runtime

### Store API

```rust
extern "C" fluct_store_db_open(*const c_char name) -> c_int;
extern "C" fluct_store_db_open_ro(*const c_char name) -> c_int;
```

### Instance API

```rust
extern "C" fluct_wasm_instance_run(
    *const c_char name,
    *const u8 args,
) -> c_int;
```

### Consensus Entry

## Contracts

```solidity
interface UnforkableUpgrade {
    function getVersionByHeight(uint256 height) returns(bytes32 txid) view external;

    function getCodeHashByVersion(bytes32 version) returns(bytes32 code_hash) view external;
}
```



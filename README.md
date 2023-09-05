# Fluct

> Fluct is a modular blockchain framework for multi
> chain, like Rollup, Layer 1 or other chain.

## Module

### Execution Module

- [X] op-geth: Embeded op-geth into rust code.
- [ ] workbench: Anvil fork with engine api support

### Beacon Module

Beacon derive Executive Module to execute transaction and apply state.

- [ ] Devnet: Single node with manual block seal.
- [ ] PoW: PoW to seal block.
- [ ] BFT: Use bft algorithm to seal block.
- [ ] Optimistic: Load state from L1 and DA Service

### Sequencer Module

- [ ] Fifo
- [ ] Gas fee order
- [ ] Order from other Consensus(decentralised sequencer)

### P2P Module

P2P Module sync transaction into sequencer and sync block into Execution Module.

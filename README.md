# Fluct

> Fluct is a modular blockchain framework for multi
> chain, like Rollup, Layer 1 or other chain.

## Module

### Execution Module

Execute transaction and build state locally.

- [x] op-geth: Embeded op-geth into rust code.
- [ ] workbench: Anvil fork with engine api support

### Beacon Module

Beacon derive execution module to execute transaction and apply state. This module accept data from consensus.

- [ ] Single: Single node with manual block seal.
- [ ] PoW: PoW to seal block.
- [ ] BFT: Use bft algorithm to seal block.
- [ ] Optimistic: Seal block from L1 and A.

### Sequencer Module

Sequencer work with beacon to seal block. This module build block from transaction pool.

- [x] FIFO
- [ ] Gas fee order
- [ ] Order from other Consensus(decentralised sequencer)

### P2P Module

P2P Module sync transaction into sequencer and sync block into Execution Module.

### RPC Module

Expose block and tx data from execution module, and send transaction into sequencer.

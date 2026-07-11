# Hyperledger Besu Configuration for Rinova

This directory contains the configuration files for the Hyperledger Besu private blockchain network used by Rinova Website Builder.

## Overview

The blockchain network provides:
- **Proof of Ownership** for premium assets (ERC-721/ERC-1155)
- **Immutable Audit Trail** for website publications (Requirement 8.1, 8.6, 9.1-9.8)

## Consensus Mechanism

We use **IBFT 2.0 (Istanbul Byzantine Fault Tolerance)** which provides:
- Byzantine Fault Tolerance (BFT) for handling node failures
- Fast block finality (5 seconds)
- No forks - all valid blocks are final
- Tolerates up to f faulty nodes in a network of 3f+1 nodes

## Directory Structure

```
hyperledger_besu_service/
├── genesis.json              # Genesis block configuration with IBFT 2.0
├── config.toml               # Node configuration (RPC, P2P, mining)
├── keys/
│   └── nodekey               # Private key for node identity
└── permissions/
    └── permissions_config.toml # Permissioning configuration (future use)
```

## Configuration Details

### Genesis Block (`genesis.json`)

- **Chain ID**: 20182024 (unique identifier for Rinova network)
- **Consensus**: IBFT 2.0 (Byzantine Fault Tolerance)
- **Block Period**: 5 seconds
- **Epoch Length**: 30000 blocks
- **Pre-funded Accounts**:
  - `0xfe3b557e8fb62b89f4916b721be55ceb828dbd73` - Primary validator
  - `0x8f2a55949038a9610f50fb23b5883af3b4ecb3c3` - Secondary validator
  - `0xc8a60266e8c5b4d7a9c6f3b9e5a7b6c8d2e4f6a8` - Blockchain service

### Node Configuration (`config.toml`)

#### JSON-RPC Endpoint (Port 8545)

The JSON-RPC endpoint provides synchronous access to the blockchain:

```bash
# Example: Get latest block number
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545

# Example: Get account balance
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0xfe3b557e8fb62b89f4916b721be55ceb828dbd73","latest"],"id":1}' \
  http://localhost:8545
```

Available API Methods:
- `ETH` - Core Ethereum methods
- `NET` - Network information
- `WEB3` - Web3 utilities
- `IBFT` - IBFT 2.0 consensus methods
- `CLIQUE` - PoA methods (for migration compatibility)
- `ADMIN` - Node administration
- `DEBUG` - Debug utilities
- `MINER` - Mining configuration
- `EEA` - Private transaction methods
- `PRIV` - Privacy methods
- `PERM` - Permissioning methods

#### WebSocket Endpoint (Port 8546)

The WebSocket endpoint provides real-time event subscriptions:

```javascript
// Example: Subscribe to new blocks
const ws = new WebSocket('ws://localhost:8546');
ws.onopen = () => {
  ws.send(JSON.stringify({
    jsonrpc: '2.0',
    id: 1,
    method: 'eth_subscribe',
    params: ['newHeads']
  }));
};
ws.onmessage = (event) => {
  console.log('New block:', JSON.parse(event.data));
};
```

#### P2P Networking (Port 30303)

The P2P endpoint handles node discovery and block synchronization:
- Supports up to 25 peer connections
- Discovery protocol enabled for automatic peer finding
- Compatible with other Ethereum clients (geth, etc.)

## Docker Compose Configuration

The Besu service is configured in `docker-compose.yml` with:

```yaml
besu:
  image: hyperledger/besu:latest
  ports:
    - "8545:8545"   # JSON-RPC
    - "8546:8546"   # WebSocket
    - "30303:30303" # P2P
  volumes:
    - besu_data:/opt/besu/data
    - ./docker/hyperledger_besu_service:/opt/besu/config
  command: >
    --config-file=/opt/besu/config/config.toml
```

## Connecting from Backend Services

### Rust (ethers-rs)

```rust
use ethers::providers::{Provider, Http, Ws};
use ethers::types::Address;
use std::str::FromStr;

// HTTP Provider for synchronous operations
let http_provider = Provider::<Http>::try_from("http://besu:8545")?;

// WebSocket Provider for real-time events
let ws_provider = Provider::<Ws>::connect("ws://besu:8546").await?;

// Example: Get account balance
let address = Address::from_str("0xfe3b557e8fb62b89f4916b721be55ceb828dbd73")?;
let balance = http_provider.get_balance(address, None).await?;
```

## Byzantine Fault Tolerance (Requirement 8.6)

IBFT 2.0 provides BFT by:

1. **Fault Tolerance**: Can tolerate up to f faulty nodes in a network of 3f+1 nodes
2. **No Forks**: All committed blocks are immediately final
3. **Fast Finality**: Blocks are finalized within one block period (5 seconds)
4. **Validator Rotation**: Epoch-based validator list updates
5. **Consensus Recovery**: Automatic recovery from temporary network partitions

For production, deploy at least 4 validators to achieve BFT with 1 faulty node tolerance.

## Performance Considerations

Based on Requirements:

| Metric | Requirement | Configuration |
|--------|-------------|---------------|
| Block Time | - | 5 seconds (configurable) |
| Transaction Confirmation | < 2 seconds (8.3) | JSON-RPC with caching |
| Audit Trail Recording | < 5 seconds (9.3) | Direct mining, zero gas price |
| Hash Computation | < 10 seconds (9.1) | Handled by backend service |

## Security Considerations

1. **Network Isolation**: Besu runs on the internal Docker network
2. **CORS**: Currently open for development, restrict in production
3. **Authentication**: Disabled for development, enable for production
4. **TLS**: Consider TLS termination at load balancer for production
5. **Private Keys**: Stored in Docker secrets for production deployment

## Monitoring

Prometheus metrics are exposed on port 9545:

```bash
curl http://localhost:9545/metrics
```

Key metrics to monitor:
- `besu_blockchain_chain_head_height` - Current block number
- `besu_consensus_ibft_round_number` - Current consensus round
- `besu_peer_count` - Number of connected peers
- `besu_rpc_http_request_duration` - RPC latency

## Troubleshooting

### Node won't start
```bash
# Check logs
docker logs rinova-besu

# Common issues:
# - Invalid genesis file format
# - Port conflicts
# - Volume permission issues
```

### RPC calls fail
```bash
# Verify RPC is enabled
curl -X POST http://localhost:8545 -d '{"jsonrpc":"2.0","method":"web3_clientVersion","params":[],"id":1}'

# Check CORS settings if calling from browser
```

### WebSocket disconnects
```bash
# Check WebSocket port
netstat -tlnp | grep 8546

# Verify firewall allows WebSocket connections
```

## Smart Contract Deployment

After starting the network, deploy smart contracts:

```bash
# Using Hardhat/Foundry
npx hardhat run scripts/deploy.js --network rinova

# Contract addresses should be set in environment variables:
# OWNERSHIP_CONTRACT_ADDRESS=0x...
# AUDIT_CONTRACT_ADDRESS=0x...
```

## Additional Resources

- [Hyperledger Besu Documentation](https://besu.hyperledger.org/)
- [IBFT 2.0 Consensus](https://besu.hyperledger.org/en/stable/Concepts/Consensus-Protocols/IBFT/)
- [Private Network Tutorial](https://besu.hyperledger.org/en/stable/Tutorials/Private-Network/Create-IBFT-Network/)

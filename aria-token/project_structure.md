# ARIA Token Project Structure

The ARIA token is a Solana-based cryptocurrency for the decentralized AI personal assistant ecosystem. Below is the cleaned and enhanced project structure with all MVP functionality.

```
aria-token/
├── src/                  # Rust source code
│   ├── lib.rs            # Main contract implementation
│   ├── error.rs          # Error handling
│   ├── instruction.rs    # Instruction definitions
│   ├── metadata.rs       # Token metadata implementation
│   ├── token_economy.rs  # Token economic features (burn, lock)
│   ├── security.rs       # Authority and permissions system
│   ├── events.rs         # Event logging system
│   ├── upgradable.rs     # Contract upgrade mechanism
│   ├── pump_fun.rs       # pump.fun integration
│   └── test.rs           # Tests
├── scripts/              # Deployment scripts
│   └── deploy.js         # Script to deploy token on Solana
├── keys/                 # Directory for keypairs
│   └── .gitkeep          # Placeholder to ensure directory is tracked
├── Cargo.toml            # Rust project configuration
├── package.json          # Node.js dependencies
├── README.md             # Project documentation
├── DEPLOYMENT.md         # Deployment guide
└── .gitignore            # Git ignore rules
```

## Core Features Implemented

### Token Contract (src/lib.rs)
- Token initialization with 9 decimals
- Initial supply of 100 million tokens
- Distribution according to tokenomics plan:
  - 50% for user incentives
  - 20% for team development
  - 20% for community governance
  - 10% for marketing partnerships

### Token Metadata (src/metadata.rs)
- Token name: "ARIA Token"
- Token symbol: "ARI"
- URI for logo and additional information
- Metadata management functions for pump.fun listing

### Token Economy Features (src/token_economy.rs)
- Token burning mechanism
- Token locking/vesting for team and marketing allocations
- Lock duration customization

### Security and Authority System (src/security.rs)
- Role-based access control
- Multiple authority types (Admin, Minter, Freezer, Burner)
- Authority management functions

### Event Logging System (src/events.rs)
- Structured event logging
- Transaction tracking
- Base58 encoded events for client parsing
- Comprehensive event types for all operations

### Contract Upgrade Mechanism (src/upgradable.rs)
- Versioning system (major.minor.patch)
- Scheduled upgrades with delay
- Authority-controlled upgrade process
- Cancellation functionality

### pump.fun Integration (src/pump_fun.rs)
- Platform-specific configuration
- Price and fee management
- Trading controls
- Transaction limits

### Error Handling (src/error.rs)
- Comprehensive error types
- Utility functions for common checks
- Clean error messages

### Deployment and Client Integration (scripts/deploy.js)
- Token deployment workflow
- Metadata setup
- Authority initialization
- Token allocation distribution
- Team and marketing tokens locking
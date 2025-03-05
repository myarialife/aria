# ARIA Project Directory Structure

This document describes the directory structure of the ARIA project and the purpose of each component.

## Top-level Directory Structure

```
aria/
├── android/              # Android client application
├── aria-token/           # Solana token smart contract
├── assets/               # Project resources (images, logos, etc.)
├── backend/              # Backend service API
├── docs/                 # Project documentation
├── keys/                 # Keys directory (not version controlled)
├── scripts/              # Project scripts (deployment, build, etc.)
└── README.md             # Project overview
```

## Android Client

```
android/
└── app/
    └── src/
        └── main/
            ├── java/com/aria/assistant/   # Java/Kotlin source code
            │   ├── AriaApplication.kt     # Application entry class
            │   ├── blockchain/            # Blockchain interaction components
            │   ├── data/                  # Data layer
            │   │   ├── dao/               # Data access objects
            │   │   ├── entities/          # Data entity classes
            │   │   └── repositories/      # Data repositories
            │   ├── di/                    # Dependency injection
            │   ├── models/                # Model classes
            │   ├── network/               # Network requests
            │   │   └── models/            # API request/response models
            │   ├── services/              # Background services
            │   ├── ui/                    # User interface
            │   │   ├── assistant/         # AI assistant interface
            │   │   ├── dashboard/         # Main dashboard interface
            │   │   ├── data/              # Data management interface
            │   │   └── wallet/            # Wallet interface
            │   └── utils/                 # Utility classes
            └── res/                       # Android resource files
                ├── drawable/              # Image resources
                ├── layout/                # Layout files
                ├── menu/                  # Menu files
                ├── navigation/            # Navigation graph
                └── values/                # Strings, colors, and other resources
```

## Solana Token Smart Contract

```
aria-token/
├── keys/                 # Keys directory (not version controlled)
├── scripts/              # Deployment scripts
└── src/                  # Contract source code
    ├── error.rs          # Error handling
    ├── events.rs         # Event definitions
    ├── instruction.rs    # Instruction definitions
    ├── lib.rs            # Main contract logic
    ├── metadata.rs       # Token metadata
    ├── pump_fun.rs       # pump.fun integration
    ├── security.rs       # Security management
    ├── token_economy.rs  # Token economy mechanisms
    └── upgradable.rs     # Contract upgrade mechanism
```

## Backend Services

```
backend/
└── src/
    ├── api/                # API endpoints
    │   ├── controllers/    # Controllers
    │   └── routes/         # Route definitions
    ├── config/             # Configuration files
    ├── models/             # Data models
    ├── services/           # Business services
    ├── utils/              # Utility functions
    └── index.js            # Backend entry file
```

## Documentation Directory

```
docs/
├── api/                  # API documentation
├── architecture/         # Architecture design documents
├── images/               # Images used in documentation
└── project_structure.md  # Project structure document (this file)
```

## Scripts Directory

```
scripts/
├── android/              # Android client build scripts
├── backend/              # Backend service scripts
└── deployment/           # Deployment scripts
```

## Assets Directory

```
assets/
├── aria-logo.svg         # ARIA logo
└── other-resources...    # Other project resources
``` 
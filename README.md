# Decentik - Decentralized TikTok

A Web3 social video platform built with Rust, featuring cryptographic signatures, decentralized storage, and cross-protocol compatibility.

## 🚀 Features

- **Cryptographic Integrity**: Ed25519 signatures for tamper-proof content
- **Decentralized Storage**: IPFS + Livepeer for video processing
- **Cross-Platform**: AT Protocol integration for interoperability
- **Advanced Algorithm**: Graph-based feed with engagement scoring
- **Hexagonal Architecture**: Scalable, testable, modular design

## 🏗️ Architecture

```
dtt-core/     # Domain logic & business rules
dtt-infra/    # Infrastructure implementations
dtt-api/      # REST API & authentication
```

## ⚡ Quick Start

### Prerequisites
- Rust 1.70+
- SurrealDB
- IPFS node
- Livepeer API key

### Installation

```bash
git clone <repo>
cd decentik

# Set environment variables
export SURREAL_URL="ws://localhost:8000"
export JWT_SECRET="your-secret-key"
export LIVEPEER_API_KEY="your-api-key"
export IPFS_GATEWAY="http://localhost:5001"

# Run
cargo run -p dtt-api
```

## 📡 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/register` | User registration |
| POST | `/auth/login` | Authentication |
| POST | `/upload` | Video upload |
| GET | `/feed` | Personalized feed |
| GET | `/profile/:id` | User profile |

## 🔧 Configuration

Environment variables:
- `SURREAL_URL`: Database connection
- `JWT_SECRET`: Authentication secret
- `LIVEPEER_API_KEY`: Video processing
- `IPFS_GATEWAY`: Decentralized storage
- `AT_PDS_URL`: AT Protocol server
- `AT_DID`: Decentralized identifier

## 🛠️ Development

```bash
# Test
cargo test

# Build
cargo build --release

# Run specific crate
cargo run -p dtt-core
```

## 📚 Documentation

- [User Guide](docs/USER_GUIDE.md)
- [Developer Guide](docs/DEVELOPER_GUIDE.md)
- [API Reference](docs/API.md)

## 🤝 Contributing

1. Fork repository
2. Create feature branch
3. Submit pull request

## 📄 License

MIT License
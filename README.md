# Decentik - Decentralized TikTok

A Web3 social video platform built with Rust, featuring cryptographic signatures, decentralized storage, and cross-protocol compatibility.

## 🚀 Features

- **Enhanced Security**: Ed25519 signatures + key rotation + signature verification
- **Decentralized Storage**: IPFS + Livepeer for video processing
- **Cross-Platform**: AT Protocol integration for interoperability
- **ML-Powered Algorithm**: Engagement scoring with caching optimization
- **Rate Limiting**: Abuse prevention with configurable limits
- **Mobile-First UI**: TikTok-like responsive web interface
- **Hexagonal Architecture**: Clean, testable, maintainable design

## 🏗️ Enhanced Hexagonal Architecture

```
dtt-core/     # Domain logic & business rules
├── domain/   # Entities, value objects, aggregates
├── ports/    # Interface definitions (repositories, services)
└── use_cases/# Application business logic

dtt-infra/    # Infrastructure implementations
├── db/       # Database adapters
├── protocol/ # External service adapters
├── security/ # Cryptography & rate limiting
└── services/ # Algorithm implementations

dtt-api/      # REST API & authentication
└── handlers/ # HTTP request handlers

dtt-ui/       # Web interface (hexagonal compliant)
├── adapters/ # UI service orchestration
├── handlers/ # Web route handlers
└── templates/# HTML templates
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

# Run enhanced version with UI
cargo run -p dtt-api --bin enhanced_main

# Or run original API-only version
cargo run -p dtt-api
```

## 📡 API Endpoints

### Web Interface
| Route | Description |
|-------|-------------|
| `/` | Redirect to feed |
| `/feed` | Mobile-optimized video feed |
| `/upload` | Drag & drop video upload |
| `/profile/:id` | User profile page |

### API Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/register` | User registration |
| POST | `/auth/login` | Authentication |
| POST | `/api/upload` | Video upload with signature |
| GET | `/api/feed` | Personalized feed JSON |
| GET | `/api/profile/:id` | User profile JSON |

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
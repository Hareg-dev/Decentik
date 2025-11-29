# Developer Guide

## Architecture Overview

### Hexagonal Architecture
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   dtt-api       │    │   dtt-core      │    │   dtt-infra     │
│                 │    │                 │    │                 │
│ • REST handlers │───▶│ • Domain models │◀───│ • SurrealDB     │
│ • Auth middleware│    │ • Business logic│    │ • Livepeer      │
│ • JWT tokens    │    │ • Use cases     │    │ • AT Protocol   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Components

#### Domain Models (`dtt-core/src/domain/`)
- `User`: Identity and profile
- `Post`: Video content with signatures
- `MediaID`: IPFS + Livepeer references
- `CryptoSignature`: Ed25519 verification

#### Ports (`dtt-core/src/ports/`)
- `VideoRepository`: Database abstraction
- `LivepeerAdapter`: Video processing
- `ATClient`: Protocol integration
- `AuthService`: Authentication

#### Use Cases (`dtt-core/src/use_cases/`)
- `publish_video`: Complete upload workflow
- `get_for_you_feed`: Algorithm-driven feed

## Development Setup

### 1. Dependencies
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install SurrealDB
curl -sSf https://install.surrealdb.com | sh

# Start IPFS
ipfs daemon
```

### 2. Database Schema
```sql
-- SurrealDB schema
DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD handle ON TABLE users TYPE string;

DEFINE TABLE posts SCHEMAFULL;
DEFINE FIELD user_id ON TABLE posts TYPE record(users);
DEFINE FIELD media_id ON TABLE posts TYPE object;
DEFINE FIELD signature ON TABLE posts TYPE option<object>;
DEFINE FIELD timestamp ON TABLE posts TYPE number;

DEFINE TABLE posted SCHEMAFULL;
DEFINE FIELD in ON TABLE posted TYPE record(users);
DEFINE FIELD out ON TABLE posted TYPE record(posts);
```

### 3. Environment Setup
```bash
# .env file
SURREAL_URL=ws://localhost:8000
JWT_SECRET=your-256-bit-secret
LIVEPEER_API_KEY=your-livepeer-key
IPFS_GATEWAY=http://localhost:5001
AT_PDS_URL=https://bsky.social
AT_DID=did:plc:your-identifier
```

## Implementation Details

### Cryptographic Signatures
```rust
use dtt_core::domain::{KeyPair, verify_signature};

// Generate keypair
let keypair = KeyPair::generate();

// Sign data
let signature = keypair.sign(b"video content");

// Verify signature
let is_valid = verify_signature(b"video content", &signature);
```

### Video Upload Flow
```rust
// 1. Upload to IPFS
let ipfs_hash = ipfs_client.add(video_data).await?;

// 2. Create Livepeer job
let job_id = livepeer.transcode(ipfs_hash).await?;

// 3. Create signed post
let post = Post {
    media_id: MediaID { ipfs_hash, livepeer_job_id: Some(job_id) },
    signature: Some(keypair.sign(&post_data)),
    // ...
};

// 4. Save to database
repo.save_post(post).await?;

// 5. Publish to AT Protocol
at_client.publish_to_at(&post).await?;
```

### Feed Algorithm
```sql
-- SurrealDB graph query for personalized feed
SELECT 
    id AS post_id,
    (
        (count(<-likes) * 1.0 + count(<-comments) * 2.0 + count(<-shares) * 3.0) / 
        math::max(1, time::unix($now) - time::unix(timestamp))
    ) AS engagement_rate,
    math::exp(-0.1 * (time::unix($now) - time::unix(timestamp)) / 3600) AS recency_score,
    (
        count((SELECT <-follows FROM users WHERE id = $parent.user_id)) * 0.1 +
        (SELECT verified FROM users WHERE id = $parent.user_id LIMIT 1)[0] * 10
    ) AS creator_score
FROM posts 
WHERE timestamp > (time::unix($now) - 604800)
ORDER BY (engagement_rate * 0.4 + recency_score * 0.3 + creator_score * 0.2) DESC
```

## Testing

### Unit Tests
```bash
# Test core domain logic
cargo test -p dtt-core

# Test infrastructure
cargo test -p dtt-infra

# Test API handlers
cargo test -p dtt-api
```

### Integration Tests
```bash
# Start test environment
docker-compose -f docker-compose.test.yml up -d

# Run integration tests
cargo test --test integration
```

## Deployment

### Docker
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/dtt-api /usr/local/bin/
CMD ["dtt-api"]
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: decentik
spec:
  replicas: 3
  selector:
    matchLabels:
      app: decentik
  template:
    spec:
      containers:
      - name: api
        image: decentik:latest
        env:
        - name: SURREAL_URL
          value: "ws://surrealdb:8000"
```

## Contributing

### Code Style
```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Check
cargo check
```

### Adding Features

1. **Domain First**: Add models to `dtt-core/domain/`
2. **Define Ports**: Create abstractions in `dtt-core/ports/`
3. **Implement Infrastructure**: Add adapters in `dtt-infra/`
4. **Create Use Cases**: Business logic in `dtt-core/use_cases/`
5. **Add API Handlers**: REST endpoints in `dtt-api/handlers/`

### Performance Optimization

- Use connection pooling for SurrealDB
- Implement caching for feed algorithm
- Add CDN for IPFS content
- Optimize graph queries with indexes
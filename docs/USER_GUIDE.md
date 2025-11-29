# User Guide

## Getting Started

### 1. Create Account
```bash
curl -X POST http://localhost:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "handle": "your_username",
    "password": "secure_password",
    "display_name": "Your Name"
  }'
```

### 2. Login
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "handle": "your_username",
    "password": "secure_password"
  }'
```

Save the returned `token` for authenticated requests.

### 3. Upload Video
```bash
curl -X POST http://localhost:3000/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "your-user-id",
    "video_data": [/* base64 encoded video */],
    "caption": "My first decentralized video!"
  }'
```

### 4. View Feed
```bash
curl -X GET http://localhost:3000/feed \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## Features

### Cryptographic Signatures
Every post is cryptographically signed, ensuring:
- Content authenticity
- Tamper detection
- Creator verification

### Decentralized Storage
Videos are stored on:
- **IPFS**: Permanent, distributed storage
- **Livepeer**: Professional video processing
- **AT Protocol**: Cross-platform compatibility

### Smart Feed Algorithm
The feed uses graph-based scoring:
- **Engagement Rate**: Likes, comments, shares
- **Recency Score**: Time-based decay
- **Creator Score**: Follower count, verification
- **Social Network**: Friends and followers boost

## Privacy & Security

- **End-to-End Verification**: Cryptographic signatures
- **Decentralized**: No single point of failure
- **Open Protocol**: AT Protocol compatibility
- **Self-Sovereign**: Own your data and identity

## Troubleshooting

### Authentication Issues
- Check JWT token expiration
- Verify credentials
- Ensure proper headers

### Upload Failures
- Check video file size
- Verify IPFS connectivity
- Confirm Livepeer API key

### Feed Not Loading
- Check authentication
- Verify database connection
- Review algorithm logs
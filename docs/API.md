# API Reference

## Authentication

All endpoints except `/auth/*` require JWT token in Authorization header:
```
Authorization: Bearer <jwt_token>
```

## Endpoints

### POST /auth/register
Register new user account.

**Request:**
```json
{
  "handle": "username",
  "password": "secure_password",
  "display_name": "Display Name"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "user_id": "uuid"
}
```

### POST /auth/login
Authenticate existing user.

**Request:**
```json
{
  "handle": "username",
  "password": "secure_password"
}
```

**Response:**
```json
{
  "token": "jwt_token_here",
  "user_id": "uuid"
}
```

### POST /upload
Upload video content with cryptographic signature.

**Request:**
```json
{
  "user_id": "uuid",
  "video_data": [/* base64 encoded bytes */],
  "caption": "Video description"
}
```

**Response:**
```json
{
  "post_id": "uuid",
  "signature": "base64_signature",
  "at_uri": "at://did:plc:example/app.bsky.feed.post/uuid"
}
```

### GET /feed
Get personalized video feed using graph algorithm.

**Response:**
```json
{
  "posts": [
    {
      "id": "uuid",
      "user_id": "uuid",
      "media_id": {
        "ipfs_hash": "QmHash123",
        "livepeer_job_id": "job_456"
      },
      "caption": "Video description",
      "signature": {
        "signature": [/* bytes */],
        "public_key": [/* bytes */]
      },
      "timestamp": 1640995200
    }
  ],
  "algorithm_version": "v1.0-graph-engagement"
}
```

### GET /profile/:id
Get user profile information.

**Response:**
```json
{
  "user_id": "uuid",
  "handle": "username",
  "display_name": "Display Name"
}
```

## Error Responses

### 400 Bad Request
```json
{
  "error": "Invalid request format"
}
```

### 401 Unauthorized
```json
{
  "error": "Authentication required"
}
```

### 500 Internal Server Error
```json
{
  "error": "Server error occurred"
}
```

## Rate Limits

- Authentication: 10 requests/minute
- Upload: 5 requests/minute
- Feed: 60 requests/minute
- Profile: 100 requests/minute

## WebSocket Events (Future)

### Video Processing Status
```json
{
  "type": "video_processing",
  "post_id": "uuid",
  "status": "transcoding|completed|failed",
  "progress": 75
}
```

### Real-time Engagement
```json
{
  "type": "engagement",
  "post_id": "uuid",
  "action": "like|comment|share",
  "count": 42
}
```
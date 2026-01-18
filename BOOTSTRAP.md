# DePIN-Orcha Bootstrap Guide

## Creating the First Admin API Key

### Quick Start

1. **Run the bootstrap script:**

   ```bash
   python create_admin_key.py
   ```

2. **Save the displayed API key securely:**

   ```
   ============================================================
   ✅ Admin API Key Created Successfully!
   ============================================================
   Key ID: 2
   API Key: dpn_85d625022084426a9b49f47d49547db4
   Name: Admin Bootstrap Key
   Rate Limit: 1000 requests/minute
   Permissions: read, write, admin, delete

   Usage: Add this header to your API requests:
     X-API-Key: dpn_85d625022084426a9b49f47d49547db4

   ⚠️  IMPORTANT: Save this key securely - it cannot be retrieved later!
   ============================================================
   ```

3. **Test the key:**

   ```powershell
   # PowerShell
   $headers = @{"X-API-Key" = "dpn_85d625022084426a9b49f47d49547db4"}
   Invoke-RestMethod -Uri "http://localhost:8080/api/v1/admin/keys" -Headers $headers
   ```

   ```bash
   # Bash/curl
   curl http://localhost:8080/api/v1/admin/keys \
     -H "X-API-Key: dpn_85d625022084426a9b49f47d49547db4"
   ```

## Technical Details

### Hashing Algorithm

- **Algorithm**: bcrypt
- **Cost Factor**: 12 (adjustable in script)
- **Salt**: Automatically generated per key
- **Hash Length**: 60 characters (bcrypt standard)

### Default Permissions

- **read**: Access metrics, status, and health endpoints
- **write**: Execute resource allocations and configurations
- **admin**: Manage API keys and system settings
- **delete**: Remove resources and API keys

### Rate Limiting

- **Default**: 1000 requests/minute (configurable)
- **Enforcement**: Per-key, sliding window algorithm
- **429 Response**: Includes `Retry-After` header

## Security Best Practices

### Key Storage

1. Store in secure password manager (1Password, LastPass, etc.)
2. Never commit to version control
3. Use environment variables in production:
   ```bash
   export DEPIN_ADMIN_KEY="dpn_..."
   ```

### Key Management

1. **Create limited keys for daily use:**

   ```bash
   curl -X POST http://localhost:8080/api/v1/admin/keys \
     -H "X-API-Key: $ADMIN_KEY" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "Production Key",
       "permissions": ["read", "write"],
       "rate_limit_per_minute": 60,
       "expires_at": "2026-12-31T23:59:59Z"
     }'
   ```

2. **Rotate bootstrap key after setup:**

   ```bash
   # Create new admin key
   NEW_KEY=$(curl -X POST ... | jq -r '.data.api_key')

   # Delete old bootstrap key
   curl -X DELETE http://localhost:8080/api/v1/admin/keys/2 \
     -H "X-API-Key: $NEW_KEY"
   ```

3. **Set expiration dates:**
   - Development keys: 30 days
   - Production keys: 90 days
   - Admin keys: 180 days

### Permission Model

```
┌─────────────────────────────────────────────┐
│  read                                       │
│  ├─ /api/v1/health                         │
│  ├─ /api/v1/status                         │
│  ├─ /api/v1/metrics                        │
│  └─ /api/v1/allocation/current            │
├─────────────────────────────────────────────┤
│  write                                      │
│  ├─ /api/v1/allocation                     │
│  ├─ /api/v1/allocation/reallocate         │
│  └─ /api/v1/alerts/acknowledge            │
├─────────────────────────────────────────────┤
│  admin                                      │
│  ├─ /api/v1/admin/keys                    │
│  ├─ /api/v1/admin/keys/{id}               │
│  └─ System configuration                   │
├─────────────────────────────────────────────┤
│  delete                                     │
│  ├─ /api/v1/admin/keys/{id}               │
│  └─ Resource cleanup                       │
└─────────────────────────────────────────────┘
```

## Creating Additional Keys

### Read-Only Key (Monitoring)

```bash
curl -X POST http://localhost:8080/api/v1/admin/keys \
  -H "X-API-Key: $ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Monitoring Service",
    "permissions": ["read"],
    "rate_limit_per_minute": 120
  }'
```

### Write Key (Automation)

```bash
curl -X POST http://localhost:8080/api/v1/admin/keys \
  -H "X-API-Key: $ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Automation Script",
    "permissions": ["read", "write"],
    "rate_limit_per_minute": 60,
    "expires_at": "2026-06-30T23:59:59Z"
  }'
```

### Admin Key (Backup)

```bash
curl -X POST http://localhost:8080/api/v1/admin/keys \
  -H "X-API-Key: $ADMIN_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Backup Admin Key",
    "permissions": ["read", "write", "admin", "delete"],
    "rate_limit_per_minute": 1000,
    "expires_at": "2027-01-01T00:00:00Z"
  }'
```

## Troubleshooting

### "Invalid API key" Error

**Cause**: Key not found or incorrect

**Solutions**:

1. Verify key starts with `dpn_`
2. Check database has key with `is_active=1`:
   ```sql
   SELECT * FROM api_keys WHERE is_active = 1;
   ```
3. Ensure bcrypt module installed:
   ```bash
   pip install bcrypt
   ```
4. Verify key hash in database matches bcrypt format (`$2b$12$...`)

### "Rate limit exceeded" Error

**Cause**: Too many requests in time window

**Solutions**:

1. Wait 60 seconds for rate limit reset
2. Check current limit:
   ```bash
   curl http://localhost:8080/api/v1/admin/keys/{id} \
     -H "X-API-Key: $ADMIN_KEY"
   ```
3. Increase rate limit:
   ```bash
   curl -X PUT http://localhost:8080/api/v1/admin/keys/{id} \
     -H "X-API-Key: $ADMIN_KEY" \
     -H "Content-Type: application/json" \
     -d '{"rate_limit_per_minute": 2000}'
   ```
4. Use bootstrap key (1000 req/min) for testing

### 404 Errors on Admin Endpoints

**Cause**: Server not running or routes not configured

**Solutions**:

1. Verify server running:
   ```bash
   curl http://localhost:8080/api/v1/health
   ```
2. Check logs for route configuration:
   ```
   [INFO] All API routes configured successfully
   [INFO]    Admin: /api/v1/admin/keys
   ```
3. Ensure using full path: `/api/v1/admin/keys` (not `/admin/keys`)
4. Rebuild if routes were recently changed:
   ```bash
   cargo build --bin depin-orcha
   ```

### Authentication Working but Permission Denied

**Cause**: Key lacks required permissions

**Solutions**:

1. Check key permissions:
   ```bash
   curl http://localhost:8080/api/v1/admin/keys/{id} \
     -H "X-API-Key: $ADMIN_KEY" | jq '.data.permissions'
   ```
2. Update permissions:
   ```bash
   curl -X PUT http://localhost:8080/api/v1/admin/keys/{id} \
     -H "X-API-Key: $ADMIN_KEY" \
     -H "Content-Type: application/json" \
     -d '{"permissions": ["read", "write", "admin"]}'
   ```

### Key Expired

**Cause**: Key expiration date passed

**Solutions**:

1. Check expiration:
   ```bash
   curl http://localhost:8080/api/v1/admin/keys/{id} \
     -H "X-API-Key: $ADMIN_KEY" | jq '.data.expires_at'
   ```
2. Create new key with longer expiration
3. Update expiration (if key still active):
   ```bash
   curl -X PUT http://localhost:8080/api/v1/admin/keys/{id} \
     -H "X-API-Key: $ADMIN_KEY" \
     -H "Content-Type: application/json" \
     -d '{"expires_at": "2027-12-31T23:59:59Z"}'
   ```

## Database Schema

```sql
CREATE TABLE api_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key_hash TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    last_used_at DATETIME,
    is_active INTEGER NOT NULL DEFAULT 1,
    rate_limit_per_minute INTEGER NOT NULL DEFAULT 60,
    permissions TEXT NOT NULL DEFAULT '["read"]'
);

CREATE INDEX idx_api_keys_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_active ON api_keys(is_active);
```

## Authentication Flow

```
┌─────────────────────────────────────────────┐
│  1. Client sends request with X-API-Key    │
│     header                                  │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│  2. AuthMiddleware extracts key from       │
│     header                                  │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│  3. Query database for active keys         │
│     WHERE is_active = 1                    │
└────────────┬────────────────────────────────┘
             │
             ▼
┌─────────────────────────────────────────────┐
│  4. Loop through keys:                     │
│     - bcrypt::verify(plaintext, hash)     │
│     - Check expiration                     │
│     - Update last_used_at                  │
└────────────┬────────────────────────────────┘
             │
             ├─ Match found ─────┐
             │                    ▼
             │            ┌───────────────────┐
             │            │  5. Return        │
             │            │     ApiKeyInfo    │
             │            └─────────┬─────────┘
             │                      │
             │                      ▼
             │            ┌───────────────────┐
             │            │  6. RateLimit     │
             │            │     Middleware    │
             │            └─────────┬─────────┘
             │                      │
             │                      ▼
             │            ┌───────────────────┐
             │            │  7. Handler       │
             │            │     executes      │
             │            └───────────────────┘
             │
             └─ No match ──────┐
                                ▼
                      ┌──────────────────┐
                      │  401 Unauthorized│
                      └──────────────────┘
```

## Performance Metrics

| Operation           | Latency  | Notes                    |
| ------------------- | -------- | ------------------------ |
| Key validation      | 50-100ms | bcrypt verification      |
| Rate limit check    | < 5ms    | In-memory sliding window |
| Database query      | < 10ms   | Indexed lookup           |
| Total auth overhead | < 120ms  | Per request              |

## Advanced Configuration

### Custom Cost Factor

```python
# Higher cost = more secure but slower
key_hash = bcrypt.hashpw(api_key.encode(), bcrypt.gensalt(14))
```

### Batch Key Creation

```python
import bcrypt
import sqlite3
import uuid

def create_multiple_keys(count=10):
    keys = []
    for i in range(count):
        api_key = f"dpn_{uuid.uuid4().hex}"
        key_hash = bcrypt.hashpw(api_key.encode(), bcrypt.gensalt(12))
        keys.append((key_hash.decode(), f"Auto Key {i+1}"))

    conn = sqlite3.connect("depin_orcha.db")
    cursor = conn.cursor()
    cursor.executemany("""
        INSERT INTO api_keys (key_hash, name, permissions)
        VALUES (?, ?, '["read"]')
    """, keys)
    conn.commit()
    return keys
```

### Key Usage Analytics

```sql
-- Most used keys
SELECT name, last_used_at, rate_limit_per_minute
FROM api_keys
WHERE is_active = 1
ORDER BY last_used_at DESC
LIMIT 10;

-- Keys by permission
SELECT permissions, COUNT(*) as count
FROM api_keys
WHERE is_active = 1
GROUP BY permissions;

-- Expired keys cleanup
DELETE FROM api_keys
WHERE expires_at < datetime('now')
AND is_active = 0;
```

## Production Checklist

- [ ] Bootstrap key created and securely stored
- [ ] Additional admin backup key created
- [ ] Limited permission keys created for services
- [ ] All keys have expiration dates set
- [ ] Rate limits appropriate for use case
- [ ] Bootstrap key rotated after initial setup
- [ ] Key management procedures documented
- [ ] Monitoring alerts configured for auth failures
- [ ] Regular key audit schedule established
- [ ] Backup of api_keys table automated

## Support

For issues or questions:

1. Check server logs: `tail -f depin_orcha.log`
2. Review this documentation
3. Run test suite: `./test_auth.ps1`
4. Open GitHub issue with:
   - Error messages (redacted keys)
   - Server logs
   - Request/response examples
   - Environment details

---

**Last Updated**: 2026-01-16  
**Version**: 1.0.0  
**Status**: Production Ready ✅

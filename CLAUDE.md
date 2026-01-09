# CLAUDE.md - cnctd_redis

> Brief reference for the Redis wrapper library.

## Purpose

Singleton-pattern Redis wrapper with connection pooling and automatic JSON serialization for type-safe key-value operations.

## Key Exports

```rust
pub struct CnctdRedis;

impl CnctdRedis {
    pub fn start(redis_url: &str) -> Result<()>;
    pub fn get_pool() -> Result<PooledConnection>;
    pub fn set<T: Serialize>(key: &str, value: T) -> Result<()>;
    pub fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>>;
    pub fn publish(channel: &str, message: &str) -> Result<()>;
    pub fn hset<T: Serialize>(key: &str, field: &str, value: T) -> Result<()>;
    pub fn check_connection() -> bool;
}
```

## Usage Example

```rust
use cnctd_redis::CnctdRedis;

// Initialize (once at startup)
CnctdRedis::start("redis://localhost:6379")?;

// Type-safe operations
CnctdRedis::set("user:123", &user_data)?;
let user: Option<User> = CnctdRedis::get("user:123")?;
```

## Ecosystem Role

- **Used by**: cnctd_server (WebSocket client state), cnctd.world
- **Features**: Connection pooling via r2d2, auto JSON serialization

## Version

**0.1.9**

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*

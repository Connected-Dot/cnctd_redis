use redis::{Client, RedisResult};
use r2d2_redis::RedisConnectionManager;
use r2d2::{Pool, PooledConnection}; 
use state::InitCell;
pub struct Redis;

type RedisPool = Pool<RedisConnectionManager>;

static REDIS_POOL: InitCell<Option<RedisPool>> = InitCell::new();

impl Redis {
    pub fn start_pool(redis_url: &str) -> anyhow::Result<RedisPool> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager)?;
        REDIS_POOL.set(Some(pool.clone())); 

        Ok(pool)
    }

    pub fn get_pool() -> anyhow::Result<RedisPool> {
        REDIS_POOL.get().clone().ok_or_else(|| anyhow::anyhow!("Redis pool not initialized"))
    }

    pub fn get_client() -> anyhow::Result<PooledConnection<RedisConnectionManager>> {
        let pool = Self::get_pool()?;
        let conn = pool.get()?;


        Ok(conn)
    }
}
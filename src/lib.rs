use redis::{Client, RedisResult};
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use r2d2::{Pool, PooledConnection}; 
use serde::{de::DeserializeOwned, Serialize};
use state::InitCell;
pub struct CnctdRedis;

type RedisPool = Pool<RedisConnectionManager>;

static REDIS_POOL: InitCell<Option<RedisPool>> = InitCell::new();

impl CnctdRedis {
    pub fn start_pool(redis_url: &str) -> anyhow::Result<RedisPool> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager)?;
        REDIS_POOL.set(Some(pool.clone())); 

        Ok(pool)
    }

    pub fn get_client() -> anyhow::Result<PooledConnection<RedisConnectionManager>> {
        let pool = REDIS_POOL.get().clone().ok_or_else(|| anyhow::anyhow!("Redis pool not initialized"))?;
        let conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;

        Ok(conn)
    }

    pub fn set<S>(key: &str, value: S) -> anyhow::Result<()> 
    where S: Serialize + std::fmt::Debug + DeserializeOwned + Send + Sync + Clone + 'static{
        let mut client = Self::get_client()?;
        let value = serde_json::to_string(&value)?;

        client.set(key, value)?;

        Ok(())
    }

    pub fn get<V>(key: &str) -> anyhow::Result<V>
    where
        V: DeserializeOwned + std::fmt::Debug + Send + Sync + Clone + 'static,
    {
        let mut client = Self::get_client()?;
    
        let value_str: String = client.get(key)?;
        let value: V = serde_json::from_str(&value_str)?;
    
        Ok(value)
    }

    pub fn publish<S>(channel: &str, message: S) -> anyhow::Result<()>
    where S: Serialize + std::fmt::Debug + DeserializeOwned + Send + Sync + Clone + 'static {
        let mut client = Self::get_client()?;
        let message = serde_json::to_string(&message)?;

        client.publish(channel, message)?;

        Ok(())
    }

    pub fn hset<S>(key: &str, field: &str, value: S) -> anyhow::Result<()>
    where S: Serialize + std::fmt::Debug + DeserializeOwned + Send + Sync + Clone + 'static {
        let mut client = Self::get_client()?;
        let value = serde_json::to_string(&value)?;

        client.hset(key, field, value.clone())?;
        client.publish(key, value)?;

        Ok(())
    }

}
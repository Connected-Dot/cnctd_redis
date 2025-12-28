use r2d2::{Pool, PooledConnection};
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use redis::{Client, ConnectionLike, RedisResult};
use serde::{de::DeserializeOwned, Serialize};
use state::InitCell;
pub struct CnctdRedis;

type RedisPool = Pool<RedisConnectionManager>;

static REDIS_POOL: InitCell<RedisPool> = InitCell::new();

impl CnctdRedis {
    pub fn start(redis_url: &str) -> anyhow::Result<()> {
        let manager = RedisConnectionManager::new(redis_url)?;
        let pool = Pool::builder().build(manager)?;
        Self::check_connection(redis_url)?;

        REDIS_POOL.set(pool);

        Ok(())
    }

    pub fn get_pool() -> anyhow::Result<PooledConnection<RedisConnectionManager>> {
        let pool = REDIS_POOL.get();
        let conn = pool.get().map_err(|e| anyhow::anyhow!(e))?;

        Ok(conn)
    }

    pub fn set<S>(key: &str, value: S) -> anyhow::Result<()>
    where
        S: Serialize + std::fmt::Debug + DeserializeOwned + Send + Sync + Clone + 'static,
    {
        let mut client = Self::get_pool()?;
        let value = serde_json::to_string(&value)?;

        let _: () = client.set(key, value)?;

        Ok(())
    }

    pub fn get<V>(key: &str) -> anyhow::Result<V>
    where
        V: DeserializeOwned + std::fmt::Debug + Send + Sync + Clone + 'static,
    {
        let mut client = Self::get_pool()?;

        let value_str: String = client.get(key)?;
        let value: V = serde_json::from_str(&value_str)?;

        Ok(value)
    }

    pub fn publish<S>(channel: &str, message: S) -> anyhow::Result<()>
    where
        S: Serialize + std::fmt::Debug + DeserializeOwned + Send + Sync + Clone + 'static,
    {
        let mut client = Self::get_pool()?;
        let message = serde_json::to_string(&message)?;

        let _: () = client.publish(channel, message)?;

        Ok(())
    }

    pub fn hset<S>(key: &str, field: &str, value: S) -> anyhow::Result<()>
    where
        S: Serialize + std::fmt::Debug + DeserializeOwned + Send + Sync + Clone + 'static,
    {
        let mut client = Self::get_pool()?;
        let value = serde_json::to_string(&value)?;

        let _: () = client.hset(key, field, value.clone())?;
        let _: () = client.publish(key, value)?;

        Ok(())
    }

    pub fn check_connection(redis_url: &str) -> anyhow::Result<()> {
        let client = Client::open(redis_url)?;
        let mut con = client.get_connection()?;
        match con.check_connection() {
            true => {
                println!("Connected to Redis");
                Ok(())
            }
            false => Err(anyhow::anyhow!("Failed to connect to Redis")),
        }
    }
}

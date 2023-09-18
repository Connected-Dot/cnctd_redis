use redis::{Connection, Commands};

pub struct RedisStore {}

impl RedisStore {
    pub fn connect() -> Result<Connection, anyhow::Error> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let con = client.get_connection()?;
        Ok(con)
    }

    pub fn set(key: &str, value: &str) -> Result<(), anyhow::Error> {
        let mut connection = Self::connect()?;
        connection.set(key, value)?;
        Ok(())
    }

    pub fn get(key: &str) -> Result<String, anyhow::Error> {
        let mut connection = Self::connect()?;
        let value = connection.get(key)?;
        Ok(value)
    }

    pub fn delete(key: &str) -> Result<(), anyhow::Error> {
        let mut connection = Self::connect()?;
        connection.del(key)?;
        Ok(())
    }
    
}
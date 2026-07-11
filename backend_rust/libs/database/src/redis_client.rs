use redis::{Client, AsyncCommands};
use std::env;

pub type RedisClient = Client;

pub fn create_redis_client() -> Result<RedisClient, redis::RedisError> {
    let redis_url = env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://:rinova_redis_secret@localhost:6379".to_string());

    Client::open(redis_url)
}

pub async fn get_cached<T: serde::de::DeserializeOwned>(
    conn: &mut redis::aio::Connection,
    key: &str,
) -> Result<Option<T>, redis::RedisError> {
    let result: Option<String> = conn.get(key).await?;
    
    match result {
        Some(json) => {
            let value = serde_json::from_str(&json)
                .map_err(|_| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization error")))?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub async fn set_cached<T: serde::Serialize>(
    conn: &mut redis::aio::Connection,
    key: &str,
    value: &T,
    ttl_seconds: usize,
) -> Result<(), redis::RedisError> {
    let json = serde_json::to_string(value)
        .map_err(|_| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization error")))?;
    
    conn.set_ex(key, json, ttl_seconds).await
}

pub async fn delete_cached(
    conn: &mut redis::aio::Connection,
    key: &str,
) -> Result<(), redis::RedisError> {
    conn.del(key).await
}

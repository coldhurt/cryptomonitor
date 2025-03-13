use redis::AsyncCommands;
use redis::Client;
use redis::RedisResult;
use redis::aio::MultiplexedConnection;

struct RedisCli {
    con: MultiplexedConnection,
}

impl RedisCli {
    async fn new(url: &str) -> RedisResult<Self> {
        let client = Client::open(url)?;
        let con = client.get_multiplexed_async_connection().await?;
        Ok(Self { con })
    }

    async fn all_keys(&mut self) -> RedisResult<Vec<String>> {
        let all_keys: Vec<String> = self.con.keys("*").await?;
        println!("all keys: {:?}", all_keys);
        Ok(all_keys)
    }

    async fn all_values(&mut self) -> RedisResult<()> {
        let keys = self.all_keys().await?;
        for k in keys.iter() {
            let rv: String = self.con.get(k).await?;

            println!("{} => {}", k, rv);
        }
        Ok(())
    }
}

async fn test_redis() -> RedisResult<()> {
    let mut client = RedisCli::new("redis://127.0.0.1:6379/").await?;

    // _ = client.all_keys().await?;

    let _: () = client.con.set("test", "test_data").await?;

    client.all_values().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> RedisResult<()> {
    _ = test_redis().await?;
    Ok(())
}

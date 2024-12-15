use crate::{models::{DepthInterval, EarningInterval, RunePoolInterval, SwapsInterval}, services::db_traits::Database};
use surrealdb::engine::remote::ws::{Client , Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::{error::Error, time::Instant};
use async_trait::async_trait;
pub struct SurrealDB {
    client : Surreal<Client>
}

impl SurrealDB {
    pub async fn new(conn_str: &str, username: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let client = Surreal::new::<Ws>(conn_str).await.expect("Error connecting to surreal db");
        client.signin(Root {
            username,
            password,
        }).await?;
        client.use_ns("thor").use_db("mydb").await?;

        Ok(SurrealDB { client })
    }
}
#[async_trait]
impl Database for SurrealDB {
    async fn store_depth_intervals(&self, interval: DepthInterval) -> Result<std::time::Duration, Box<dyn Error>> {
        let start_time = Instant::now();
        
        let _out: Option<DepthInterval> = self.client
        .create("depth_interval")
        .content(interval)
        .await?;

        Ok(start_time.elapsed())
    }

    async fn store_swaps_intervals(&self, swap: SwapsInterval) -> Result<std::time::Duration, Box<dyn Error>> {
        let start_time = Instant::now();

        let _out : Option<SwapsInterval> = self.client
            .create("swaps_interval")
            .content(swap)
            .await?;

        Ok(start_time.elapsed())
    }

    async fn store_earnings_intervals(&self, interval: EarningInterval) -> Result<std::time::Duration, Box<dyn Error>> {
        let start_time = Instant::now();

        let _out : Option<EarningInterval> = self.client
            .create("earning_interval")
            .content(interval)
            .await?;

        Ok(start_time.elapsed())
    }

    async fn store_runepool_intervals(&self, runepool: RunePoolInterval) -> Result<std::time::Duration, Box<dyn Error>> {
        let start_time = Instant::now();

        let _out: Option<RunePoolInterval> = self.client
            .create("rune_pool_interval")
            .content(runepool)
            .await?;

        Ok(start_time.elapsed())
    }

    async fn read_depth_intervals(&self) -> Result<(Vec<DepthInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();

        let result: Vec<DepthInterval> = self.client
            .query("SELECT * FROM depth_interval")
            .await?
            .take(0)?;

        let duration = start_time.elapsed();
        Ok((result, duration))
    }

    async fn read_swaps_intervals(&self) -> Result<(Vec<SwapsInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();

        let result: Vec<SwapsInterval> = self.client
            .query("SELECT * FROM swaps_interval")
            .await?
            .take(0)?;

        let duration = start_time.elapsed();
        Ok((result, duration))
    }

    async fn read_earnings_intervals(&self) -> Result<(Vec<EarningInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();

        let result: Vec<EarningInterval> = self.client
            .query("SELECT * FROM earning_interval")
            .await?
            .take(0)?;

        let duration = start_time.elapsed();
        Ok((result, duration))
    }

    async fn read_runepool_intervals(&self) -> Result<(Vec<RunePoolInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();

        let result: Vec<RunePoolInterval> = self.client
            .query("SELECT * FROM rune_pool_interval")
            .await?
            .take(0)?;

        let duration = start_time.elapsed();
        Ok((result, duration))
    }
}
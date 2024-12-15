use std::{error::Error, time::Instant};
use async_trait::async_trait;
use futures::StreamExt as _;
use mongodb::{Client, Collection};

use crate::{models::{DepthInterval, EarningInterval, RunePoolInterval, SwapsInterval}, services::db_traits::Database};

pub struct MongoDb {
    client: Client,
    depth_collection: Collection<DepthInterval>,
    swaps_collection: Collection<SwapsInterval>,
    earnings_collection: Collection<EarningInterval>,
    rune_collection: Collection<RunePoolInterval>,
}

impl MongoDb {
    pub async fn new(uri: &str , db_name: &str) -> Result<Self , Box<dyn Error>> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(db_name);

        let depth_collection = db.collection::<DepthInterval>("depth_intervals");
        let swaps_collection = db.collection::<SwapsInterval>("swaps_intervals");
        let earnings_collection = db.collection::<EarningInterval>("earnings_intervals");
        let rune_collection = db.collection::<RunePoolInterval>("rune_intervals");

        Ok(Self {
            client,
            depth_collection,
            swaps_collection,
            earnings_collection,
            rune_collection,
        })
    }
}

#[async_trait]
impl Database for MongoDb {
    async fn store_depth_intervals(&self , interval: DepthInterval) -> Result<std::time::Duration , Box<dyn Error>>{
        let start_time = Instant::now();
        self.depth_collection.insert_one(interval).await.expect("Couldn't insert into depth_intervals");
        let duration = start_time.elapsed();
        Ok(duration) 
    }
    async fn store_swaps_intervals(&self , interval: SwapsInterval) -> Result<std::time::Duration , Box<dyn Error>>{
        let start_time = Instant::now();
        self.swaps_collection.insert_one(interval).await.expect("Couldn't insert into swaps_intervals");
        let duration = start_time.elapsed(); 
        Ok(duration)
    }
    async fn store_earnings_intervals(&self , interval: EarningInterval) -> Result<std::time::Duration , Box<dyn Error>>{
        let start_time = Instant::now();
        self.earnings_collection.insert_one(interval).await.expect("Couldn't insert into earnings_intervals");
        let duration = start_time.elapsed();
        Ok(duration)
    }
    async fn store_runepool_intervals(&self , interval: RunePoolInterval) -> Result<std::time::Duration , Box<dyn Error>>{
        let start_time = Instant::now();
        self.rune_collection.insert_one(interval).await.expect("Couldn't insert into rune_intervals");
        let duration = start_time.elapsed();
        Ok(duration)
    }
    async fn read_depth_intervals(&self) -> Result<(Vec<DepthInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let cursor = self.depth_collection.find(None).await?;
        let depth_intervals: Vec<DepthInterval> = cursor
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect(); 

        let duration = start_time.elapsed();
        Ok((depth_intervals, duration))
    }
    async fn read_swaps_intervals(&self) -> Result<(Vec<SwapsInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let cursor = self.swaps_collection.find(None).await?;
        let swaps_intervals: Vec<SwapsInterval> = cursor
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect(); 

        let duration = start_time.elapsed();
        Ok((swaps_intervals, duration))
    }
    async fn read_earnings_intervals(&self) -> Result<(Vec<EarningInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let cursor = self.earnings_collection.find(None).await?;
        let earnings_intervals: Vec<EarningInterval> = cursor
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect(); 

        let duration = start_time.elapsed();
        Ok((earnings_intervals, duration))
    }
    async fn read_runepool_intervals(&self) -> Result<(Vec<RunePoolInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let cursor = self.rune_collection.find(None).await?;
        let rune_pool_intervals: Vec<RunePoolInterval> = cursor
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect(); 

        let duration = start_time.elapsed();
        Ok((rune_pool_intervals, duration))
    }
}
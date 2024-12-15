use std::env;

use api::api_fetcher::{fetch_depth_data, fetch_earnings_data, fetch_runepool_data, fetch_swaps_data, IntervalParams};
use dotenv::dotenv;
use services::db_factory::{match_database_type, DatabaseFactory}; 
mod models;
mod api;
mod services;
mod repositories;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let mongo_db_name = String::from("thor_api");
    // let mongodb_url = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    // let db = match match_database_type("mongodb",&[mongodb_url , mongo_db_name]) {
    //     Ok(db_type) => DatabaseFactory::create(db_type).await,
    //     Err(e) => {
    //         eprintln!("Failed to create mongodb database: {}", e);
    //         return;
    //     }
    // };

    // let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    // let db = match match_database_type("postgres",&[postgres_url]) {
    //     Ok(db_type) => DatabaseFactory::create(db_type).await,
    //     Err(e) => {
    //         eprintln!("Failed to create postgres database: {}", e);
    //         return;
    //     }
    // };

    let surreal_url = env::var("SURREALDB_URL").expect("SURREALDB_URL must be set");
    let surreal_username = env::var("SURREALDB_USERNAME").expect("SURREALDB_USERNAME must be set");
    let surreal_password = env::var("SURREALDB_PASSWORD").expect("SURREALDB_PASSWORD must be set");

    let db = match match_database_type("surrealdb", &[surreal_url, surreal_username, surreal_password]) {
        Ok(db_type) => DatabaseFactory::create(db_type).await,
        Err(e) => {
            eprintln!("Failed to create SurrealDB database: {}", e);
            return;
        }
    };

    if let Err(e) = db {
        eprintln!("Failed to connect to the database: {}", e);
        return;
    }
    let db = db.unwrap();

    println!("DATABASE CONNECTED SUCCESSFULLY!");

    let params = IntervalParams {
        from: 1726758000,
        count: 1,
        interval: "hour".to_string(),
    };

    // Fetch depth data
    let depth_data = match fetch_depth_data(&params, "BTC.BTC").await {
        Ok(depth_data) => depth_data,
        Err(e) => {eprintln!("Failed to fetch depth data: {}", e); return;},
    };

    for interval in depth_data {
        let duration = db.store_depth_intervals(interval).await;
        println!("Inserted depth intervals in {:?}",duration);
    }
    println!("DEPTH DATA INSERTED SUCCESSFULLY!");

    // Fetch swaps data
    let swaps_data = match fetch_swaps_data(&params).await {
        Ok(swaps_data) => swaps_data,
        Err(e) => { eprintln!("Failed to fetch swap data: {}", e); return; },
    };

    for interval in swaps_data {
        let duration = db.store_swaps_intervals(interval).await;
        println!("Inserted swap intervals in {:?}",duration);
    }
    println!("SWAP DATA INSERTED SUCCESSFULLY!");

    // Fetch earnings data
    let earnings_data = match fetch_earnings_data(&params).await {
        Ok(earnings_data) => earnings_data,
        Err(e) => { eprintln!("Failed to fetch earnings data: {}", e); return; },
    };

    for interval in earnings_data {
        let duration = db.store_earnings_intervals(interval).await;
        println!("Inserted earnings intervals in {:?}",duration);
    }
    println!("EARNING DATA INSERTED SUCCESSFULLY!");

    // Fetch rune pool data
    let runepool_data = match fetch_runepool_data(&params).await {
        Ok(runepool_data) => runepool_data,
        Err(e) => { eprintln!("Failed to fetch rune pool data: {}", e); return; },
    };

    for interval in runepool_data {
        let duration = db.store_runepool_intervals(interval).await;
        println!("Inserted runepool intervals in {:?}",duration);
    }
    println!("RUNEPOOL DATA INSERTED SUCCESSFULLY!");
}

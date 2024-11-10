use std::error::Error;
use crate::repositories::{mongo_db::MongoDb, postgres_db::PostgresDb};
use super::db_traits::Database;
#[derive(Debug)]
pub enum DbType {
    Postgres(String),
    Mongodb(String,String),
    Rocksdb(String),
    // LevelDb(String),
    // SurrealDb(String)
}

pub fn match_database_type(db_type: &str, args: &[String]) -> Result<DbType, Box<dyn Error>> {
    match db_type {
        "postgres" if args.len() > 0 => {
            let conn_str = &args[0]; 
            Ok(DbType::Postgres(conn_str.clone()))
        },
        "mongodb" if args.len() >= 2 => { // Corrected to check for 2 arguments
            let uri = &args[0]; 
            let db_name = &args[1]; 
            Ok(DbType::Mongodb(uri.clone(), db_name.clone()))
        },
        // "rocksdb" if args.len() > 0 => {
        //     // let path = &args[0]; 
        //     // Ok(DbType::RocksDb(path.clone()))
        // },
        _ => Err("Unsupported or insufficient arguments for the specified database type".into()),
    }
}

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create(db: DbType) -> Result<Box<dyn Database> , Box<dyn Error>> {
        match db {
            DbType::Postgres(conn) => {
                let postgres_db = PostgresDb::new(&conn).await?;
                Ok(Box::new(postgres_db))
            },
            DbType::Mongodb(uri, db_name) => {
                let mongo_db = MongoDb::new(&uri, &db_name).await?;
                Ok(Box::new(mongo_db))
            },
            DbType::Rocksdb(path) => {
                // Placeholder for RocksDB implementation
                // let rocks_db = RocksDb::new(&path)?;
                // Ok(Box::new(rocks_db))
                Err("RocksDB is not implemented".into())
            },
        }
        
    }
}
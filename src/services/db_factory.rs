use std::error::Error;
use crate::repositories::{surreal_db::SurrealDB , postgres_db::PostgresDb , mongo_db::MongoDb};
use super::db_traits::Database;

#[derive(Debug)]
pub enum DbType {
    Postgres(String),
    Mongodb(String, String),
    Rocksdb(String),
    SurrealDb(String, String, String),
}

pub fn match_database_type(db_type: &str, args: &[String]) -> Result<DbType, Box<dyn Error>> {
    match db_type {
        "postgres" if args.len() >= 1 => {
            let conn_str = args[0].clone(); 
            Ok(DbType::Postgres(conn_str))
        },
        "mongodb" if args.len() >= 2 => {
            let uri = args[0].clone();
            let db_name = args[1].clone();
            Ok(DbType::Mongodb(uri, db_name))
        },
        "surrealdb" if args.len() >= 3 => {
            let url = args[0].clone();
            let username = args[1].clone();
            let password = args[2].clone();
            Ok(DbType::SurrealDb(url, username, password))
        },
        "rocksdb" if args.len() >= 1 => {
            let path = args[0].clone();
            Ok(DbType::Rocksdb(path))
        },
        _ => Err("Unsupported or insufficient arguments for the specified database type".into()),
    }
}

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create(db: DbType) -> Result<Box<dyn Database>, Box<dyn Error>> {
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
            DbType::SurrealDb(conn, username, password) => {
                let surreal_db = SurrealDB::new(&conn, &username, &password).await?;
                Ok(Box::new(surreal_db))
            },
        }
    }
}

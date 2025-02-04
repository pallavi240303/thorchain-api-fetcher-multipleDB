# Thorchain Data Fetcher - Modular Database Integration

## Overview
`thorchain_data_fetcher` is a Rust-based backend designed to fetch, process, and store Thorchain blockchain data in multiple databases. The primary aim of this project is to create a highly modular and configurable system that allows seamless switching between different database technologies, ensuring flexibility and scalability for data storage and retrieval.

## Key Features
- **Multi-Database Compatibility**: Supports MongoDB, PostgreSQL, and SurrealDB with minimal configuration changes.
- **Efficient Thorchain Data Processing**: Fetches depth, swaps, earnings, and rune pool data.
- **Highly Configurable**: Environment-driven database selection for effortless adaptability.
- **Asynchronous Execution**: Utilizes `tokio` for optimized concurrency and performance.
- **Robust Error Handling**: Ensures stability through comprehensive database connection and API failure handling.

## Technology Stack
- **Rust** - Core programming language
- **Tokio** - Asynchronous runtime
- **SurrealDB, PostgreSQL, MongoDB** - Supported database systems
- **dotenv** - Environment variable management

## Data Processing Workflow
- **Depth Data**: Extracted and stored in the configured database.
- **Swaps Data**: Captures and persists swap transaction records.
- **Earnings Data**: Retrieves and saves earnings-related information.
- **Rune Pool Data**: Collects and archives rune pool statistics.

## Future Enhancements
- Expansion to additional database systems (RocksDB , levelDB).
- Real-time data streaming implementation.
- Enhanced logging and monitoring using `tracing`.
- Further configuration enhancements for improved flexibility.


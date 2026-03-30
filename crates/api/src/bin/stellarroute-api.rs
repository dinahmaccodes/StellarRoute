//! StellarRoute API Server Binary

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use stellarroute_api::{telemetry, Server, ServerConfig, state::DatabasePools};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Initialize structured logging (reads RUST_LOG and LOG_FORMAT env vars)
    telemetry::init();

    info!("Starting StellarRoute API Server");

    // Get database URLs from environment
    // DATABASE_URL: Primary database for writes and fallback reads
    // REPLICA_DATABASE_URL: Optional replica database for read-only queries to reduce primary load
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/stellarroute".to_string());
    let replica_database_url = std::env::var("REPLICA_DATABASE_URL").ok();

    // Read pool configuration from environment variables
    let max_connections: u32 = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let min_connections: u32 = std::env::var("DB_MIN_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(2);
    let connection_timeout_secs: u64 = std::env::var("DB_CONNECTION_TIMEOUT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(30);
    let idle_timeout_secs: u64 = std::env::var("DB_IDLE_TIMEOUT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(600);
    let max_lifetime_secs: u64 = std::env::var("DB_MAX_LIFETIME")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1800);

    // Create pool options
    let pool_options = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(connection_timeout_secs))
        .idle_timeout(Duration::from_secs(idle_timeout_secs))
        .max_lifetime(Duration::from_secs(max_lifetime_secs));

    info!(
        "Connecting to primary database (pool: min={}, max={}, timeout={}s)...",
        min_connections, max_connections, connection_timeout_secs
    );
    let primary_pool = match pool_options.connect(&database_url).await {
        Ok(pool) => {
            info!(
                "✅ Primary database connection pool established (max_connections={})",
                max_connections
            );
            pool
        }
        Err(e) => {
            error!("❌ Failed to connect to primary database: {}", e);
            std::process::exit(1);
        }
    };

    // Create replica pool if configured
    let replica_pool = if let Some(replica_url) = &replica_database_url {
        info!("Connecting to replica database...");
        match pool_options.connect(replica_url).await {
            Ok(pool) => {
                info!("✅ Replica database connection pool established");
                Some(pool)
            }
            Err(e) => {
                warn!("⚠️ Failed to connect to replica database ({}), falling back to primary for reads", e);
                None
            }
        }
    } else {
        info!("ℹ️ No replica database configured, using primary for all operations");
        None
    };

    let db_pools = DatabasePools::new(primary_pool, replica_pool);

    // Log dual-connection status for testing/verification
    if db_pools.replica.is_some() {
        info!("✅ Dual-connection setup: reads will use replica, writes use primary");
    } else {
        info!("ℹ️ Single-connection setup: all operations use primary");
    }

    // Create server configuration
    let config = ServerConfig {
        host: std::env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: std::env::var("API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000),
        enable_cors: true,
        enable_compression: true,
        redis_url: std::env::var("REDIS_URL").ok(),
        quote_cache_ttl_seconds: std::env::var("QUOTE_CACHE_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2),
    };

    // Create and start server
    let server = Server::new(config, db_pools).await;

    if let Err(e) = server.start().await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}

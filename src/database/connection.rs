use sqlx::{PgPool, Row};
use std::time::Duration;
use anyhow::{Context, Result};

/// Database configuration for healthcare systems
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/hims".to_string()),
            max_connections: 20,
            min_connections: 5,
            connect_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_lifetime: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Database connection pool for healthcare applications
#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create new database connection with healthcare-optimized settings
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        tracing::info!("Connecting to PostgreSQL database for healthcare system");
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.connect_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .test_before_acquire(true) // Ensure connections are healthy
            .connect(&config.database_url)
            .await
            .context("Failed to create database connection pool")?;

        // Verify database connection
        let version: String = sqlx::query("SELECT version()")
            .fetch_one(&pool)
            .await
            .context("Failed to verify database connection")?
            .get(0);
            
        tracing::info!("Connected to database: {}", version);

        Ok(Self { pool })
    }

    /// Get database pool for query execution
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations for healthcare schema
    pub async fn migrate(&self) -> Result<()> {
        tracing::info!("Running database migrations for healthcare schema");
        
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("Failed to run database migrations")?;
            
        tracing::info!("Database migrations completed successfully");
        Ok(())
    }

    /// Health check for database connection
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .context("Database health check failed")?;
        Ok(())
    }

    /// Get database statistics for monitoring
    pub async fn get_stats(&self) -> Result<DatabaseStats> {
        let pool_stats = self.pool.size();
        
        // Get PostgreSQL-specific stats
        let active_connections: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM pg_stat_activity WHERE state = 'active'"
        )
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        let database_size: i64 = sqlx::query_scalar(
            "SELECT pg_database_size(current_database())"
        )
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0);

        Ok(DatabaseStats {
            pool_size: pool_stats,
            active_connections: active_connections as u32,
            database_size_bytes: database_size as u64,
        })
    }

    /// Close database connection pool
    pub async fn close(&self) {
        tracing::info!("Closing database connection pool");
        self.pool.close().await;
    }
}

/// Database connection statistics
#[derive(Debug)]
pub struct DatabaseStats {
    pub pool_size: u32,
    pub active_connections: u32,
    pub database_size_bytes: u64,
}

/// Database transaction wrapper for healthcare operations
pub struct DatabaseTransaction<'a> {
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
}

impl<'a> DatabaseTransaction<'a> {
    /// Begin a new database transaction
    pub async fn begin(db: &'a Database) -> Result<Self> {
        let tx = db.pool.begin().await
            .context("Failed to begin database transaction")?;
        Ok(Self { tx })
    }

    /// Commit the transaction
    pub async fn commit(self) -> Result<()> {
        self.tx.commit().await
            .context("Failed to commit database transaction")?;
        Ok(())
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> Result<()> {
        self.tx.rollback().await
            .context("Failed to rollback database transaction")?;
        Ok(())
    }

    /// Get mutable reference to the transaction
    pub fn as_mut(&mut self) -> &mut sqlx::Transaction<'a, sqlx::Postgres> {
        &mut self.tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 5);
    }
}
//! PostgreSQL Bridge - PostgreSQL with pgvector for Pheno
//!
//! Provides pgvector-compatible vector search with PostgreSQL.
//!
//! # Hexagonal port (D19)
//!
//! `PgBridge` implements the [`pheno_query::QueryPort`] trait so domain
//! code can depend on the trait only. Planning delegates to
//! [`pheno_query::PostgresQueryPlanner`].

use anyhow::Result;
use async_trait::async_trait;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use pheno_data_core::{Dataset, DatasetSchema, Record, Writer};
use pheno_query::{QueryPort, QueryRequest, QueryStatement};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use tokio_postgres::types::Json;
use url::Url;

const DATASET_TABLE: &str = "pheno_dataset_records";

/// Hexagonal Dataset adapter for PostgreSQL.
pub struct PgDataset {
    pool: Pool,
}

impl PgDataset {
    pub async fn connect(dsn: &str) -> Result<Self> {
        let pool = create_pool(dsn)?;
        let dataset = Self { pool };
        dataset.ensure_dataset_table().await?;
        Ok(dataset)
    }

    async fn ensure_dataset_table(&self) -> Result<()> {
        let client = self.pool.get().await?;
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS pheno_dataset_records (
                    id BIGSERIAL PRIMARY KEY,
                    payload JSONB NOT NULL
                )",
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Dataset for PgDataset {
    async fn records(&self) -> Result<Vec<Record>> {
        let client = self.pool.get().await?;
        let rows = client
            .query("SELECT payload FROM pheno_dataset_records ORDER BY id", &[])
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| row.get::<_, serde_json::Value>(0))
            .collect())
    }

    async fn schema(&self) -> Result<DatasetSchema> {
        Ok(serde_json::json!({
            "backend": "postgres",
            "table": DATASET_TABLE,
            "record_format": "jsonb"
        }))
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// Hexagonal Writer adapter for PostgreSQL.
pub struct PgWriter {
    pool: Pool,
}

impl PgWriter {
    pub async fn connect(dsn: &str) -> Result<Self> {
        let pool = create_pool(dsn)?;
        let writer = Self { pool };
        writer.ensure_dataset_table().await?;
        Ok(writer)
    }

    async fn ensure_dataset_table(&self) -> Result<()> {
        let client = self.pool.get().await?;
        client
            .batch_execute(
                "CREATE TABLE IF NOT EXISTS pheno_dataset_records (
                    id BIGSERIAL PRIMARY KEY,
                    payload JSONB NOT NULL
                )",
            )
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Writer for PgWriter {
    async fn write(&self, record: Record) -> Result<()> {
        let client = self.pool.get().await?;
        client
            .execute(
                "INSERT INTO pheno_dataset_records (payload) VALUES ($1)",
                &[&Json(record)],
            )
            .await?;
        Ok(())
    }

    async fn flush(&self) -> Result<()> {
        Ok(())
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// PgBridge - PostgreSQL with pgvector
pub struct PgBridge {
    pool: Pool,
    /// Embedded planner so `QueryPort::plan` is `&self`-callable.
    planner: pheno_query::PostgresQueryPlanner,
}

impl QueryPort for PgBridge {
    fn plan(&self, req: &QueryRequest) -> Result<QueryStatement> {
        self.planner.plan(req)
    }
}

impl PgBridge {
    /// Create new PostgreSQL bridge with connection pool from a connection string.
    /// Supports standard PostgreSQL URI format:
    /// `postgres://user:pass@host:port/dbname?sslmode=require`
    pub async fn new(conn_string: &str) -> Result<Self> {
        let pool = create_pool(conn_string)?;

        Ok(Self {
            pool,
            planner: pheno_query::PostgresQueryPlanner,
        })
    }

    /// Initialize pgvector extension and tables
    pub async fn init_schema(&self) -> Result<()> {
        let client = self.pool.get().await?;

        client
            .batch_execute(
                "CREATE EXTENSION IF NOT EXISTS vector;
             CREATE TABLE IF NOT EXISTS embeddings (
                 id SERIAL PRIMARY KEY,
                 name TEXT NOT NULL,
                 vector VECTOR(1536),
                 metadata JSONB
             );
             CREATE INDEX IF NOT EXISTS embeddings_vector_idx
                 ON embeddings USING HNSW (vector vector_cosine_ops);",
            )
            .await?;

        Ok(())
    }

    /// Store embedding
    pub async fn store_embedding(
        &self,
        name: &str,
        vector: Vec<f32>,
        metadata: serde_json::Value,
    ) -> Result<i32> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO embeddings (name, vector, metadata) VALUES ($1, $2, $3) RETURNING id",
                &[&name, &vector, &Json(metadata)],
            )
            .await?;

        Ok(row.get(0))
    }

    /// Search similar embeddings using pgvector
    pub async fn search_similar(
        &self,
        query: &[f32],
        limit: usize,
    ) -> Result<Vec<EmbeddingResult>> {
        let client = self.pool.get().await?;

        let rows = client
            .query(
                "SELECT id, name, 1 - (vector <=> $1) AS similarity, metadata
             FROM embeddings
             ORDER BY vector <=> $1
             LIMIT $2",
                &[&query, &(limit as i64)],
            )
            .await?;

        let results: Vec<EmbeddingResult> = rows
            .iter()
            .map(|row| EmbeddingResult {
                id: row.get(0),
                name: row.get(1),
                similarity: row.get(2),
                metadata: row.get::<_, Json<serde_json::Value>>(3).0,
            })
            .collect();

        Ok(results)
    }
}

fn create_pool(conn_string: &str) -> Result<Pool> {
    let parsed =
        Url::parse(conn_string).map_err(|e| anyhow::anyhow!("invalid connection string: {}", e))?;

    let mut cfg = Config::new();
    cfg.host = parsed.host_str().map(|h| h.to_string());
    cfg.port = parsed.port();
    cfg.user = if parsed.username().is_empty() {
        None
    } else {
        Some(parsed.username().to_string())
    };
    cfg.password = parsed.password().map(|p| p.to_string());
    cfg.dbname = if parsed.path().trim_start_matches('/').is_empty() {
        None
    } else {
        Some(parsed.path().trim_start_matches('/').to_string())
    };
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    Ok(cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}

/// Embedding search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResult {
    pub id: i32,
    pub name: String,
    pub similarity: f64,
    pub metadata: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires PostgreSQL with pgvector
    async fn test_store_and_search() -> Result<()> {
        let bridge = PgBridge::new("postgres://localhost/pheno").await?;
        bridge.init_schema().await?;

        let id = bridge
            .store_embedding(
                "test",
                vec![0.1; 1536],
                serde_json::json!({"source": "test"}),
            )
            .await?;

        assert!(id > 0);

        let results = bridge.search_similar(&vec![0.1; 1536], 5).await?;
        assert!(!results.is_empty());

        Ok(())
    }
}

//! SurrealDB Bridge - SurrealDB embedded with Pheno extensions
//!
//! Provides embedded SurrealDB with MCP protocol adapter and skill storage.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

/// PhenoSurreal - SurrealDB with extensions
pub struct PhenoSurreal {
    db: Surreal<Db>,
}

impl PhenoSurreal {
    /// Create new embedded SurrealDB
    pub async fn new(path: impl Into<String>) -> Result<Self> {
        let db = Surreal::new::<RocksDb>(path.into()).await?;
        db.use_ns("pheno").use_db("main").await?;
        Ok(Self { db })
    }

    /// Store a skill with versioning
    pub async fn store_skill(&self, skill: Skill) -> Result<String> {
        let id = format!("skill_{}", generate_id());
        let data = serde_json::to_value(skill)?;
        self.db
            .query("CREATE skill CONTENT $data")
            .bind(("data", data))
            .await?;
        Ok(id)
    }

    /// Query skills
    pub async fn query_skills(&self) -> Result<Vec<Skill>> {
        let mut result = self.db.query("SELECT * FROM skill").await?;
        let records: Vec<serde_json::Value> = result.take(0)?;
        let skills = records
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(skills)
    }

    /// Store vector embedding
    pub async fn store_embedding(&self, embedding: Embedding) -> Result<String> {
        let id = format!("embedding_{}", generate_id());
        let data = serde_json::to_value(embedding)?;
        self.db
            .query("CREATE embedding CONTENT $data")
            .bind(("data", data))
            .await?;
        Ok(id)
    }

    /// Search similar embeddings
    pub async fn search_similar(
        &self,
        query: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<ScoredEmbedding>> {
        let mut result = self
            .db
            .query(
                "SELECT *, vector::distance::cosine(embedding, $query) AS score \
                 FROM embedding ORDER BY score ASC LIMIT $limit",
            )
            .bind(("query", query))
            .bind(("limit", limit))
            .await?;
        let records: Vec<serde_json::Value> = result.take(0)?;
        let embeddings = records
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(embeddings)
    }
}

/// Generate a random ID
fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{:x}-{:x}", now, counter)
}

/// Skill record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub code: String,
    pub runtime: String,
    pub metadata: serde_json::Value,
}

impl Skill {
    pub fn new(name: String, version: String, code: String, runtime: String) -> Self {
        Self {
            id: None,
            name,
            version,
            code,
            runtime,
            metadata: serde_json::json!({}),
        }
    }
}

/// Embedding record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub id: Option<String>,
    pub vector: Vec<f32>,
    pub metadata: serde_json::Value,
}

/// Scored embedding result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredEmbedding {
    pub id: String,
    pub embedding: Vec<f32>,
    pub metadata: serde_json::Value,
    pub score: f64,
}

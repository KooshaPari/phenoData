//! SurrealDB Bridge - SurrealDB embedded with Pheno extensions
//!
//! Provides embedded SurrealDB with MCP protocol adapter and skill storage.

use anyhow::Result;
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
    pub async fn store_skill(&self, skill: Skill) -> Result<RecordId> {
        let result = self.db.create("skill").content(skill).await?;
        Ok(result)
    }

    /// Query skills
    pub async fn query_skills(&self) -> Result<Vec<Skill>> {
        let skills: Vec<Skill> = self.db.select("skill").await?;
        Ok(skills)
    }

    /// Store vector embedding
    pub async fn store_embedding(&self, embedding: Embedding) -> Result<RecordId> {
        let result = self.db.create("embedding").content(embedding).await?;
        Ok(result)
    }

    /// Search similar embeddings
    pub async fn search_similar(&self, query: &[f32], limit: usize) -> Result<Vec<ScoredEmbedding>> {
        // Use SurrealDB's vector search
        let results: Vec<ScoredEmbedding> = self.db
            .query("SELECT *, vector::distance::cosine(embedding, $query) AS score FROM embedding ORDER BY score ASC LIMIT $limit")
            .bind(("query", query))
            .bind(("limit", limit))
            .await?
            .take(0)?;
        
        Ok(results)
    }
}

/// Skill record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Option<RecordId>,
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
    pub id: Option<RecordId>,
    pub vector: Vec<f32>,
    pub metadata: serde_json::Value,
}

/// Scored embedding result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredEmbedding {
    pub id: RecordId,
    pub embedding: Vec<f32>,
    pub metadata: serde_json::Value,
    pub score: f32,
}

/// SurrealDB record ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordId {
    pub tb: String,
    pub id: surrealdb::sql::Thing,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_create_skill() -> Result<()> {
        let dir = tempdir()?;
        let db = PhenoSurreal::new(dir.path().join("test.db")).await?;
        
        let skill = Skill::new(
            "test-skill".to_string(),
            "1.0.0".to_string(),
            "fn main() {}".to_string(),
            "wasm".to_string(),
        );
        
        let id = db.store_skill(skill).await?;
        assert!(id.tb == "skill");
        
        Ok(())
    }
}

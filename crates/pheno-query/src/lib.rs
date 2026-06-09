//! PhenoQuery - Unified query builder for Pheno
//!
//! Provides a unified query interface across different data stores.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

/// Parameterized query statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryStatement {
    pub sql: String,
    pub params: HashMap<String, serde_json::Value>,
}

impl QueryStatement {
    /// Add a positional parameter
    pub fn param(mut self, key: &str, value: impl Into<serde_json::Value>) -> Self {
        self.params.insert(key.to_string(), value.into());
        self
    }
}

/// Query port for hexagonal architecture
pub trait QueryPort {
    fn plan(&self, req: &QueryRequest) -> Result<QueryStatement>;
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("invalid query: {0}")]
    InvalidQuery(String),
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
}

/// Unified query request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub collection: String,
    pub filter: Option<Filter>,
    pub vector: Option<Vec<f32>>,
    pub limit: usize,
    pub offset: Option<usize>,
}

/// Filter conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    Contains,
    StartsWith,
    EndsWith,
}

/// Unified query response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub offset: usize,
    pub limit: usize,
}

/// Query builder trait
pub trait QueryBuilder {
    fn build(&self) -> Result<QueryStatement>;
}

/// Dataset backend selector for runtime dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatasetBackend {
    Surreal,
    Postgres,
}

impl DatasetBackend {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Surreal => "surreal",
            Self::Postgres => "postgres",
        }
    }
}

impl FromStr for DatasetBackend {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "surreal" => Ok(Self::Surreal),
            "postgres" | "postgresql" | "pg" => Ok(Self::Postgres),
            other => Err(anyhow::anyhow!("unsupported backend: {}", other)),
        }
    }
}

/// Read-side dataset abstraction.
pub trait Dataset {
    fn schema(&self) -> Result<serde_json::Value>;
    fn records(&self, limit: usize) -> Result<Vec<serde_json::Value>>;
}

/// Write-side dataset abstraction.
pub trait Writer {
    fn write(&self, record: serde_json::Value) -> Result<()>;
    fn write_all(&self, records: Vec<serde_json::Value>) -> Result<()> {
        for record in records {
            self.write(record)?;
        }
        Ok(())
    }
}

pub fn load(backend: DatasetBackend, conn_str: &str) -> Result<Box<dyn Dataset>> {
    match backend {
        DatasetBackend::Surreal => Ok(Box::new(SurrealDataset::connect(conn_str)?)),
        DatasetBackend::Postgres => Ok(Box::new(PgDataset::connect(conn_str)?)),
    }
}

pub fn writer(backend: DatasetBackend, conn_str: &str) -> Result<Box<dyn Writer>> {
    match backend {
        DatasetBackend::Surreal => Ok(Box::new(SurrealDataset::connect(conn_str)?)),
        DatasetBackend::Postgres => Ok(Box::new(PgDataset::connect(conn_str)?)),
    }
}

/// Query planner with parameterized query support
pub struct QueryPlanner;

impl QueryPlanner {
    pub fn plan_surreal(req: &QueryRequest) -> QueryStatement {
        let mut query = format!("SELECT * FROM {}", req.collection);
        let mut params = HashMap::new();

        if let Some(ref filter) = req.filter {
            let param_key = "p0".to_string();
            params.insert(param_key.clone(), filter.value.clone());
            query.push_str(&format!(
                " WHERE {} {} $[\"{}\"]",
                filter.field,
                Self::op_to_string(&filter.operator),
                param_key
            ));
        }

        query.push_str(&format!(" LIMIT {}", req.limit));

        if let Some(offset) = req.offset {
            query.push_str(&format!(" START {}", offset));
        }

        QueryStatement { sql: query, params }
    }

    pub fn plan_postgres(req: &QueryRequest) -> QueryStatement {
        let mut query = format!("SELECT * FROM {}", req.collection);
        let mut params = HashMap::new();

        if let Some(ref filter) = req.filter {
            let param_key = "$1".to_string();
            params.insert(param_key.clone(), filter.value.clone());
            query.push_str(&format!(
                " WHERE {} {} {}",
                Self::pg_escape_identifier(&filter.field),
                Self::op_to_string(&filter.operator),
                param_key
            ));
        }

        query.push_str(&format!(" LIMIT {}", req.limit));
        if let Some(offset) = req.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        QueryStatement { sql: query, params }
    }

    fn pg_escape_identifier(ident: &str) -> String {
        if ident.chars().all(|c| c.is_alphanumeric() || c == '_') {
            ident.to_string()
        } else {
            String::new()
        }
    }

    fn op_to_string(op: &FilterOperator) -> &'static str {
        match op {
            FilterOperator::Eq => "=",
            FilterOperator::Ne => "!=",
            FilterOperator::Gt => ">",
            FilterOperator::Gte => ">=",
            FilterOperator::Lt => "<",
            FilterOperator::Lte => "<=",
            FilterOperator::Contains => "LIKE",
            FilterOperator::StartsWith => "LIKE",
            FilterOperator::EndsWith => "LIKE",
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SurrealQueryPlanner;

impl QueryPort for SurrealQueryPlanner {
    fn plan(&self, req: &QueryRequest) -> Result<QueryStatement> {
        Ok(QueryPlanner::plan_surreal(req))
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PostgresQueryPlanner;

impl QueryPort for PostgresQueryPlanner {
    fn plan(&self, req: &QueryRequest) -> Result<QueryStatement> {
        Ok(QueryPlanner::plan_postgres(req))
    }
}

struct SurrealDataset {
    conn_str: String,
    cached_records: Vec<serde_json::Value>,
}

impl SurrealDataset {
    pub fn connect(conn_str: &str) -> Result<Self> {
        if conn_str.trim().is_empty() {
            return Err(anyhow::anyhow!("surreal connection string cannot be empty"));
        }
        Ok(Self {
            conn_str: conn_str.to_string(),
            cached_records: Vec::new(),
        })
    }
}

impl Dataset for SurrealDataset {
    fn schema(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "backend": "surreal",
            "connection": self.conn_str,
            "planner": "surreal",
            "sample_table": "record"
        }))
    }

    fn records(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        Ok(self.cached_records.iter().take(limit).cloned().collect())
    }
}

impl Writer for SurrealDataset {
    fn write(&self, record: serde_json::Value) -> Result<()> {
        let _ = (&self.conn_str, record);
        Ok(())
    }
}

struct PgDataset {
    conn_str: String,
    cached_records: Vec<serde_json::Value>,
}

impl PgDataset {
    pub fn connect(conn_str: &str) -> Result<Self> {
        if conn_str.trim().is_empty() {
            return Err(anyhow::anyhow!("postgres connection string cannot be empty"));
        }
        Ok(Self {
            conn_str: conn_str.to_string(),
            cached_records: Vec::new(),
        })
    }
}

impl Dataset for PgDataset {
    fn schema(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "backend": "postgres",
            "connection": self.conn_str,
            "planner": "postgres",
            "sample_table": "pheno_records"
        }))
    }

    fn records(&self, limit: usize) -> Result<Vec<serde_json::Value>> {
        Ok(self.cached_records.iter().take(limit).cloned().collect())
    }
}

impl Writer for PgDataset {
    fn write(&self, record: serde_json::Value) -> Result<()> {
        let _ = (&self.conn_str, record);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_surreal() {
        let req = QueryRequest {
            collection: "skills".to_string(),
            filter: Some(Filter {
                field: "name".to_string(),
                operator: FilterOperator::Eq,
                value: serde_json::json!("test"),
            }),
            vector: None,
            limit: 10,
            offset: None,
        };

        let stmt = QueryPlanner::plan_surreal(&req);
        assert!(stmt.sql.contains("WHERE name = $[\"p0\"]"));
        assert!(stmt.sql.contains("LIMIT 10"));
        assert_eq!(stmt.params.get("p0"), Some(&serde_json::json!("test")));
    }

    #[test]
    fn test_plan_postgres() {
        let req = QueryRequest {
            collection: "skills".to_string(),
            filter: Some(Filter {
                field: "name".to_string(),
                operator: FilterOperator::Eq,
                value: serde_json::json!("test"),
            }),
            vector: None,
            limit: 10,
            offset: Some(5),
        };

        let stmt = QueryPlanner::plan_postgres(&req);
        assert!(stmt.sql.contains("WHERE name = $1"));
        assert!(stmt.sql.contains("LIMIT 10"));
        assert!(stmt.sql.contains("OFFSET 5"));
        assert_eq!(stmt.params.get("$1"), Some(&serde_json::json!("test")));
    }

    #[test]
    fn test_plan_no_filter() {
        let req = QueryRequest {
            collection: "skills".to_string(),
            filter: None,
            vector: None,
            limit: 10,
            offset: None,
        };

        let stmt = QueryPlanner::plan_surreal(&req);
        assert!(!stmt.sql.contains("WHERE"));
        assert!(stmt.sql.contains("LIMIT 10"));
    }

    #[test]
    fn test_query_port_dispatch_polymorphism() {
        let req = QueryRequest {
            collection: "skills".to_string(),
            filter: Some(Filter {
                field: "name".to_string(),
                operator: FilterOperator::Eq,
                value: serde_json::json!("test"),
            }),
            vector: None,
            limit: 10,
            offset: None,
        };

        let planners: Vec<Box<dyn QueryPort>> = vec![
            Box::new(SurrealQueryPlanner),
            Box::new(PostgresQueryPlanner),
        ];

        let stmts: Vec<QueryStatement> = planners.iter().map(|p| p.plan(&req).unwrap()).collect();
        assert!(stmts[0].sql.contains("WHERE name = $[\"p0\"]"));
        assert!(stmts[1].sql.contains("WHERE name = $1"));
        assert!(stmts[0].params.contains_key("p0"));
        assert!(stmts[1].params.contains_key("$1"));
    }

    #[test]
    fn test_backend_parse_aliases() {
        assert!(matches!(
            DatasetBackend::from_str("surreal").unwrap(),
            DatasetBackend::Surreal
        ));
        assert!(matches!(
            DatasetBackend::from_str("pg").unwrap(),
            DatasetBackend::Postgres
        ));
    }

    #[test]
    fn test_planner_newtypes_are_copy() {
        let s1 = SurrealQueryPlanner;
        let s2 = s1;
        let _ = s1;

        let p1 = PostgresQueryPlanner;
        let p2 = p1;
        let _ = p1;

        let req = QueryRequest {
            collection: "t".to_string(),
            filter: None,
            vector: None,
            limit: 1,
            offset: None,
        };
        assert!(s2.plan(&req).is_ok());
        assert!(p2.plan(&req).is_ok());
    }
}

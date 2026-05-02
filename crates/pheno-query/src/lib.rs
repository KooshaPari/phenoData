//! PhenoQuery - Unified query builder for Pheno
//!
//! Provides a unified query interface across different data stores.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    fn build(&self) -> Result<String>;
}

/// Simple query planner
pub struct QueryPlanner;

impl QueryPlanner {
    /// Plan query for SurrealDB
    pub fn plan_surreal(req: &QueryRequest) -> String {
        let mut query = format!("SELECT * FROM {}", req.collection);
        
        if let Some(ref filter) = req.filter {
            query.push_str(&format!(" WHERE {} {} {}", 
                filter.field,
                Self::op_to_string(&filter.operator),
                filter.value
            ));
        }
        
        if let Some(ref _vec) = req.vector {
            query.push_str(&format!(" FETCH {}", req.collection));
        }
        
        query.push_str(&format!(" LIMIT {}", req.limit));
        
        if let Some(offset) = req.offset {
            query.push_str(&format!(" START {}", offset));
        }
        
        query
    }

    /// Plan query for PostgreSQL
    pub fn plan_postgres(req: &QueryRequest) -> String {
        let mut query = format!("SELECT * FROM {}", req.collection);
        
        if let Some(ref filter) = req.filter {
            query.push_str(&format!(" WHERE {} {} {}", 
                filter.field,
                Self::op_to_string(&filter.operator),
                filter.value
            ));
        }
        
        query.push_str(&format!(" LIMIT {} OFFSET {}", req.limit, req.offset.unwrap_or(0)));
        
        query
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
        
        let query = QueryPlanner::plan_surreal(&req);
        assert!(query.contains("WHERE name = \"test\""));
        assert!(query.contains("LIMIT 10"));
    }
}

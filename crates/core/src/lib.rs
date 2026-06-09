#![allow(async_fn_in_trait)]
//! Core dataset and writer ports for the Pheno data workspace.

pub mod errors;
pub mod types;
pub mod core {
    pub use crate::{Dataset, Schema, Writer};
}

use async_trait::async_trait;
use std::pin::Pin;

pub use crate::errors::{CoreError, Result};
pub use crate::types::{DatasetMetadata, FieldDef, Record, RecordBatch};

pub type BoxStream<'a, T> = Pin<Box<dyn Iterator<Item = T> + Send + 'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub fields: Vec<FieldDef>,
}

impl Schema {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        fields: Vec<FieldDef>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            fields,
        }
    }
}

#[async_trait]
pub trait Dataset: Send + Sync {
    fn records(&self) -> BoxStream<'_, Result<Record>>;
    fn schema(&self) -> Schema;
    async fn close(&self) -> Result<()>;
}

#[async_trait]
pub trait Writer: Send + Sync {
    async fn write(&mut self, record: Record) -> Result<()>;
    async fn flush(&mut self) -> Result<()>;
    async fn close(&mut self) -> Result<()>;
}

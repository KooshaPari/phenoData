use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub type Record = Value;
pub type DatasetSchema = Value;

#[async_trait]
pub trait Dataset: Send + Sync {
    async fn records(&self) -> Result<Vec<Record>>;
    async fn schema(&self) -> Result<DatasetSchema>;
    async fn close(&self) -> Result<()>;
}

#[async_trait]
pub trait Writer: Send + Sync {
    async fn write(&self, record: Record) -> Result<()>;
    async fn flush(&self) -> Result<()>;
    async fn close(&self) -> Result<()>;
}

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use pheno_query::{load, writer, DatasetBackend};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pheno-data", about = "Inspect and write pheno datasets")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Load { backend: BackendArg, conn: String },
    Inspect { backend: BackendArg, conn: String },
    Write {
        backend: BackendArg,
        conn: String,
        jsonl_file: PathBuf,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum BackendArg {
    Surreal,
    Postgres,
}

impl From<BackendArg> for DatasetBackend {
    fn from(value: BackendArg) -> Self {
        match value {
            BackendArg::Surreal => DatasetBackend::Surreal,
            BackendArg::Postgres => DatasetBackend::Postgres,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Load { backend, conn } => {
            let backend = DatasetBackend::from(backend);
            let dataset = load(backend, &conn)?;
            println!("loaded {} dataset", backend.as_str());
            println!("{}", serde_json::to_string_pretty(&dataset.schema()?)?);
        }
        Commands::Inspect { backend, conn } => {
            let backend = DatasetBackend::from(backend);
            let dataset = load(backend, &conn)?;
            println!("schema:");
            println!("{}", serde_json::to_string_pretty(&dataset.schema()?)?);
            println!("records:");
            println!("{}", serde_json::to_string_pretty(&dataset.records(5)?)?);
        }
        Commands::Write {
            backend,
            conn,
            jsonl_file,
        } => {
            let backend = DatasetBackend::from(backend);
            let sink = writer(backend, &conn)?;
            let contents = fs::read_to_string(&jsonl_file)
                .with_context(|| format!("failed to read {}", jsonl_file.display()))?;
            let records = contents
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(serde_json::from_str)
                .collect::<std::result::Result<Vec<serde_json::Value>, _>>()
                .context("failed to parse jsonl input")?;
            let count = records.len();
            sink.write_all(records)?;
            println!("wrote {} records to {}", count, backend.as_str());
        }
    }

    Ok(())
}
